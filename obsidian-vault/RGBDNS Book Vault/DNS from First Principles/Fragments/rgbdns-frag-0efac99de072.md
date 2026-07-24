---
type: "code-fragment"
fragment_id: "rgbdns-frag-0efac99de072"
source_path: "tests/drill_interop.rs"
code_note: "DNS from First Principles/Code/tests/drill_interop.rs.source"
language: "rust"
subsystem: "Tests and performance"
symbol: "drop"
kind: "fn"
start_line: 16
end_line: 22
---

# drop

- Fragment ID: `rgbdns-frag-0efac99de072`
- Source file: [[DNS from First Principles/Code/tests/drill_interop.rs.source|tests/drill_interop.rs]]
- Lines: 16-22
- Subsystem: [[DNS from First Principles/Subsystems/Tests and performance|Tests and performance]]

```rgbdns-fragment
{"id": "rgbdns-frag-0efac99de072", "codeNote": "DNS from First Principles/Code/tests/drill_interop.rs.source", "heading": "rgbdns-frag-0efac99de072: fn drop", "sourcePath": "tests/drill_interop.rs", "startLine": 16, "endLine": 22}
```

## Excerpt

<span id="rgbdns-frag-0efac99de072" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0efac99de072: fn drop

```rust
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_file(&self.data);
    }
}

```
