---
type: "code-fragment"
fragment_id: "rgbdns-frag-c1564a44db86"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "from_file_with_publication"
kind: "fn"
start_line: 259
end_line: 305
---

# from_file_with_publication

- Fragment ID: `rgbdns-frag-c1564a44db86`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 259-305
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-c1564a44db86", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 259, "endLine": 305}
```

## Excerpt

<span id="rgbdns-frag-c1564a44db86" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c1564a44db86: fn from_file_with_publication

```rust
    pub fn from_file_with_publication(
        config: impl AsRef<Path>,
        state_dir: impl AsRef<Path>,
        base: Zone,
        live: LiveZone,
        publication: Option<Publication>,
    ) -> Result<Self> {
        let policies = parse_config(&fs::read_to_string(config)?, &base)?;
        let state_dir = state_dir.as_ref().to_path_buf();
        let overlay = read_overlay(&state_dir.join("acme-overlay.data"))?;
        let serials = read_serials(&state_dir.join("acme-serials"))?;
        validate_overlay(&overlay, &policies)?;
        let signed = policies
            .values()
            .any(|policy| base.is_dnssec_signed(&policy.zone));
        let publication = if signed {
            Some(publication.ok_or_else(|| {
                Error::InvalidRecord(
                    "ACME updates to a signed zone require an external publication command".into(),
                )
            })?)
        } else {
            // A mixed snapshot may contain signed zones which have no ACME policy.
            // Applying an unsigned-zone overlay in memory preserves those signatures;
            // invoking a root-only signer here would be unnecessary and unsafe.
            None
        };
        let published = if let Some(publication) = &publication {
            publication.run(&state_dir, &overlay, &serials, &policies)?
        } else {
            base.with_acme_overlay(&flatten(&overlay, &policies)?, &serials)?
        };
        live.publish(published);
        Ok(Self {
            policies,
            base,
            live,
            state_dir,
            publication,
            state: Mutex::new(State {
                overlay,
                serials,
                recent_macs: VecDeque::new(),
            }),
        })
    }

```
