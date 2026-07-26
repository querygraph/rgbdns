---
type: "code-fragment"
fragment_id: "rgbdns-frag-144c116f58ed"
source_path: "docs/DEBIAN.md"
code_note: "DNS from First Principles/Code/docs/DEBIAN.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Build and install the package"
kind: "heading"
start_line: 8
end_line: 30
---

# Build and install the package

- Fragment ID: `rgbdns-frag-144c116f58ed`
- Source file: [[DNS from First Principles/Code/docs/DEBIAN.md.source|docs/DEBIAN.md]]
- Lines: 8-30
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-144c116f58ed", "codeNote": "DNS from First Principles/Code/docs/DEBIAN.md.source", "heading": "rgbdns-frag-144c116f58ed: heading Build and install the package", "sourcePath": "docs/DEBIAN.md", "startLine": 8, "endLine": 30}
```

## Excerpt

<span id="rgbdns-frag-144c116f58ed" class="rgbdns-fragment-target"></span>
### rgbdns-frag-144c116f58ed: heading Build and install the package

```markdown
## Build and install the package

Use a current Debian or Ubuntu build host with Rust, Cargo, debhelper, and the
ordinary C build tools:

```sh
sudo apt update
sudo apt install build-essential cargo debhelper rustc
git clone https://github.com/querygraph/rgbdns.git
cd rgbdns
packaging/build-deb.sh
sudo apt install ../rgbdns_0.1.1_$(dpkg --print-architecture).deb
```

`packaging/build-deb.sh` calls `dpkg-buildpackage --build=binary --no-sign`.
Debian's package builder runs the release build and complete Rust test suite.
The resulting package is architecture-specific because it contains native Rust
binaries. Build it on the same Debian architecture as the destination, or use
a proper Debian cross-build environment.

The install step discovers every binary target through `cargo metadata`; the
Debian rules do not maintain a second, manually synchronized program list.

```
