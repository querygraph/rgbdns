---
type: "code-fragment"
fragment_id: "rgbdns-frag-91f686dca9db"
source_path: ".github/workflows/repackage.yml"
code_note: "DNS from First Principles/Code/github/workflows/repackage.yml.source"
language: "yaml"
subsystem: "Project automation"
symbol: "repackage.yml"
kind: "file"
start_line: 1
end_line: 81
---

# repackage.yml

- Fragment ID: `rgbdns-frag-91f686dca9db`
- Source file: [[DNS from First Principles/Code/github/workflows/repackage.yml.source|.github/workflows/repackage.yml]]
- Lines: 1-81
- Subsystem: [[DNS from First Principles/Subsystems/Project automation|Project automation]]

```rgbdns-fragment
{"id": "rgbdns-frag-91f686dca9db", "codeNote": "DNS from First Principles/Code/github/workflows/repackage.yml.source", "sourcePath": ".github/workflows/repackage.yml", "startLine": 1, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-91f686dca9db" class="rgbdns-fragment-target"></span>
### rgbdns-frag-91f686dca9db: file repackage.yml

```yaml
name: Fast repackage

on:
  workflow_dispatch:
    inputs:
      base_run_id:
        description: Successful full package-build run whose binaries should be reused
        required: true
        type: string
      package:
        description: Package format to rebuild
        required: true
        default: both
        type: choice
        options:
          - both
          - deb
          - rpm

permissions:
  actions: read
  contents: read

jobs:
  deb:
    if: inputs.package == 'both' || inputs.package == 'deb'
    runs-on: ubuntu-24.04
    timeout-minutes: 15
    steps:
      - uses: actions/checkout@v5

      - name: Download binaries from the full Debian build
        uses: actions/download-artifact@v5
        with:
          github-token: ${{ github.token }}
          run-id: ${{ inputs.base_run_id }}
          name: rgbdns-debian-amd64
          path: base-package

      - name: Install Debian packaging tools
        run: |
          sudo apt-get update
          sudo apt-get install --yes debhelper

      - name: Reuse packaged binaries
        run: |
          mkdir -p target/release
          dpkg-deb --extract base-package/*.deb base-package/extracted
          cp base-package/extracted/usr/bin/* target/release/
          chmod 0755 target/release/*

      - name: Build package without Rust
        run: |
          RGBDNS_SKIP_RUST_BUILD=1 packaging/build-deb.sh -d

      - name: Test daily query reporting
        run: packaging/tests/test-daily-query-report.sh target/release/rgbdns-log-report

      - name: Collect package
        run: |
          mkdir -p dist
          cp ../rgbdns_*_amd64.deb dist/

      - uses: actions/upload-artifact@v4
        with:
          name: rgbdns-debian-amd64-fast
          path: dist/rgbdns_*_amd64.deb
          if-no-files-found: error

  rpm:
    if: inputs.package == 'both' || inputs.package == 'rpm'
    runs-on: ubuntu-24.04
    container: opensuse/leap:16.0
    timeout-minutes: 15
    steps:
      - name: Install checkout prerequisites
        run: zypper --non-interactive install git gzip tar

      - uses: actions/checkout@v5

      - name: Download binaries from the full RPM build
```
