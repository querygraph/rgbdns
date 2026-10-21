---
type: "code-fragment"
fragment_id: "rgbdns-frag-fb9ab974de2c"
source_path: "docs/DNSSEC.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Enable packaged publication"
kind: "heading"
start_line: 80
end_line: 101
---

# Enable packaged publication

- Fragment ID: `rgbdns-frag-fb9ab974de2c`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC.md.source|docs/DNSSEC.md]]
- Lines: 80-101
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-fb9ab974de2c", "codeNote": "DNS from First Principles/Code/docs/DNSSEC.md.source", "sourcePath": "docs/DNSSEC.md", "startLine": 80, "endLine": 101}
```

## Excerpt

<span id="rgbdns-frag-fb9ab974de2c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-fb9ab974de2c: heading Enable packaged publication

````markdown
## Enable packaged publication

Pass the policy while configuring a primary:

```sh
sudo rgbdns-setup primary \
  --data /path/to/rgbdns.data \
  --dnssec-policy /etc/rgbdns/dnssec
```

This creates `/etc/rgbdns/dnssec.env`, performs the first publication as root,
and enables two timers. `rgbdns-dnssec-publish.timer` refreshes materialized
ANAME data and signatures every 12 hours. `rgbdns-dnssec-check.timer` verifies
the active CDB hourly as the unprivileged `rgbdns` account. A failed resolver,
signer, compiler, or checker leaves the prior CDB active.

The root-only publisher reads the private key. The `rgbdns` service account
can read the public policy and active CDB but cannot traverse
`/etc/rgbdns/keys`. Removing `/etc/rgbdns/dnssec.env` returns `compile-zone` to
the original `tinydns-data` path; use `rgbdns-setup` to make role changes so
timer state remains consistent.

````
