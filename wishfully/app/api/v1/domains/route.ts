import { randomBytes } from "node:crypto";
import { authenticate, unauthorized } from "@/lib/auth";
import { db } from "@/lib/db";
import { domainRequest } from "@/lib/domain";

export const runtime = "nodejs";

export async function GET(request: Request) {
  const principal = await authenticate(request);
  if (!principal) return unauthorized();
  const sql = db();
  const rows = await sql`
    SELECT name, status, destination_type, destination_value, include_www,
           verified_at, created_at, updated_at
      FROM domains WHERE account_id = ${principal.accountId}
     ORDER BY name
  `;
  return Response.json({ domains: rows });
}

export async function POST(request: Request) {
  const principal = await authenticate(request);
  if (!principal) return unauthorized();
  const parsed = domainRequest.safeParse(await request.json().catch(() => null));
  if (!parsed.success) return Response.json({ error: "Invalid domain", details: parsed.error.flatten() }, { status: 422 });
  const limits: Record<string, number> = { seed: 1, maker: 10, studio: 100, infrastructure: 10000 };
  const sql = db();
  const [{ count }] = await sql<{ count: number }[]>`SELECT count(*)::int AS count FROM domains WHERE account_id = ${principal.accountId} AND status != 'suspended'`;
  if (count >= (limits[principal.tier] ?? 0)) return Response.json({ error: `The ${principal.tier} zone limit has been reached` }, { status: 409 });
  const token = `wishfully-verification=${randomBytes(24).toString("base64url")}`;
  const [row] = await sql`
    INSERT INTO domains (account_id, name, verification_token, destination_type, destination_value, include_www)
    VALUES (${principal.accountId}, ${parsed.data.domain}, ${token}, ${parsed.data.destination.type}, ${parsed.data.destination.value}, ${parsed.data.includeWww})
    RETURNING id, name, status, created_at
  `;
  return Response.json({ domain: row, verification: { owner: `_wishfully.${parsed.data.domain}`, type: "TXT", value: token } }, { status: 201 });
}
