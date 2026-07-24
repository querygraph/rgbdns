---
type: "code-fragment"
fragment_id: "rgbdns-frag-241a6862cf68"
source_path: "src/tai64.rs"
code_note: "DNS from First Principles/Code/src/tai64.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "LEAP_TRANSITIONS"
kind: "const"
start_line: 13
end_line: 43
---

# LEAP_TRANSITIONS

- Fragment ID: `rgbdns-frag-241a6862cf68`
- Source file: [[DNS from First Principles/Code/src/tai64.rs.source|src/tai64.rs]]
- Lines: 13-43
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-241a6862cf68", "codeNote": "DNS from First Principles/Code/src/tai64.rs.source", "heading": "rgbdns-frag-241a6862cf68: const LEAP_TRANSITIONS", "sourcePath": "src/tai64.rs", "startLine": 13, "endLine": 43}
```

## Excerpt

<span id="rgbdns-frag-241a6862cf68" class="rgbdns-fragment-target"></span>
### rgbdns-frag-241a6862cf68: const LEAP_TRANSITIONS

```rust
const LEAP_TRANSITIONS: [i64; 27] = [
    78_796_800,
    94_694_400,
    126_230_400,
    157_766_400,
    189_302_400,
    220_924_800,
    252_460_800,
    283_996_800,
    315_532_800,
    362_793_600,
    394_329_600,
    425_865_600,
    489_024_000,
    567_993_600,
    631_152_000,
    662_688_000,
    709_948_800,
    741_484_800,
    773_020_800,
    820_454_400,
    867_715_200,
    915_148_800,
    1_136_073_600,
    1_230_768_000,
    1_341_100_800,
    1_435_708_800,
    1_483_228_800,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
```
