---
type: "code-fragment"
fragment_id: "rgbdns-frag-2e3e3303acd3"
source_path: "docs/DNSSEC.md"
code_note: "DNS from First Principles/Code/docs/DNSSEC.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Parent DS and activation"
kind: "heading"
start_line: 121
end_line: 140
---

# Parent DS and activation

- Fragment ID: `rgbdns-frag-2e3e3303acd3`
- Source file: [[DNS from First Principles/Code/docs/DNSSEC.md.source|docs/DNSSEC.md]]
- Lines: 121-140
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-2e3e3303acd3", "codeNote": "DNS from First Principles/Code/docs/DNSSEC.md.source", "sourcePath": "docs/DNSSEC.md", "startLine": 121, "endLine": 140}
```

## Excerpt

<span id="rgbdns-frag-2e3e3303acd3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2e3e3303acd3: heading Parent DS and activation

````markdown
## Parent DS and activation

Print the DS line with `rgbsec-ds`, then install the exact key tag, algorithm
13, digest type 2, and digest at the parent. Do not publish the DS until every
delegated authority answers DNSKEY and signed positive and negative queries.

Useful checks are:

```sh
dig @127.0.0.1 example.com SOA +dnssec
dig @127.0.0.1 example.com DNSKEY +dnssec
dig @127.0.0.1 does-not-exist.example.com A +dnssec
dig @127.0.0.1 child.example.com DS +dnssec
delv example.com
```

After parent publication, monitor from more than one validating network. To
disable DNSSEC safely, remove the parent DS first, wait through its TTL and
negative caches, and only then remove local policy and republish unsigned data.

````
