---
type: "code-fragment"
fragment_id: "rgbdns-frag-4f57935e9e18"
source_path: ".github/workflows/ci.yml"
code_note: "DNS from First Principles/Code/github/workflows/ci.yml.source"
language: "yaml"
subsystem: "Project automation"
symbol: "ci.yml"
kind: "file"
start_line: 1
end_line: 32
---

# ci.yml

- Fragment ID: `rgbdns-frag-4f57935e9e18`
- Source file: [[DNS from First Principles/Code/github/workflows/ci.yml.source|.github/workflows/ci.yml]]
- Lines: 1-32
- Subsystem: [[DNS from First Principles/Subsystems/Project automation|Project automation]]

```rgbdns-fragment
{"id": "rgbdns-frag-4f57935e9e18", "codeNote": "DNS from First Principles/Code/github/workflows/ci.yml.source", "sourcePath": ".github/workflows/ci.yml", "startLine": 1, "endLine": 32}
```

## Excerpt

<span id="rgbdns-frag-4f57935e9e18" class="rgbdns-fragment-target"></span>
### rgbdns-frag-4f57935e9e18: file ci.yml

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
