import { z } from "zod";
import { authenticate, unauthorized } from "@/lib/auth";
import { db } from "@/lib/db";

const record = z.object({ owner: z.string().trim().toLowerCase().regex(/^(?:@|[a-z0-9_*.-]+)$/), type: z.enum(["A","ANAME","CNAME","MX","TXT"]), value: z.string().min(1).max(255), ttl: z.number().int().min(30).max(86400).default(300), priority: z.number().int().min(0).max(65535).nullable().optional() });
const bodySchema = z.object({ records: z.array(record).max(500) });

export async function GET(request: Request, context: { params: Promise<{ name: string }> }) {
  const principal = await authenticate(request); if (!principal) return unauthorized(); const { name } = await context.params; const sql = db();
  const rows = await sql`SELECT record.owner, record.type, record.value, record.ttl, record.priority FROM records AS record JOIN domains AS domain ON domain.id = record.domain_id WHERE domain.account_id = ${principal.accountId} AND domain.name = ${name.toLowerCase()} ORDER BY record.owner, record.type, record.value`;
  return Response.json({ records: rows });
}

export async function PUT(request: Request, context: { params: Promise<{ name: string }> }) {
  const principal = await authenticate(request); if (!principal) return unauthorized(); const { name } = await context.params;
  const parsed = bodySchema.safeParse(await request.json().catch(() => null)); if (!parsed.success) return Response.json({ error: "Invalid record set", details: parsed.error.flatten() }, { status: 422 });
  for (const item of parsed.data.records) if (item.owner === "@" && ["A","ANAME","CNAME"].includes(item.type)) return Response.json({ error: "Change the domain destination instead of adding a second apex address or alias" }, { status: 422 });
  const sql = db();
  const [domain] = await sql<{ id: string }[]>`SELECT id FROM domains WHERE account_id = ${principal.accountId} AND name = ${name.toLowerCase()}`; if (!domain) return Response.json({ error: "Domain not found" }, { status: 404 });
  await sql.begin(async (transaction) => {
    await transaction.unsafe("DELETE FROM records WHERE domain_id = $1", [domain.id]);
    for (const item of parsed.data.records) await transaction.unsafe("INSERT INTO records (domain_id, owner, type, value, ttl, priority) VALUES ($1, $2, $3, $4, $5, $6)", [domain.id, item.owner, item.type, item.value, item.ttl, item.priority ?? null]);
    await transaction.unsafe("UPDATE domains SET updated_at = now() WHERE id = $1", [domain.id]);
  });
  return Response.json({ updated: true, count: parsed.data.records.length });
}
