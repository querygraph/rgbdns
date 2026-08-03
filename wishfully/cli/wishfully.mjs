#!/usr/bin/env node

import { chmod, mkdtemp, readFile, rm, writeFile } from "node:fs/promises";
import { spawn } from "node:child_process";
import { tmpdir } from "node:os";
import { join } from "node:path";

const args = process.argv.slice(2);
const api = (process.env.WISHFULLY_API_URL || "https://wishful.ly").replace(/\/$/, "");

function usage() {
  console.error(`usage:
  wishfully domains plan DOMAIN {--aname NAME | --address IPV4}
  wishfully domains add DOMAIN {--aname NAME | --address IPV4} [--no-www]
  wishfully domains verify DOMAIN
  wishfully domains list
  wishfully records list DOMAIN
  wishfully records apply DOMAIN --file records.json
  wishfully deployments create
  wishfully deployments list
  wishfully certbot credentials DOMAIN --output FILE
  wishfully certbot rotate DOMAIN --output FILE
  wishfully certbot revoke DOMAIN
  wishfully certbot run DOMAIN [-d NAME ...] [--certbot PATH]`);
  process.exit(2);
}

function option(name) { const index = args.indexOf(name); return index >= 0 ? args[index + 1] : undefined; }
function options(name) { return args.flatMap((value, index) => value === name && args[index + 1] ? [args[index + 1]] : []); }
function destination() {
  const aname = option("--aname"); const address = option("--address");
  if (Boolean(aname) === Boolean(address)) usage();
  return aname ? { type: "ANAME", value: aname } : { type: "A", value: address };
}

async function request(path, init = {}) {
  const token = process.env.WISHFULLY_TOKEN;
  const response = await fetch(`${api}${path}`, { ...init, headers: { "content-type": "application/json", ...(token ? { authorization: `Bearer ${token}` } : {}), ...init.headers } });
  const body = await response.json().catch(() => ({}));
  if (!response.ok) throw new Error(body.error || `${response.status} ${response.statusText}`);
  return body;
}

function table(rows, columns) {
  if (!rows.length) return console.log("No results.");
  const widths = columns.map(([key, label]) => Math.max(label.length, ...rows.map((row) => String(row[key] ?? "").length)));
  console.log(columns.map(([, label], index) => label.padEnd(widths[index])).join("  "));
  for (const row of rows) console.log(columns.map(([key], index) => String(row[key] ?? "").padEnd(widths[index])).join("  "));
}

async function run() {
  const [group, command, domain] = args;
  if (group === "domains" && command === "plan" && domain) {
    console.log(JSON.stringify(await request("/api/v1/plan", { method: "POST", body: JSON.stringify({ domain, destination: destination(), includeWww: !args.includes("--no-www") }) }), null, 2));
  } else if (group === "domains" && command === "add" && domain) {
    const result = await request("/api/v1/domains", { method: "POST", body: JSON.stringify({ domain, destination: destination(), includeWww: !args.includes("--no-www") }) });
    console.log(`Add this TXT record at the current DNS provider:\n${result.verification.owner}  ${result.verification.value}`);
  } else if (group === "domains" && command === "verify" && domain) {
    await request(`/api/v1/domains/${encodeURIComponent(domain)}/verify`, { method: "POST" });
    console.log(`✓ ${domain} ownership verified`);
  } else if (group === "domains" && command === "list") {
    const result = await request("/api/v1/domains");
    table(result.domains, [["name", "DOMAIN"], ["status", "STATUS"], ["destination_type", "TYPE"], ["destination_value", "DESTINATION"]]);
  } else if (group === "records" && command === "list" && domain) {
    const result = await request(`/api/v1/domains/${encodeURIComponent(domain)}/records`);
    table(result.records, [["owner", "OWNER"], ["type", "TYPE"], ["value", "VALUE"], ["ttl", "TTL"]]);
  } else if (group === "records" && command === "apply" && domain) {
    const file = option("--file"); if (!file) usage(); const body = JSON.parse(await readFile(file, "utf8"));
    const result = await request(`/api/v1/domains/${encodeURIComponent(domain)}/records`, { method: "PUT", body: JSON.stringify(Array.isArray(body) ? { records: body } : body) });
    console.log(`✓ ${result.count} record${result.count === 1 ? "" : "s"} staged for ${domain}`);
  } else if (group === "deployments" && command === "create") {
    const result = await request("/api/v1/deployments", { method: "POST" });
    console.log(`✓ deployment pull request: ${result.pullRequestUrl}`);
  } else if (group === "deployments" && command === "list") {
    const result = await request("/api/v1/deployments");
    table(result.deployments, [["id", "ID"], ["status", "STATUS"], ["pull_request_url", "PULL REQUEST"]]);
  } else if (group === "certbot" && command === "credentials" && domain) {
    const output = option("--output"); if (!output) usage();
    const result = await request(`/api/v1/domains/${encodeURIComponent(domain)}/acme-credentials`, { method: "POST" });
    await writeFile(output, result.ini, { mode: 0o600, flag: "wx" }); await chmod(output, 0o600);
    console.log(`✓ one-time credentials written to ${output}`);
  } else if (group === "certbot" && command === "rotate" && domain) {
    const output = option("--output"); if (!output) usage();
    const result = await request(`/api/v1/domains/${encodeURIComponent(domain)}/acme-credentials?rotate=true`, { method: "POST" });
    await writeFile(output, result.ini, { mode: 0o600, flag: "wx" }); await chmod(output, 0o600);
    console.log(`✓ rotated credentials written once to ${output}; replace the old file after DNS policy deployment completes`);
  } else if (group === "certbot" && command === "revoke" && domain) {
    await request(`/api/v1/domains/${encodeURIComponent(domain)}/acme-credentials`, { method: "DELETE" });
    console.log(`✓ ${domain} Certbot credential revoked and DNS policy deployment dispatched`);
  } else if (group === "certbot" && command === "run" && domain) {
    const result = await request(`/api/v1/domains/${encodeURIComponent(domain)}/acme-credentials`, { method: "POST" });
    const directory = await mkdtemp(join(tmpdir(), "wishfully-certbot-")); const credentials = join(directory, "rfc2136.ini");
    try {
      await writeFile(credentials, result.ini, { mode: 0o600 });
      const names = options("-d"); if (!names.length) names.push(domain, `*.${domain}`);
      const certbot = option("--certbot") || "certbot";
      const child = spawn(certbot, ["certonly", "--dns-rfc2136", "--dns-rfc2136-credentials", credentials, "--dns-rfc2136-propagation-seconds", "90", ...names.flatMap((name) => ["-d", name])], { stdio: "inherit" });
      const status = await new Promise((resolve, reject) => { child.once("error", reject); child.once("exit", (code) => resolve(code ?? 1)); });
      if (status !== 0) throw new Error(`certbot exited with status ${status}`);
    } finally { await rm(directory, { recursive: true, force: true }); }
  } else usage();
}

run().catch((error) => { console.error(`wishfully: ${error.message}`); process.exit(1); });
