---
type: "code-fragment"
fragment_id: "rgbdns-frag-a0a1070f29fa"
source_path: "docs/blog/rgbdns-dnssec/post.md"
code_note: "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "The parent adopts the child's key by digest"
kind: "heading"
start_line: 117
end_line: 153
---

# The parent adopts the child's key by digest

- Fragment ID: `rgbdns-frag-a0a1070f29fa`
- Source file: [[DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source|docs/blog/rgbdns-dnssec/post.md]]
- Lines: 117-153
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-a0a1070f29fa", "codeNote": "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source", "sourcePath": "docs/blog/rgbdns-dnssec/post.md", "startLine": 117, "endLine": 153}
```

## Excerpt

<span id="rgbdns-frag-a0a1070f29fa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-a0a1070f29fa: heading The parent adopts the child's key by digest

````markdown
## The parent adopts the child's key by digest

The child publishes DNSKEY. The parent publishes DS. `rgbsec-ds` connects the
two without asking an operator to reconstruct wire data by hand:

```sh
sudo rgbsec-ds "$(
  sudo grep '^Kexample[.]com[.]:' /etc/rgbdns/dnssec
)"
```

Registrar forms usually split the result into key tag, algorithm 13, digest
type 2, and a 64-hex-digit SHA-256 digest. The DS must not appear until every
delegated authority has the signed child data. Once it does appear, removing
signatures first makes the domain bogus. Safe rollback removes the parent DS,
waits through caches, confirms that resolvers no longer see it, and only then
returns the child to unsigned publication.

Two small public zones carried the rollout. `bitcoin.science` established the
first complete chain. Its parent publishes key tag 53856, algorithm 13, digest
type 2, and public resolvers validate both positive and negative answers.
`foto.gs` followed with key tag 38557. Its DS is now present at the `.gs` parent
and validates through independent recursive services.

The useful test is not merely “does DNSKEY exist?” It is an end-to-end fresh
negative query:

```sh
for resolver in 1.1.1.1 9.9.9.9 8.8.8.8; do
  dig +dnssec @"$resolver" \
    "MiXeD-$(date +%s).bitcoin.science" A
done
```

Each answer should be NXDOMAIN with the AD flag. The fresh owner avoids an old
negative cache entry. The mixed case exercises DNS 0x20 randomization.

````
