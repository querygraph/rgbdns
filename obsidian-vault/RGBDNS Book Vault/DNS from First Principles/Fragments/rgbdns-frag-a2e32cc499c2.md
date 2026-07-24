---
type: "code-fragment"
fragment_id: "rgbdns-frag-a2e32cc499c2"
source_path: "src/bin/dnsfilter.rs"
code_note: "DNS from First Principles/Code/src/bin/dnsfilter.rs.source"
language: "rust"
subsystem: "Command-line programs"
crate: "dnsfilter"
symbol: "process"
kind: "fn"
start_line: 53
end_line: 85
---

# process

- Fragment ID: `rgbdns-frag-a2e32cc499c2`
- Source file: [[DNS from First Principles/Code/src/bin/dnsfilter.rs.source|src/bin/dnsfilter.rs]]
- Lines: 53-85
- Subsystem: [[DNS from First Principles/Subsystems/Command-line programs|Command-line programs]]
- Component: [[DNS from First Principles/Components/dnsfilter|dnsfilter]]

```rgbdns-fragment
{"id": "rgbdns-frag-a2e32cc499c2", "codeNote": "DNS from First Principles/Code/src/bin/dnsfilter.rs.source", "heading": "rgbdns-frag-a2e32cc499c2: fn process", "sourcePath": "src/bin/dnsfilter.rs", "startLine": 53, "endLine": 85}
```

## Excerpt

<span id="rgbdns-frag-a2e32cc499c2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a2e32cc499c2: fn process

```rust
fn process(lines: Vec<String>, concurrency: usize) -> Result<(), Box<dyn std::error::Error>> {
    let lines = Arc::new(lines);
    let next = Arc::new(AtomicUsize::new(0));
    let (sender, receiver) = mpsc::channel();
    thread::scope(|scope| {
        for _ in 0..concurrency.min(lines.len()) {
            let lines = lines.clone();
            let next = next.clone();
            let sender = sender.clone();
            scope.spawn(move || {
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(line) = lines.get(index) else {
                        break;
                    };
                    let _ = sender.send((index, filter_line(line)));
                }
            });
        }
    });
    drop(sender);
    let mut output = vec![String::new(); lines.len()];
    for (index, line) in receiver {
        output[index] = line;
    }
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for line in output {
        writeln!(stdout, "{line}")?;
    }
    Ok(())
}

```
