import { z } from "zod";

export const domainRequest = z.object({
  domain: z
    .string()
    .trim()
    .toLowerCase()
    .regex(/^(?=.{1,253}$)[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)+$/),
  destination: z.discriminatedUnion("type", [
    z.object({ type: z.literal("A"), value: z.ipv4() }),
    z.object({
      type: z.literal("ANAME"),
      value: z.string().trim().toLowerCase().regex(/^[a-z0-9.-]+$/),
    }),
  ]),
  includeWww: z.boolean().default(true),
});

export type DomainRequest = z.infer<typeof domainRequest>;

export function planDomain(input: DomainRequest) {
  const serial = Number(
    new Intl.DateTimeFormat("en-CA", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      timeZone: "UTC",
    })
      .format(new Date())
      .replaceAll("-", "") + "01",
  );
  const record = input.destination.type === "A"
    ? `+${input.domain}:${input.destination.value}:300`
    : `A${input.domain}:${input.destination.value}:300`;
  const records = [
    `Z${input.domain}:a.ns.cron.sh:hostmaster.${input.domain}:${serial}:16384:2048:1048576:2560:3600`,
    `&${input.domain}::a.ns.cron.sh:3600`,
    `&${input.domain}::b.ns.cron.sh:3600`,
    record,
  ];
  if (input.includeWww) records.push(`Cwww.${input.domain}:${input.domain}:300`);

  return {
    domain: input.domain,
    nameservers: ["a.ns.cron.sh", "b.ns.cron.sh"],
    serial,
    records,
    zonesEntry: input.domain,
    verification: {
      owner: `_wishfully.${input.domain}`,
      type: "TXT",
      value: `wishfully-verification=${crypto.randomUUID()}`,
    },
    certificate: {
      challenge: "DNS-01",
      owner: `_acme-challenge.${input.domain}`,
      algorithm: "HMAC-SHA256",
      ttl: 60,
    },
  };
}
