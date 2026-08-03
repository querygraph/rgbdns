#!/usr/bin/env node

const args = process.argv.slice(2);
const api = process.env.WISHFULLY_API_URL || "https://wishful.ly";

function usage() {
  console.error(`usage:
  wishfully domains plan DOMAIN {--aname NAME | --address IPV4}
  wishfully domains add DOMAIN {--aname NAME | --address IPV4}
  wishfully domains verify DOMAIN
  wishfully domains list
  wishfully certbot credentials DOMAIN --output FILE
  wishfully certbot run DOMAIN [--certbot PATH]`);
  process.exit(2);
}

function option(name) {
  const index = args.indexOf(name);
  return index >= 0 ? args[index + 1] : undefined;
}

async function request(path, init = {}) {
  const token = process.env.WISHFULLY_TOKEN;
  const response = await fetch(`${api}${path}`, { ...init, headers: { "content-type": "application/json", ...(token ? { authorization: `Bearer ${token}` } : {}), ...init.headers } });
  const body = await response.json().catch(() => ({}));
  if (!response.ok) throw new Error(body.error || `${response.status} ${response.statusText}`);
  return body;
}

const [group, command, domain] = args;
if (group === "domains" && command === "plan" && domain) {
  const aname = option("--aname"); const address = option("--address");
  if (Boolean(aname) === Boolean(address)) usage();
  const plan = await request("/api/v1/plan", { method: "POST", body: JSON.stringify({ domain, destination: aname ? { type: "ANAME", value: aname } : { type: "A", value: address }, includeWww: true }) });
  console.log(JSON.stringify(plan, null, 2));
} else if (group === "domains" && ["add", "verify", "list"].includes(command)) {
  console.error("This command requires the authenticated control-plane API; see WISHFULLY.md."); process.exit(1);
} else if (group === "certbot") {
  console.error("Certificate commands require a provisioned, one-time TSIG credential; see WISHFULLY.md."); process.exit(1);
} else usage();
