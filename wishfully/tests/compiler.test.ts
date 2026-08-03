import assert from "node:assert/strict";
import test from "node:test";
import { compileConsolidated } from "../lib/compiler";

const domain = { name: "example.com", destinationType: "ANAME" as const, destinationValue: "site.example.net", includeWww: true, serial: 2026080201, records: [{ owner: "@", type: "TXT", value: "hello:world", ttl: 300 }] };

test("appends a complete managed section without altering legacy zones", () => {
  const result = compileConsolidated("# legacy\nZlegacy.test:ns:host:1:2:3:4:5:6\n", "legacy.test\n", [domain]);
  assert.match(result.data, /^# legacy\nZlegacy\.test/m);
  assert.match(result.data, /Zexample\.com:a\.ns\.cron\.sh:hostmaster\.example\.com:2026080201/);
  assert.match(result.data, /Aexample\.com:site\.example\.net:300/);
  assert.match(result.data, /'example\.com:hello\\072world:300/);
  assert.match(result.zones, /^legacy\.test/m);
  assert.match(result.zones, /^example\.com$/m);
});

test("replaces rather than duplicates the managed section", () => {
  const first = compileConsolidated("# legacy\n", "legacy.test\n", [domain]);
  const second = compileConsolidated(first.data, first.zones, [domain]);
  assert.equal(second.data, first.data);
  assert.equal(second.zones, first.zones);
  assert.equal(second.digest, first.digest);
  assert.equal(second.data.match(/BEGIN WISHFULLY/g)?.length, 1);
});

test("sorts zones for deterministic output", () => {
  const other = { ...domain, name: "alpha.example", destinationType: "A" as const, destinationValue: "192.0.2.1" };
  const result = compileConsolidated("", "", [domain, other]);
  assert.ok(result.data.indexOf("Zalpha.example") < result.data.indexOf("Zexample.com"));
});

test("rejects malformed managed boundaries", () => {
  assert.throws(() => compileConsolidated("# END WISHFULLY MANAGED ZONES\n", "", [domain]), /Malformed/);
});
