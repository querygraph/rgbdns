---
type: "code-fragment"
fragment_id: "rgbdns-frag-b8d86ac1cb69"
source_path: "Cargo.toml"
code_note: "DNS from First Principles/Code/Cargo.toml.source"
language: "toml"
subsystem: "Repository and build"
symbol: "Cargo.toml"
kind: "file"
start_line: 1
end_line: 81
---

# Cargo.toml

- Fragment ID: `rgbdns-frag-b8d86ac1cb69`
- Source file: [[DNS from First Principles/Code/Cargo.toml.source|Cargo.toml]]
- Lines: 1-81
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-b8d86ac1cb69", "codeNote": "DNS from First Principles/Code/Cargo.toml.source", "heading": "rgbdns-frag-b8d86ac1cb69: file Cargo.toml", "sourcePath": "Cargo.toml", "startLine": 1, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-b8d86ac1cb69" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b8d86ac1cb69: file Cargo.toml

```toml
[package]
name = "rgbdns"
version = "0.1.0"
edition = "2024"
license = "Unlicense"
description = "A safe, faithful Rust reimplementation of the djbdns suite"

[dependencies]
cdb = "0.6"
chrono = "0.4"
getrandom = "0.3"
hickory-server = { version = "0.26.1", default-features = false, features = ["dnssec-ring", "recursor", "resolver"] }
ipnet = "2"
nix = { version = "0.31.3", features = ["process", "user"] }
tokio = { version = "1", features = ["macros", "net", "rt-multi-thread", "signal"] }
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
proptest = "1"

[profile.release]
lto = "thin"
codegen-units = 1
strip = true

[profile.bench]
lto = false
codegen-units = 16
strip = false

[[bench]]
name = "dns_core"
harness = false

[[bin]]
name = "tinydns"
path = "src/bin/tinydns.rs"
bench = false

[[bin]]
name = "tinydns-data"
path = "src/bin/tinydns-data.rs"
bench = false

[[bin]]
name = "tinydns-get"
path = "src/bin/tinydns-get.rs"
bench = false

[[bin]]
name = "dnsq"
path = "src/bin/dnsq.rs"
bench = false

[[bin]]
name = "dnscache"
path = "src/bin/dnscache.rs"
bench = false
test = false

[[bin]]
name = "axfrdns"
path = "src/bin/axfrdns.rs"
bench = false

[[bin]]
name = "axfr-get"
path = "src/bin/axfr-get.rs"
bench = false

[[bin]]
name = "rbldns"
path = "src/bin/rbldns.rs"
bench = false

[[bin]]
name = "rbldns-data"
path = "src/bin/rbldns-data.rs"
bench = false

[[bin]]
```
