---
type: "code-fragment"
fragment_id: "rgbdns-frag-e78faa9dfca9"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Truncation must preserve a valid message"
kind: "heading"
start_line: 340
end_line: 355
---

# Truncation must preserve a valid message

- Fragment ID: `rgbdns-frag-e78faa9dfca9`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 340-355
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-e78faa9dfca9", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-e78faa9dfca9: heading Truncation must preserve a valid message", "sourcePath": "docs/book/rgbdns.md", "startLine": 340, "endLine": 355}
```

## Excerpt

<span id="rgbdns-frag-e78faa9dfca9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e78faa9dfca9: heading Truncation must preserve a valid message

```markdown
## Truncation must preserve a valid message

Cutting the last bytes off an encoded message creates a malformed packet.
Correct truncation removes complete records, sets TC, updates section counts,
and re-encodes. A useful removal order discards nonessential additional data
before authority and answer data. OPT sometimes needs special treatment
because it carries the EDNS response.

`src/server.rs` calculates a response limit from the caller’s transport limit
and the client’s EDNS advertisement. It caps advertised UDP size, rejects
multiple OPT records, responds to unsupported EDNS versions, and constructs a
full typed response. If encoding exceeds the limit, `truncate` sets TC and
removes complete records in a defined order until the packet fits. The same
core response logic serves UDP and TCP without treating TCP as a giant UDP
datagram.

```
