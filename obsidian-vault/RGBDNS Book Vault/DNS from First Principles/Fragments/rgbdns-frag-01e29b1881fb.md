---
type: "code-fragment"
fragment_id: "rgbdns-frag-01e29b1881fb"
source_path: "ietf/Makefile"
code_note: "DNS from First Principles/Code/ietf/Makefile.source"
language: ""
subsystem: "Repository and build"
symbol: "Makefile"
kind: "file"
start_line: 1
end_line: 21
---

# Makefile

- Fragment ID: `rgbdns-frag-01e29b1881fb`
- Source file: [[DNS from First Principles/Code/ietf/Makefile.source|ietf/Makefile]]
- Lines: 1-21
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-01e29b1881fb", "codeNote": "DNS from First Principles/Code/ietf/Makefile.source", "sourcePath": "ietf/Makefile", "startLine": 1, "endLine": 21}
```

## Excerpt

<span id="rgbdns-frag-01e29b1881fb" class="rgbdns-fragment-target"></span>
### rgbdns-frag-01e29b1881fb: file Makefile

```
DRAFT = draft-khrabrov-dnsop-aname-axfr-00
XML2RFC = asdf exec uvx --from xml2rfc xml2rfc

.PHONY: all check clean

all: $(DRAFT).txt $(DRAFT).html

check: all
	asdf exec uvx --from codespell codespell \
		$(DRAFT).xml $(DRAFT).txt README.md \
		--ignore-words-list=aname,nd
	asdf exec uvx --from rfclint rfclint $(DRAFT).xml

$(DRAFT).txt: $(DRAFT).xml
	$(XML2RFC) --text --out $@ $<

$(DRAFT).html: $(DRAFT).xml
	$(XML2RFC) --html --out $@ $<

clean:
	rm -f $(DRAFT).txt $(DRAFT).html
```
