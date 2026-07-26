---
type: "code-fragment"
fragment_id: "rgbdns-frag-1c12785a949f"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Observe the right signals"
kind: "heading"
start_line: 1195
end_line: 1219
---

# Observe the right signals

- Fragment ID: `rgbdns-frag-1c12785a949f`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1195-1219
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-1c12785a949f", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-1c12785a949f: heading Observe the right signals", "sourcePath": "docs/book/rgbdns.md", "startLine": 1195, "endLine": 1219}
```

## Excerpt

<span id="rgbdns-frag-1c12785a949f" class="rgbdns-fragment-target"></span>
### rgbdns-frag-1c12785a949f: heading Observe the right signals

```markdown
## Observe the right signals

Useful signals include:

- query and error rate by transport;
- truncated UDP responses and TCP retries;
- SERVFAIL, REFUSED, NXDOMAIN, and validation-failure rates;
- resolver cache capacity and latency percentiles;
- process restarts and file-descriptor use;
- root-hint and trust-anchor freshness;
- ANAME refresh latency, upstream failures, cache misses, and synthesized TTLs;
- time synchronization;
- CDB build identity and deployment time.

High NXDOMAIN volume is not automatically an incident; browsers, typo traffic,
and discovery protocols generate it. A change from baseline paired with
latency or SERVFAIL is more meaningful.

TAI64N log labels make events stable for storage. Convert them for human
display at the edge:

```sh
tail -f main/current | tai64nlocal
```

```
