import { readFile } from "node:fs/promises";
import postgres from "postgres";

if (!process.env.DATABASE_URL) throw new Error("DATABASE_URL is required");
const sql = postgres(process.env.DATABASE_URL, { max: 1 });
try {
  await sql.unsafe(await readFile(new URL("../db/001_initial.sql", import.meta.url), "utf8"));
  console.log("Wishfully database migration complete.");
} finally { await sql.end(); }
