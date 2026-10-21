---
type: "code-fragment"
fragment_id: "rgbdns-frag-d1fb368f56c0"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Conformance and performance"
kind: "heading"
start_line: 208
end_line: 219
---

# Conformance and performance

- Fragment ID: `rgbdns-frag-d1fb368f56c0`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 208-219
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-d1fb368f56c0", "codeNote": "DNS from First Principles/Code/README.md.source", "sourcePath": "README.md", "startLine": 208, "endLine": 219}
```

## Excerpt

<span id="rgbdns-frag-d1fb368f56c0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d1fb368f56c0: heading Conformance and performance

````markdown
## Conformance and performance

[`docs/conformance.md`](docs/conformance.md) maps implemented DNS requirements
to RFC-numbered, adversarial, property, live-network, and independent ldns
tests. [`docs/performance.md`](docs/performance.md) documents the stable-Rust
core benchmark:

```sh
cargo test --test rfc_conformance
cargo test --test wire_security
cargo bench --bench dns_core
```
````
