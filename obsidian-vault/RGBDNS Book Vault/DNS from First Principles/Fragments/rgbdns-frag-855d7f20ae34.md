---
type: "code-fragment"
fragment_id: "rgbdns-frag-855d7f20ae34"
source_path: "README.md"
code_note: "DNS from First Principles/Code/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Conformance and performance"
kind: "heading"
start_line: 64
end_line: 75
---

# Conformance and performance

- Fragment ID: `rgbdns-frag-855d7f20ae34`
- Source file: [[DNS from First Principles/Code/README.md.source|README.md]]
- Lines: 64-75
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-855d7f20ae34", "codeNote": "DNS from First Principles/Code/README.md.source", "heading": "rgbdns-frag-855d7f20ae34: heading Conformance and performance", "sourcePath": "README.md", "startLine": 64, "endLine": 75}
```

## Excerpt

<span id="rgbdns-frag-855d7f20ae34" class="rgbdns-fragment-target"></span>
### rgbdns-frag-855d7f20ae34: heading Conformance and performance

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
