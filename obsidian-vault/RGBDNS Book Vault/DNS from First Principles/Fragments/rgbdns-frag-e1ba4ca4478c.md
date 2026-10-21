---
type: "code-fragment"
fragment_id: "rgbdns-frag-e1ba4ca4478c"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Activating the parent DS"
kind: "heading"
start_line: 915
end_line: 957
---

# Activating the parent DS

- Fragment ID: `rgbdns-frag-e1ba4ca4478c`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 915-957
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-e1ba4ca4478c", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "sourcePath": "docs/book/rgbdns.md", "startLine": 915, "endLine": 957}
```

## Excerpt

<span id="rgbdns-frag-e1ba4ca4478c" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e1ba4ca4478c: heading Activating the parent DS

````markdown
## Activating the parent DS

Signing the child does not complete the chain. The parent must publish a DS
that matches the active child DNSKEY. Derive it from the installed policy
rather than transcribing public-key material by hand:

```sh
sudo rgbsec-ds "$(
  sudo grep '^Kexample[.]com[.]:' /etc/rgbdns/dnssec
)"
```

The output is a presentation-format record:

```text
example.com. IN DS <key-tag> 13 2 <64-hex-sha256-digest>
```

That line is illustrative; use the output for the actual key. Registrar forms
usually ask for four fields: key tag, algorithm 13, digest type 2, and the
SHA-256 digest. Publish it only after every nameserver in the parent delegation
serves the matching signed snapshot. A stale third-party secondary is enough
to make validation intermittent.

Check the parent directly, then use several independent validating resolvers:

```sh
dig +short +nosplit example.com DS

for resolver in 1.1.1.1 9.9.9.9 8.8.8.8; do
  dig +dnssec @"$resolver" example.com A
  dig +dnssec @"$resolver" \
    "MiXeD-$(date +%s).example.com" A
done
```

A secure positive response is `NOERROR` with the AD flag. A secure negative
response is NXDOMAIN with AD. Querying a fresh name avoids mistaking an old
negative cache entry for current behavior. The parent DS, child DNSKEY, and
signatures can be correct while a resolver still exposes a transport,
delegation, case-handling, or denial-proof defect; end-to-end testing is what
turns a collection of records into evidence.

````
