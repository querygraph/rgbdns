import { createHash, randomBytes } from "node:crypto";
import postgres from "postgres";

if (!process.env.DATABASE_URL) throw new Error("DATABASE_URL is required");
const name = process.env.WISHFULLY_BOOTSTRAP_ACCOUNT || "Wishfully owner";
const tier = process.env.WISHFULLY_BOOTSTRAP_TIER || "studio";
if (!["seed","maker","studio","infrastructure"].includes(tier)) throw new Error("invalid bootstrap tier");
const token = `wf_${randomBytes(32).toString("base64url")}`;
const hash = createHash("sha256").update(token).digest("hex");
const sql = postgres(process.env.DATABASE_URL, { max: 1 });
try {
  const [account] = await sql`INSERT INTO accounts (name, tier) VALUES (${name}, ${tier}) RETURNING id`;
  await sql`INSERT INTO api_tokens (account_id, label, token_hash) VALUES (${account.id}, 'bootstrap', ${hash})`;
  console.log(`Account: ${account.id}\nWISHFULLY_TOKEN=${token}\n\nThis token will not be shown again.`);
} finally { await sql.end(); }
