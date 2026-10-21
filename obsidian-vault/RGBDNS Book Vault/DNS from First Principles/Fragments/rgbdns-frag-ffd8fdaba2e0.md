---
type: "code-fragment"
fragment_id: "rgbdns-frag-ffd8fdaba2e0"
source_path: "AGENTS.md"
code_note: "DNS from First Principles/Code/AGENTS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Required validation"
kind: "heading"
start_line: 142
end_line: 155
---

# Required validation

- Fragment ID: `rgbdns-frag-ffd8fdaba2e0`
- Source file: [[DNS from First Principles/Code/AGENTS.md.source|AGENTS.md]]
- Lines: 142-155
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-ffd8fdaba2e0", "codeNote": "DNS from First Principles/Code/AGENTS.md.source", "sourcePath": "AGENTS.md", "startLine": 142, "endLine": 155}
```

## Excerpt

<span id="rgbdns-frag-ffd8fdaba2e0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ffd8fdaba2e0: heading Required validation

````markdown
## Required validation

For code, protocol, packaging, or release changes, run as applicable:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
packaging/tests/test-secondary-sync.sh
make -C ietf check
git diff --check
```

The direct public-network DNSSEC test remains opt-in.
````
