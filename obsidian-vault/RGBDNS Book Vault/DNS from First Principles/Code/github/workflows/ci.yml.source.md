---
type: "code-file"
source_path: ".github/workflows/ci.yml"
language: "yaml"
subsystem: "Project automation"
line_count: 27
fragment_count: 1
rgbdns_commit: "472c2087"
---

# .github/workflows/ci.yml

- Subsystem: [[DNS from First Principles/Subsystems/Project automation|Project automation]]
- Source path: `.github/workflows/ci.yml`
- Lines: 27
- Summary: Source file in the Project automation subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-2c928fa0e845|ci.yml]]: lines 1-27

## Full Source

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
