---
type: "code-fragment"
fragment_id: "rgbdns-frag-b62ce97d1c7b"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Other apex data remains independent"
kind: "heading"
start_line: 456
end_line: 536
---

# Other apex data remains independent

- Fragment ID: `rgbdns-frag-b62ce97d1c7b`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 456-536
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-b62ce97d1c7b", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-b62ce97d1c7b: heading Other apex data remains independent", "sourcePath": "docs/book/rgbdns.md", "startLine": 456, "endLine": 536}
```

## Excerpt

<span id="rgbdns-frag-b62ce97d1c7b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b62ce97d1c7b: heading Other apex data remains independent

```markdown
# Other apex data remains independent.
@example.com::mail.example.com:10:3600
'example.com:v=spf1 -all:3600
```

The form is:

```text
Aowner:target:maximum-ttl
```

The TTL field is optional and defaults to 300 seconds. It is a ceiling, not a
promise to extend the target’s lifetime. If the upstream address has 45
seconds remaining and the ANAME limit is 300, rgbdns returns 45. If the
upstream has 900 seconds remaining, rgbdns returns no more than 300.

ANAME is stored separately from ordinary `Record` values. It can coexist with
SOA, NS, MX, TXT, CAA, and other non-address data. Zone validation rejects:

- A, AAAA, or CNAME data at the same owner;
- a wildcard ANAME owner;
- an owner that targets itself;
- different ANAME targets at one owner;
- a zero TTL.

The server only applies ANAME to A and AAAA questions. SOA, NS, MX, TXT, CAA,
and all other questions continue through normal authoritative lookup. ANAME
also does not override a delegation cut: a name beneath a delegated child
still produces a referral from the parent.

For an address question, the response path is:

1. establish that ordinary authoritative lookup reaches the ANAME owner and
   does not cross a delegation;
2. query the configured recursive resolver for the target and requested
   address family;
3. validate response identity and framing;
4. follow only a connected CNAME chain beginning at the configured target;
5. collect the terminal A or AAAA RRset;
6. replace each terminal owner with the ANAME owner;
7. cap the remaining TTL and return an authoritative answer.

For example, an upstream result such as:

```text
customer.blog-host.example. 180 IN CNAME edge.host.example.
edge.host.example.          120 IN A     192.0.2.80
```

becomes:

```text
example.com.                120 IN A     192.0.2.80
```

The CNAME is deliberately absent. Consumers see a conventional authoritative
address RRset at the apex.

The resolver cache is shared by requests handled by one server process.
Positive entries expire with the upstream chain’s shortest relevant TTL.
Negative results use the authority SOA’s negative TTL when available and 60
seconds otherwise. The configured ANAME ceiling is applied when constructing
each response, so two owners may safely share a target while using different
TTL policies.

Resolution is bounded in the same spirit as the rest of rgbdns:

- CNAME chains stop after 16 links;
- visited names detect cycles;
- no more than 64 terminal addresses are accepted;
- conflicting CNAME targets are rejected;
- upstream SERVFAIL and other resolver errors become authoritative SERVFAIL,
  not false NODATA;
- A and AAAA are cached independently.

`DNSCACHEIP` selects one or more recursive endpoints, separated by commas.
Each endpoint may be an IP address using port 53 or an explicit socket address.
Without it, rgbdns reads `/etc/resolv.conf`. A local validating `dnscache` is
the preferred upstream when operators want DNSSEC validation and a cache shared
with other local DNS work:

```
