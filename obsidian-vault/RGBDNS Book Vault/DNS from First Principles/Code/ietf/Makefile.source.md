---
type: "code-file"
source_path: "ietf/Makefile"
language: ""
subsystem: "Repository and build"
line_count: 21
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# ietf/Makefile

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `ietf/Makefile`
- Lines: 21
- Summary: Source file in the Repository and build subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-01e29b1881fb|Makefile]]: lines 1-21

## Full Source

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
