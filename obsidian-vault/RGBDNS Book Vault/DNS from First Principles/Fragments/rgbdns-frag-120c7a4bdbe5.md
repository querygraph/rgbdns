---
type: "code-fragment"
fragment_id: "rgbdns-frag-120c7a4bdbe5"
source_path: "docs/conformance.md"
code_note: "DNS from First Principles/Code/docs/conformance.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Independent and ecosystem sources"
kind: "heading"
start_line: 31
end_line: 52
---

# Independent and ecosystem sources

- Fragment ID: `rgbdns-frag-120c7a4bdbe5`
- Source file: [[DNS from First Principles/Code/docs/conformance.md.source|docs/conformance.md]]
- Lines: 31-52
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-120c7a4bdbe5", "codeNote": "DNS from First Principles/Code/docs/conformance.md.source", "sourcePath": "docs/conformance.md", "startLine": 31, "endLine": 52}
```

## Excerpt

<span id="rgbdns-frag-120c7a4bdbe5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-120c7a4bdbe5: heading Independent and ecosystem sources

```markdown
## Independent and ecosystem sources

The matrix was cross-checked against:

- ISC's [EDNS compliance program](https://ednscomp.isc.org/), whose cases
  informed the EDNS flag/version/option combinations;
- the upstream
  [DNS Compliance Testing](https://gitlab.isc.org/isc-projects/DNS-Compliance-Testing)
  tool for authoritative and recursive servers;
- DNSimple's [dnstest](https://github.com/dnsimple/dnstest), derived from the
  PowerDNS regression suite;
- ldns `drill`, which is executed locally as an independent encoder, decoder,
  UDP/TCP client, and EDNS client;
- research on systematic DNS testing:
  [SCALE](https://www.microsoft.com/en-us/research/publication/scale-automatically-finding-rfc-compliance-bugs-in-dns-nameservers/),
  ResolverFuzz, and Eywa. Their stateful mutation and differential-testing
  findings motivate the structured generators, truncation corpus, response
  matching, and cache/AXFR boundary checks.

No third-party test fixture is copied into this repository. Test names cite the
normative RFC that supplies each oracle.

```
