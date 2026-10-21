---
type: "code-fragment"
fragment_id: "rgbdns-frag-f632c00a7d2c"
source_path: "src/acme.rs"
code_note: "DNS from First Principles/Code/src/acme.rs.source"
language: "rust"
subsystem: "Rust library"
crate: "rgbdns"
symbol: "handle"
kind: "fn"
start_line: 312
end_line: 392
---

# handle

- Fragment ID: `rgbdns-frag-f632c00a7d2c`
- Source file: [[DNS from First Principles/Code/src/acme.rs.source|src/acme.rs]]
- Lines: 312-392
- Subsystem: [[DNS from First Principles/Subsystems/Rust library|Rust library]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-f632c00a7d2c", "codeNote": "DNS from First Principles/Code/src/acme.rs.source", "sourcePath": "src/acme.rs", "startLine": 312, "endLine": 392}
```

## Excerpt

<span id="rgbdns-frag-f632c00a7d2c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f632c00a7d2c: fn handle

```rust
    pub fn handle(&self, wire: &[u8]) -> Result<Vec<u8>> {
        let parsed = match parse_update(wire) {
            Ok(value) => value,
            Err(_) => return bare_response(wire, 1),
        };
        let Some(policy) = self.policies.get(&parsed.tsig.key_name) else {
            return bare_response(wire, 9);
        };
        if parsed.zone != policy.zone
            || parsed.flags & 0x7800 != OPCODE_UPDATE
            || parsed.tsig.original_id != parsed.id
            || parsed.tsig.error != 0
            || !parsed.tsig.other.is_empty()
            || !is_hmac_sha256(&parsed.tsig.algorithm)
        {
            return signed_response(&parsed, policy, 9, 0, &[]);
        }
        let now = unix_now();
        if parsed.tsig.fudge > 300
            || now.abs_diff(parsed.tsig.time_signed) > u64::from(parsed.tsig.fudge)
        {
            return signed_response(&parsed, policy, 9, 18, &now.to_be_bytes()[2..]);
        }
        let expected = request_mac(&parsed, policy)?;
        if parsed.tsig.mac.len() != 32 || !constant_time_eq(&expected, &parsed.tsig.mac) {
            return signed_response(&parsed, policy, 9, 16, &[]);
        }
        let update_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(self.state_dir.join("acme-update.lock"))?;
        update_lock.lock_exclusive()?;
        let mut state = self.state.lock().expect("ACME state lock poisoned");
        prune_replays(&mut state.recent_macs, now);
        if state
            .recent_macs
            .iter()
            .any(|(_, mac)| constant_time_eq(mac, &parsed.tsig.mac))
        {
            return signed_response(&parsed, policy, 9, 16, &[]);
        }
        state.recent_macs.push_back((now, parsed.tsig.mac.clone()));

        let rcode = match apply_update(&parsed, policy, &mut state.overlay) {
            Ok(()) => {
                let canonical = self.base.soa_serial(&policy.zone).unwrap_or(1);
                let previous = state
                    .serials
                    .get(&policy.zone)
                    .copied()
                    .unwrap_or(canonical);
                let serial = next_serial(previous, canonical, now as u32);
                state.serials.insert(policy.zone.clone(), serial);
                let records = flatten(&state.overlay, &self.policies)?;
                let candidate = self.base.with_acme_overlay(&records, &state.serials)?;
                persist(
                    &self.state_dir,
                    &state.overlay,
                    &state.serials,
                    &self.policies,
                )?;
                let published = if let Some(publication) = &self.publication {
                    publication.run(
                        &self.state_dir,
                        &state.overlay,
                        &state.serials,
                        &self.policies,
                    )
                } else {
                    Ok(candidate)
                };
                match published {
                    Ok(published) => {
                        self.live.publish(published);
                        0
                    }
                    Err(error) => {
                        eprintln!("rgbdns ACME publication: {error}");
                        2
```
