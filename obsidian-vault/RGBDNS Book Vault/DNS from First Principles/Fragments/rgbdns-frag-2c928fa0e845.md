---
type: "code-fragment"
fragment_id: "rgbdns-frag-2c928fa0e845"
source_path: ".github/workflows/ci.yml"
code_note: "DNS from First Principles/Code/github/workflows/ci.yml.source"
language: "yaml"
subsystem: "Project automation"
symbol: "ci.yml"
kind: "file"
start_line: 1
end_line: 27
---

# ci.yml

- Fragment ID: `rgbdns-frag-2c928fa0e845`
- Source file: [[DNS from First Principles/Code/github/workflows/ci.yml.source|.github/workflows/ci.yml]]
- Lines: 1-27
- Subsystem: [[DNS from First Principles/Subsystems/Project automation|Project automation]]

```rgbdns-fragment
{"id": "rgbdns-frag-2c928fa0e845", "codeNote": "DNS from First Principles/Code/github/workflows/ci.yml.source", "heading": "rgbdns-frag-2c928fa0e845: file ci.yml", "sourcePath": ".github/workflows/ci.yml", "startLine": 1, "endLine": 27}
```

## Excerpt

<span id="rgbdns-frag-2c928fa0e845" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2c928fa0e845: file ci.yml

```yaml
name: CI

on:
  push:
  pull_request:

permissions:
  contents: read

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: -Dwarnings

jobs:
  test:
    runs-on: ubuntu-latest
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@v5
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --check
      - run: cargo clippy --all-targets -- -D warnings
      - run: cargo test --all-targets
      - run: cargo build --release --bins
```
