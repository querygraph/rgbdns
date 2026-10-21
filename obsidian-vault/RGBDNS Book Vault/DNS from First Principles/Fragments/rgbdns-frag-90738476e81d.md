---
type: "code-fragment"
fragment_id: "rgbdns-frag-90738476e81d"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Private ANAME-over-AXFR test zone; never delegated publicly"
kind: "heading"
start_line: 474
end_line: 530
---

# Private ANAME-over-AXFR test zone; never delegated publicly

- Fragment ID: `rgbdns-frag-90738476e81d`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 474-530
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-90738476e81d", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 474, "endLine": 530}
```

## Excerpt

<span id="rgbdns-frag-90738476e81d" class="rgbdns-fragment-target"></span>
### rgbdns-frag-90738476e81d: heading Private ANAME-over-AXFR test zone; never delegated publicly

````markdown
# Private ANAME-over-AXFR test zone; never delegated publicly.
Zaname-axfr.test:a.ns.cron.sh:hostmaster.cron.sh:2026073101:16384:2048:1048576:2560:3600
&aname-axfr.test::a.ns.cron.sh:3600
&aname-axfr.test::b.ns.cron.sh:3600
Aaname-axfr.test:publication.ghost.io:300
```

On the primary, confirm that the picker compiled and activated it:

```sh
sudo journalctl -u rgbdns-data-import.service -n 50 --no-pager
dig @127.0.0.1 aname-axfr.test SOA +norecurse
dig @127.0.0.1 aname-axfr.test A +norecurse
```

Add `aname-axfr.test` to the workstation's complete `rgbdns.zones`, publish
that file atomically as described in Section 13, and let the secondary picker
start synchronization. On the secondary, require both successful one-shot
status and the reconstructed private directive:

```sh
sudo systemctl show rgbdns-secondary-sync.service \
  -p Result -p ExecMainStatus -p ActiveState -p SubState
sudo grep '^Aaname-axfr\.test' \
  /var/lib/rgbdns/tinydns/secondary-zones/aname-axfr.test.data
```

Expected data includes:

```text
Aaname-axfr.test.:publication.ghost.io.:300
```

That source line is the definitive negotiated-transfer proof. Regular DNS
clients must instead see synthesized standard address records, never ANAME,
its target, `RGA1`, or TYPE65401:

```sh
for server in 52.10.53.234 52.38.177.160; do
  dig @"$server" aname-axfr.test SOA +norecurse
  dig @"$server" aname-axfr.test A +norecurse
  dig @"$server" aname-axfr.test AAAA +norecurse
done
```

Run the standard-AXFR privacy check **on `b.ns` or another authorized VPC
host**, because `172.31.60.189` is private AWS address space:

```sh
dig @172.31.60.189 aname-axfr.test AXFR |
  grep -E 'TYPE65401|RGA1|publication\.ghost\.io'
```

No output is expected. From an ordinary workstation outside AWS, use the two
public addresses only for normal authoritative queries. Do not broaden the
AXFR allow-list merely to run this privacy check.

````
