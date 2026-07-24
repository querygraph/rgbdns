---
type: "code-fragment"
fragment_id: "rgbdns-frag-0d9f2165b008"
source_path: "docs/conformance.md"
code_note: "DNS from First Principles/Code/docs/conformance.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Running the matrix"
kind: "heading"
start_line: 66
end_line: 79
---

# Running the matrix

- Fragment ID: `rgbdns-frag-0d9f2165b008`
- Source file: [[DNS from First Principles/Code/docs/conformance.md.source|docs/conformance.md]]
- Lines: 66-79
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-0d9f2165b008", "codeNote": "DNS from First Principles/Code/docs/conformance.md.source", "heading": "rgbdns-frag-0d9f2165b008: heading Running the matrix", "sourcePath": "docs/conformance.md", "startLine": 66, "endLine": 79}
```

## Excerpt

<span id="rgbdns-frag-0d9f2165b008" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0d9f2165b008: heading Running the matrix

```markdown
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
```
