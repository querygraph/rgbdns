import { createHash } from "node:crypto";

export type ManagedDomain = {
  name: string;
  destinationType: "A" | "ANAME";
  destinationValue: string;
  includeWww: boolean;
  records?: Array<{ owner: string; type: string; value: string; ttl: number; priority?: number | null }>;
};

const BEGIN = "# BEGIN WISHFULLY MANAGED ZONES";
const END = "# END WISHFULLY MANAGED ZONES";

function replaceSection(source: string, body: string) {
  const section = `${BEGIN}\n${body.trim()}\n${END}`;
  const start = source.indexOf(BEGIN);
  const finish = source.indexOf(END);
  if (start < 0 && finish < 0) return `${source.trimEnd()}\n\n${section}\n`;
  if (start < 0 || finish < start) throw new Error("Malformed Wishfully managed section");
  return `${source.slice(0, start)}${section}${source.slice(finish + END.length)}`.replace(/\n{3,}/g, "\n\n").trimEnd() + "\n";
}

function serialFor(domain: ManagedDomain) {
  const day = new Intl.DateTimeFormat("en-CA", { year: "numeric", month: "2-digit", day: "2-digit", timeZone: "UTC" }).format(new Date()).replaceAll("-", "");
  const suffix = parseInt(createHash("sha256").update(JSON.stringify(domain)).digest("hex").slice(0, 2), 16) % 98 + 1;
  return Number(`${day}${String(suffix).padStart(2, "0")}`);
}

function escapeTxt(value: string) { return value.replaceAll("\\", "\\\\").replaceAll(":", "\\072").replaceAll("\n", "\\012"); }

function recordLine(record: NonNullable<ManagedDomain["records"]>[number], zone: string) {
  const owner = record.owner === "@" ? zone : record.owner.endsWith(`.${zone}`) ? record.owner : `${record.owner}.${zone}`;
  switch (record.type) {
    case "A": return `+${owner}:${record.value}:${record.ttl}`;
    case "ANAME": return `A${owner}:${record.value}:${record.ttl}`;
    case "CNAME": return `C${owner}:${record.value}:${record.ttl}`;
    case "TXT": return `'${owner}:${escapeTxt(record.value)}:${record.ttl}`;
    case "MX": return `@${owner}::${record.value}:${record.priority ?? 10}:${record.ttl}`;
    default: throw new Error(`Record type ${record.type} is not yet emitted by the consolidated compiler`);
  }
}

export function compileConsolidated(currentData: string, currentZones: string, domains: ManagedDomain[]) {
  const ordered = [...domains].sort((a, b) => a.name.localeCompare(b.name));
  const blocks = ordered.map((domain) => {
    const lines = [
      `# wishfully account zone: ${domain.name}`,
      `Z${domain.name}:a.ns.cron.sh:hostmaster.${domain.name}:${serialFor(domain)}:16384:2048:1048576:2560:3600`,
      `&${domain.name}::a.ns.cron.sh:3600`,
      `&${domain.name}::b.ns.cron.sh:3600`,
      domain.destinationType === "A" ? `+${domain.name}:${domain.destinationValue}:300` : `A${domain.name}:${domain.destinationValue}:300`,
    ];
    if (domain.includeWww) lines.push(`Cwww.${domain.name}:${domain.name}:300`);
    for (const record of domain.records ?? []) lines.push(recordLine(record, domain.name));
    return lines.join("\n");
  });
  const data = replaceSection(currentData, blocks.join("\n\n"));
  const zones = replaceSection(currentZones, ordered.map((domain) => domain.name).join("\n"));
  return { data, zones, digest: createHash("sha256").update(data).update("\0").update(zones).digest("hex") };
}
