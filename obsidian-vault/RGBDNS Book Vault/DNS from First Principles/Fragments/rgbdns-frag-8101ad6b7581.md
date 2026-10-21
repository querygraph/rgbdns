---
type: "code-fragment"
fragment_id: "rgbdns-frag-8101ad6b7581"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "A query is more than sending bytes"
kind: "heading"
start_line: 1113
end_line: 1142
---

# A query is more than sending bytes

- Fragment ID: `rgbdns-frag-8101ad6b7581`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1113-1142
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-8101ad6b7581", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 1113, "endLine": 1142}
```

## Excerpt

<span id="rgbdns-frag-8101ad6b7581" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8101ad6b7581: heading A query is more than sending bytes

```markdown
## A query is more than sending bytes

A DNS client creates a random transaction ID, encodes one question, sends it
to an intended server, receives a response, and validates at least:

- source endpoint where the transport permits;
- transaction ID;
- QR and response shape;
- matching question;
- declared section lengths and names;
- truncation, with TCP retry when needed.

`src/client.rs` reads `DNSCACHEIP` or `/etc/resolv.conf`, supports IPv4 and
IPv6 socket syntax, gets IDs from the operating system, applies UDP timeouts,
rejects mismatched responses, and retries truncated UDP replies over TCP. The
small command binaries format results for different use cases, while `dnsq`
allows an explicit server and `dnsqr` uses recursive configuration.

`dnstrace` is conceptually different from a recursive lookup: it exposes the
delegation path and intermediate authority so an operator can see where the
chain stops. Good diagnosis asks four separate questions:

1. What did the stub send?
2. What did the recursive resolver cache or validate?
3. What delegation did the parent publish?
4. What does the authoritative server say directly?

Testing only the final application collapses all four layers and encourages
guessing.

```
