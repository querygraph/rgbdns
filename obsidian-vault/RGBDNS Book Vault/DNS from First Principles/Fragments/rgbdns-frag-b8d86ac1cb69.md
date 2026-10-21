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
{"id": "rgbdns-frag-b8d86ac1cb69", "codeNote": "DNS from First Principles/Code/Cargo.toml.source", "sourcePath": "Cargo.toml", "startLine": 1, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-b8d86ac1cb69" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b8d86ac1cb69: file Cargo.toml

```toml
[package]
name = "rgbdns"
version = "0.6.5"
edition = "2024"
license = "Unlicense"
description = "A safe, faithful Rust reimplementation of the djbdns suite"

[dependencies]
async-trait = "0.1"
base64 = "0.22"
cdb = "0.6"
chrono = "0.4"
fs2 = "0.4"
getrandom = "0.3"
hmac = "0.12"
hickory-server = { version = "0.26.1", default-features = false, features = ["dnssec-ring", "recursor", "resolver"] }
ipnet = "2"
nix = { version = "0.31.3", features = ["process", "user"] }
rustls-pki-types = "1"
sha2 = "0.10"
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
name = "rgbdns-acme"
path = "src/bin/rgbdns-acme.rs"
bench = false

[[bin]]
name = "rgbdns-log-report"
path = "src/bin/rgbdns-log-report.rs"
bench = false

[[bin]]
name = "tinydns-data"
path = "src/bin/tinydns-data.rs"
bench = false

[[bin]]
name = "rgbsec-keygen"
path = "src/bin/rgbsec-keygen.rs"
bench = false

[[bin]]
name = "rgbsec-sign"
path = "src/bin/rgbsec-sign.rs"
bench = false

[[bin]]
name = "rgbsec-data"
path = "src/bin/rgbsec-data.rs"
bench = false

[[bin]]
name = "rgbsec-ds"
path = "src/bin/rgbsec-ds.rs"
bench = false

[[bin]]
```
