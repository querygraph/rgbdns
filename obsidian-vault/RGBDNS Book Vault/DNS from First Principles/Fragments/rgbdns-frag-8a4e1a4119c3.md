---
type: "code-fragment"
fragment_id: "rgbdns-frag-8a4e1a4119c3"
source_path: "ietf/README.md"
code_note: "DNS from First Principles/Code/ietf/README.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "ANAME Internet-Draft"
kind: "heading"
start_line: 1
end_line: 33
---

# ANAME Internet-Draft

- Fragment ID: `rgbdns-frag-8a4e1a4119c3`
- Source file: [[DNS from First Principles/Code/ietf/README.md.source|ietf/README.md]]
- Lines: 1-33
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-8a4e1a4119c3", "codeNote": "DNS from First Principles/Code/ietf/README.md.source", "sourcePath": "ietf/README.md", "startLine": 1, "endLine": 33}
```

## Excerpt

<span id="rgbdns-frag-8a4e1a4119c3" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8a4e1a4119c3: heading ANAME Internet-Draft

````markdown
# ANAME Internet-Draft

`draft-khrabrov-dnsop-aname-axfr-00.xml` is the editable RFCXML v3 source.
The generated `.txt` and `.html` files are review artifacts. The XML is the
file intended for submission to the IETF Datatracker.

The draft builds on `draft-ietf-dnsop-aname-04` and adds interoperable AXFR and
IXFR behavior informed by the rgbdns implementation and the published behavior
of Cloudflare, DNSimple, Amazon Route 53, IBM NS1, and PowerDNS.

Build with:

```sh
asdf install
make -C ietf check
```

`xml2rfc` validates RFCXML while rendering both publication formats.
`codespell` checks the source, rendered text, and this guide. `rfclint` performs
an independent schema and prose lint pass. Its optional ABNF and `aspell`
checks require those system programs when used.

Before submission:

1. Confirm the author name, organization, email address, and draft filename.
2. Discuss the design and code-point requests with the DNSOP working group.
3. Resolve working-group feedback and increment the draft revision.
4. Upload the RFCXML source through the
   [IETF submission tool](https://datatracker.ietf.org/submit/).

The experimental rgbdns values, EDNS option 65001 and private-use TYPE65401,
are implementation details. They are deliberately not requested from IANA.
The draft uses `TBD1` and `TBD2` for eventual assigned values.
````
