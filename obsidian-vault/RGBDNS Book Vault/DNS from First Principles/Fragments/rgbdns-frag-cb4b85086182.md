---
type: "code-fragment"
fragment_id: "rgbdns-frag-cb4b85086182"
source_path: "src/bin/random-ip.rs"
code_note: "DNS from First Principles/Code/src/bin/random-ip.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "random-ip"
symbol: "run"
kind: "fn"
start_line: 8
end_line: 61
---

# run

- Fragment ID: `rgbdns-frag-cb4b85086182`
- Source file: [[DNS from First Principles/Code/src/bin/random-ip.rs.source|src/bin/random-ip.rs]]
- Lines: 8-61
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/random-ip|random-ip]]

```rgbdns-fragment
{"id": "rgbdns-frag-cb4b85086182", "codeNote": "DNS from First Principles/Code/src/bin/random-ip.rs.source", "heading": "rgbdns-frag-cb4b85086182: fn run", "sourcePath": "src/bin/random-ip.rs", "startLine": 8, "endLine": 61}
```

## Excerpt

<span id="rgbdns-frag-cb4b85086182" class="rgbdns-fragment-target"></span>
### rgbdns-frag-cb4b85086182: fn run

```rust
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() > 5 {
        return Err("usage: random-ip [ count [ octet ... ] ]".into());
    }
    let mut loops = arguments
        .first()
        .map_or(Ok(10_000u64), |value| value.parse())?;
    let fixed = arguments[1..]
        .iter()
        .map(|value| value.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?;
    let maximum = 1u64 << (8 * (4 - fixed.len()));
    loops = loops.min(maximum);

    let mut table = [0u8; 256];
    for (index, value) in table.iter_mut().enumerate() {
        *value = index as u8;
    }
    for end in (1..=256).rev() {
        let index = uniform(end as u32)
            .map_err(|error| std::io::Error::other(format!("OS randomness unavailable: {error}")))?
            as usize;
        table.swap(end - 1, index);
    }
    while loops > 0 {
        loops -= 1;
        let mut address = [0; 4];
        address[..fixed.len()].copy_from_slice(&fixed);
        let mut value = loops;
        for octet in &mut address[fixed.len()..] {
            *octet = value as u8;
            value >>= 8;
        }
        if fixed.len() == 3 {
            address[3] = table[address[3] as usize];
        } else if fixed.len() < 3 {
            let mut state = 0u8;
            for _ in 0..100 {
                for octet in &mut address[fixed.len()..] {
                    state ^= *octet;
                    state = table[state as usize];
                    *octet = state;
                }
            }
        }
        println!(
            "{}.{}.{}.{}",
            address[0], address[1], address[2], address[3]
        );
    }
    Ok(())
}

```
