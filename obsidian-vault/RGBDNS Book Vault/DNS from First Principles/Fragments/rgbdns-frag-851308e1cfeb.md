---
type: "code-fragment"
fragment_id: "rgbdns-frag-851308e1cfeb"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Conformance and performance"
kind: "heading"
start_line: 95
end_line: 106
---

# Conformance and performance

- Fragment ID: `rgbdns-frag-851308e1cfeb`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 95-106
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-851308e1cfeb", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-851308e1cfeb: heading Conformance and performance", "sourcePath": "README.md", "startLine": 95, "endLine": 106}
```

## Excerpt

<span id="rgbdns-frag-851308e1cfeb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-851308e1cfeb: heading Conformance and performance

```markdown
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
```
