import { randomBytes } from "node:crypto";
import { encryptSecret } from "@/lib/acme-secret";
import { authenticate, unauthorized } from "@/lib/auth";
import { db } from "@/lib/db";
import { dispatchDnsDeployment } from "@/lib/github";

export const runtime = "nodejs";

export async function POST(request: Request, context: { params: Promise<{ name: string }> }) {
  const principal = await authenticate(request); if (!principal) return unauthorized();
  if (!['maker','studio','infrastructure'].includes(principal.tier)) return Response.json({ error: "Scoped Certbot credentials require Maker or higher" }, { status: 403 });
  const { name } = await context.params; const sql = db();
  const [domain] = await sql<{ id: string; name: string; status: string }[]>`SELECT id, name, status FROM domains WHERE account_id = ${principal.accountId} AND name = ${name.toLowerCase()}`;
  if (!domain) return Response.json({ error: "Domain not found" }, { status: 404 });
  if (!['verified','pending_delegation','active'].includes(domain.status)) return Response.json({ error: "Verify the domain before provisioning ACME" }, { status: 409 });
  const rotate = new URL(request.url).searchParams.get("rotate") === "true";
  const existing = await sql`SELECT id FROM acme_credentials WHERE domain_id = ${domain.id} AND status = 'active'`;
  if (existing.length && !rotate) return Response.json({ error: "An active credential already exists; rotate it explicitly instead of minting duplicates" }, { status: 409 });
  const secret = randomBytes(32).toString("base64"); const encrypted = encryptSecret(secret); const keyName = `wishfully-${domain.id}-${randomBytes(6).toString("hex")}.`;
  await sql.begin(async (transaction) => {
    if (rotate) await transaction.unsafe("UPDATE acme_credentials SET status = 'revoked', revoked_at = now() WHERE domain_id = $1 AND status = 'active'", [domain.id]);
    await transaction.unsafe("INSERT INTO acme_credentials (domain_id, key_name, secret_ciphertext, secret_iv) VALUES ($1, $2, $3, $4)", [domain.id, keyName, encrypted.ciphertext, encrypted.iv]);
  });
  await dispatchDnsDeployment();
  const server = process.env.RGBDNS_PRIMARY_PUBLIC_IP || "52.10.53.234";
  const ini = `dns_rfc2136_server = ${server}\ndns_rfc2136_port = 53\ndns_rfc2136_name = ${keyName}\ndns_rfc2136_secret = ${secret}\ndns_rfc2136_algorithm = HMAC-SHA256\n`;
  return Response.json({ keyName, ini, policyDeployment: "dispatched" }, { status: 201, headers: { "cache-control": "no-store" } });
}

export async function DELETE(request: Request, context: { params: Promise<{ name: string }> }) {
  const principal = await authenticate(request); if (!principal) return unauthorized();
  const { name } = await context.params; const sql = db();
  const revoked = await sql`UPDATE acme_credentials AS credential SET status = 'revoked', revoked_at = now() FROM domains AS domain WHERE credential.domain_id = domain.id AND domain.account_id = ${principal.accountId} AND domain.name = ${name.toLowerCase()} AND credential.status = 'active' RETURNING credential.id`;
  if (!revoked.length) return Response.json({ error: "No active credential found" }, { status: 404 });
  await dispatchDnsDeployment();
  return Response.json({ revoked: revoked.length, policyDeployment: "dispatched" });
}
