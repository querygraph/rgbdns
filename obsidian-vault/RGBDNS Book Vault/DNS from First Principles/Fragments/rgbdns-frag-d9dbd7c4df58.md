---
type: "code-fragment"
fragment_id: "rgbdns-frag-d9dbd7c4df58"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Forward zones and djbdns roots"
kind: "heading"
start_line: 623
end_line: 641
---

# Forward zones and djbdns roots

- Fragment ID: `rgbdns-frag-d9dbd7c4df58`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 623-641
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-d9dbd7c4df58", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-d9dbd7c4df58: heading Forward zones and djbdns roots", "sourcePath": "docs/book/rgbdns.md", "startLine": 623, "endLine": 641}
```

## Excerpt

<span id="rgbdns-frag-d9dbd7c4df58" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d9dbd7c4df58: heading Forward zones and djbdns roots

```markdown
## Forward zones and djbdns roots

Private namespaces and split DNS often need selected suffixes sent to specific
servers. rgbdns reads forward-zone configuration from the environment and the
djbdns-style `ROOT/servers` directory. The filename identifies a suffix and
the file lists bounded server addresses.

The special `servers/@` file represents root servers. Hickory consumes a root
hints file in master-file syntax, so `PreparedRoots` translates djbdns’s plain
address list into a private temporary file. Creation uses restrictive
permissions and cleanup occurs when the prepared object is dropped. This
adapter preserves the external configuration contract without weakening the
library boundary.

Forwarded private zones disable strict case-randomization response matching
because legacy authorities may canonicalize owner case. They retain TCP retry
and a bounded cache. This is a scoped compatibility decision, not a global
removal of query hardening.

```
