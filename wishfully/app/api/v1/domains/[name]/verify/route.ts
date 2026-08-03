import { resolveTxt } from "node:dns/promises";
import { authenticate, unauthorized } from "@/lib/auth";
import { db } from "@/lib/db";

export const runtime = "nodejs";

export async function POST(request: Request, context: { params: Promise<{ name: string }> }) {
  const principal = await authenticate(request);
  if (!principal) return unauthorized();
  const { name } = await context.params;
  const sql = db();
  const [domain] = await sql<{ id: string; name: string; verification_token: string; status: string }[]>`
    SELECT id, name, verification_token, status FROM domains
     WHERE account_id = ${principal.accountId} AND name = ${name.toLowerCase()}
  `;
  if (!domain) return Response.json({ error: "Domain not found" }, { status: 404 });
  let observed: string[] = [];
  try { observed = (await resolveTxt(`_wishfully.${domain.name}`)).map((parts) => parts.join("")); } catch { /* DNS absence is a normal pending state. */ }
  if (!observed.includes(domain.verification_token)) return Response.json({ verified: false, error: "The ownership TXT value is not visible yet" }, { status: 409 });
  await sql`UPDATE domains SET status = 'verified', verified_at = now(), updated_at = now() WHERE id = ${domain.id}`;
  return Response.json({ verified: true, domain: domain.name });
}
