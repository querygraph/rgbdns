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
