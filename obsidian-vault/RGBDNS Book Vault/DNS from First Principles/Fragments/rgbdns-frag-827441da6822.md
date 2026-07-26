---
type: "code-fragment"
fragment_id: "rgbdns-frag-827441da6822"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "CDB compatibility without trusting the file"
kind: "heading"
start_line: 1661
end_line: 1687
---

# CDB compatibility without trusting the file

- Fragment ID: `rgbdns-frag-827441da6822`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1661-1687
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-827441da6822", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-827441da6822: heading CDB compatibility without trusting the file", "sourcePath": "docs/book/rgbdns.md", "startLine": 1661, "endLine": 1687}
```

## Excerpt

<span id="rgbdns-frag-827441da6822" class="rgbdns-fragment-target"></span>
### rgbdns-frag-827441da6822: heading CDB compatibility without trusting the file

```markdown
# CDB compatibility without trusting the file

Compatibility is most valuable at the data boundary. rgbdns reads and writes
the original tinydns `data.cdb` layout, so operators can preserve compilation
and rollout habits. [`cdb.rs`](https://github.com/querygraph/rgbdns/blob/master/src/cdb.rs) does not, however, inherit the
old assumption that the compiled file is trustworthy.

The loader applies independent limits and checked arithmetic:

- the complete database is capped at one GiB;
- the 2,048-byte CDB header must exist;
- every hash-table position and slot count must fit inside the file;
- key and value lengths use `checked_add`;
- markers, locations, names, TTLs, cutoffs, and type-specific RDATA are
  validated before a `Record` enters a `Zone`.

This is a useful modernization pattern: preserve a durable external format,
replace its implicit in-memory trust model. Operators gain compatibility; the
serving process receives validated Rust values rather than pointers into a
memory-mapped byte region.

Compilation likewise crosses an explicit boundary. A parsed `Zone` is written
to a temporary CDB and then installed through the command workflow. Serving
data is immutable between deployments. Rust’s ownership does not itself make
the rollout atomic, but it makes the stages—source text, validated model,
compiled bytes, installed file—unambiguous.

```
