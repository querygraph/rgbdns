---
type: "code-fragment"
fragment_id: "rgbdns-frag-97e6e776df91"
source_path: "docs/conformance.md"
code_note: "DNS from First Principles/Code/docs/conformance.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Running the matrix"
kind: "heading"
start_line: 76
end_line: 89
---

# Running the matrix

- Fragment ID: `rgbdns-frag-97e6e776df91`
- Source file: [[DNS from First Principles/Code/docs/conformance.md.source|docs/conformance.md]]
- Lines: 76-89
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-97e6e776df91", "codeNote": "DNS from First Principles/Code/docs/conformance.md.source", "sourcePath": "docs/conformance.md", "startLine": 76, "endLine": 89}
```

## Excerpt

<span id="rgbdns-frag-97e6e776df91" class="rgbdns-fragment-target"></span>
### rgbdns-frag-97e6e776df91: heading Running the matrix

````markdown
## Running the matrix

```sh
cargo test --test rfc_conformance
cargo test --test wire_security
cargo test --test packet_properties
cargo test --test drill_interop
```

The public-hierarchy DNSSEC test is intentionally opt-in:

```sh
cargo test --test dnscache_network -- --ignored
```
````
