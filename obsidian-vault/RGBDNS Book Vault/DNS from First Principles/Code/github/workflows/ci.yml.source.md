---
type: "code-file"
source_path: ".github/workflows/ci.yml"
language: "yaml"
subsystem: "Project automation"
line_count: 32
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# .github/workflows/ci.yml

- Subsystem: [[DNS from First Principles/Subsystems/Project automation|Project automation]]
- Source path: `.github/workflows/ci.yml`
- Lines: 32
- Summary: Source file in the Project automation subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-4f57935e9e18|ci.yml]]: lines 1-32

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
      - run: cargo build --bin rgbdns-log-report
      - run: packaging/tests/test-daily-query-report.sh target/debug/rgbdns-log-report
      - run: packaging/tests/test-secondary-sync.sh
      - run: packaging/tests/test-restore-role-units.sh
      - run: packaging/tests/test-publish-dnssec.sh
      - run: cargo build --release --bins
```
