---
type: "code-fragment"
fragment_id: "rgbdns-frag-675a90298dbe"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "The packet is hostile"
kind: "heading"
start_line: 813
end_line: 840
---

# The packet is hostile

- Fragment ID: `rgbdns-frag-675a90298dbe`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 813-840
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-675a90298dbe", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-675a90298dbe: heading The packet is hostile", "sourcePath": "docs/book/rgbdns.md", "startLine": 813, "endLine": 840}
```

## Excerpt

<span id="rgbdns-frag-675a90298dbe" class="rgbdns-fragment-target"></span>
### rgbdns-frag-675a90298dbe: heading The packet is hostile

```markdown
## The packet is hostile

DNS combines nearly every parser hazard: nested lengths, compression pointers,
variable counts, binary strings, recursive relationships, and network-facing
availability requirements. “Written in Rust” removes broad classes of memory
corruption, but it does not automatically prevent allocation bombs, infinite
loops, CPU amplification, path races, policy errors, or accepting incoherent
messages.

rgbdns therefore uses several layers:

- `#![forbid(unsafe_code)]` for the library;
- explicit bounds before every wire read;
- validated `Name`, `Message`, and `RData` objects;
- limits on compression traversal, aliases, records, files, configuration
  lists, recursion, transfers, and cache sizes;
- cryptographic operating-system randomness for query IDs and selection;
- complete-record truncation;
- loopback-only defaults for recursion and transfer;
- atomic replacement for compiled databases and fetched zones;
- no shell interpolation when replacing a process.

Property tests in `tests/packet_properties.rs` feed arbitrary bytes to the
decoder and exercise encode/decode invariants. Golden CDB fixtures compare
compiled output with the expected djbdns layout. Network tests cross real UDP
and TCP boundaries. Compatibility tests are valuable here because a parser can
be safe yet subtly wrong, or compatible yet unsafe.

```
