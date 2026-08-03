import { authenticate, unauthorized } from "@/lib/auth";
import { createHash } from "node:crypto";
import { compileConsolidated, type ManagedDomain } from "@/lib/compiler";
import { db } from "@/lib/db";
import { openDeploymentPullRequest, readDeploymentFiles } from "@/lib/github";

export const runtime = "nodejs";
export const maxDuration = 60;

export async function GET(request: Request) {
  const principal = await authenticate(request);
  if (!principal) return unauthorized();
  const sql = db();
  const rows = await sql`SELECT id, status, branch, pull_request_url, source_digest, error, created_at, updated_at FROM deployments WHERE account_id = ${principal.accountId} ORDER BY created_at DESC LIMIT 50`;
  return Response.json({ deployments: rows });
}

export async function POST(request: Request) {
  const principal = await authenticate(request);
  if (!principal) return unauthorized();
  const sql = db();
  const [deployment] = await sql<{ id: string }[]>`INSERT INTO deployments (account_id) VALUES (${principal.accountId}) RETURNING id`;
  try {
    type DomainRow = { id: string; name: string; destination_type: "A" | "ANAME"; destination_value: string; include_www: boolean; zone_serial: number; record_digest: string | null };
    type RecordRow = { domain_id: string; owner: string; type: string; value: string; ttl: number; priority: number | null };
    const domains = await sql.begin(async (transaction) => {
      await transaction.unsafe("SELECT pg_advisory_xact_lock(hashtext('wishfully-deployment'))");
      const rows = await transaction.unsafe("SELECT id, name, destination_type, destination_value, include_www, zone_serial, record_digest FROM domains WHERE status IN ('verified','pending_delegation','active') ORDER BY name") as unknown as DomainRow[];
      const recordRows = await transaction.unsafe("SELECT record.domain_id, record.owner, record.type, record.value, record.ttl, record.priority FROM records AS record JOIN domains AS domain ON domain.id = record.domain_id WHERE domain.status IN ('verified','pending_delegation','active') ORDER BY record.owner, record.type, record.value") as unknown as RecordRow[];
      const records = new Map<string, RecordRow[]>(); for (const item of recordRows) records.set(item.domain_id, [...(records.get(item.domain_id) ?? []), item]);
      const result: ManagedDomain[] = [];
      for (const row of rows) {
        const items = records.get(row.id) ?? []; const digest = createHash("sha256").update(JSON.stringify([row.destination_type, row.destination_value, row.include_www, items])).digest("hex");
        let serial = Number(row.zone_serial);
        if (digest !== row.record_digest) {
          const updated = await transaction.unsafe("UPDATE domains SET zone_serial = greatest(zone_serial + 1, (to_char(now() AT TIME ZONE 'UTC', 'YYYYMMDD')::bigint * 100) + 1), record_digest = $2, updated_at = now() WHERE id = $1 RETURNING zone_serial", [row.id, digest]) as unknown as Array<{ zone_serial: number }>;
          serial = Number(updated[0].zone_serial);
        }
        result.push({ name: row.name, destinationType: row.destination_type, destinationValue: row.destination_value, includeWww: row.include_www, serial, records: items });
      }
      return result;
    });
    const files = await readDeploymentFiles();
    const compiled = compileConsolidated(files.data.text, files.zones.text, domains);
    const branch = `wishfully/deploy-${deployment.id}`;
    const pull = await openDeploymentPullRequest({ ...files, data: { ...files.data, text: compiled.data }, zones: { ...files.zones, text: compiled.zones }, branch, title: `Deploy ${domains.length} Wishfully zone${domains.length === 1 ? "" : "s"}` });
    await sql`UPDATE deployments SET status = 'pull_request', branch = ${branch}, pull_request_url = ${pull.html_url}, source_digest = ${compiled.digest}, updated_at = now() WHERE id = ${deployment.id}`;
    return Response.json({ id: deployment.id, status: "pull_request", branch, pullRequestUrl: pull.html_url, digest: compiled.digest }, { status: 201 });
  } catch (error) {
    const message = error instanceof Error ? error.message.slice(0, 2000) : "Unknown deployment failure";
    await sql`UPDATE deployments SET status = 'failed', error = ${message}, updated_at = now() WHERE id = ${deployment.id}`;
    return Response.json({ error: "Deployment planning failed", deploymentId: deployment.id }, { status: 502 });
  }
}
