import { createHash } from "node:crypto";
import { db } from "./db";

export type Principal = { accountId: string; tier: string };

export async function authenticate(request: Request): Promise<Principal | null> {
  const header = request.headers.get("authorization");
  if (!header?.startsWith("Bearer ")) return null;
  const token = header.slice(7);
  if (token.length < 32 || token.length > 256) return null;
  const hash = createHash("sha256").update(token).digest("hex");
  const sql = db();
  const [row] = await sql<{ account_id: string; tier: string }[]>`
    UPDATE api_tokens AS token
       SET last_used_at = now()
      FROM accounts AS account
     WHERE token.token_hash = ${hash}
       AND token.account_id = account.id
       AND token.revoked_at IS NULL
       AND (token.expires_at IS NULL OR token.expires_at > now())
    RETURNING token.account_id, account.tier
  `;
  return row ? { accountId: row.account_id, tier: row.tier } : null;
}

export function unauthorized() {
  return Response.json({ error: "A valid Wishfully API token is required" }, { status: 401 });
}
