---
type: "code-fragment"
fragment_id: "rgbdns-frag-b36c3ba84f08"
source_path: "ietf/draft-khrabrov-dnsop-aname-axfr-00.html"
code_note: "DNS from First Principles/Code/ietf/draft-khrabrov-dnsop-aname-axfr-00.html.source"
language: "html"
subsystem: "Repository and build"
symbol: "draft-khrabrov-dnsop-aname-axfr-00.html"
kind: "file"
start_line: 1
end_line: 81
---

# draft-khrabrov-dnsop-aname-axfr-00.html

- Fragment ID: `rgbdns-frag-b36c3ba84f08`
- Source file: [[DNS from First Principles/Code/ietf/draft-khrabrov-dnsop-aname-axfr-00.html.source|ietf/draft-khrabrov-dnsop-aname-axfr-00.html]]
- Lines: 1-81
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-b36c3ba84f08", "codeNote": "DNS from First Principles/Code/ietf/draft-khrabrov-dnsop-aname-axfr-00.html.source", "sourcePath": "ietf/draft-khrabrov-dnsop-aname-axfr-00.html", "startLine": 1, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-b36c3ba84f08" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b36c3ba84f08: file draft-khrabrov-dnsop-aname-axfr-00.html

```html
<!DOCTYPE html>
<html lang="en" class="Internet-Draft">
<head>
<meta charset="utf-8">
<meta content="Common,Latin" name="scripts">
<meta content="initial-scale=1.0" name="viewport">
<title>Address-specific DNS Aliases (ANAME) and Zone Transfer</title>
<meta content="Alexy Khrabrov" name="author">
<meta content="
       This document defines the ANAME DNS resource record.  ANAME provides
      name-to-name indirection for address queries while allowing other
      resource record types to exist at the same owner name.  It is therefore
      usable at a zone apex. 
       This document also defines authoritative processing, TTL and failure
      behavior, DNSSEC considerations, and interoperable transport of ANAME
      records in full and incremental zone transfers.  In particular, each
      ANAME-capable authoritative server resolves the transferred target
      independently.  This avoids treating transient, synthesized address
      records as the portable source of zone data. 
    " name="description">
<meta content="xml2rfc 3.34.0" name="generator">
<meta content="DNS" name="keyword">
<meta content="ANAME" name="keyword">
<meta content="ALIAS" name="keyword">
<meta content="CNAME flattening" name="keyword">
<meta content="AXFR" name="keyword">
<meta content="draft-khrabrov-dnsop-aname-axfr-00" name="ietf.draft">
<!-- Generator version information:
  xml2rfc 3.34.0
    Python 3.12.8
    ConfigArgParse 1.7.5
    google-i18n-address 3.1.1
    intervaltree 3.2.1
    Jinja2 3.1.6
    lxml 6.1.1
    natsort 8.4.0
    platformdirs 4.11.0
    pycountry 26.2.16
    PyYAML 6.0.3
    requests 2.34.2
    wcwidth 0.8.2
-->
<link href="draft-khrabrov-dnsop-aname-axfr-00.xml" rel="alternate" type="application/rfc+xml">
<link href="#copyright" rel="license">
<style type="text/css">/*

  NOTE: Changes at the bottom of this file overrides some earlier settings.

  Once the style has stabilized and has been adopted as an official RFC style,
  this can be consolidated so that style settings occur only in one place, but
  for now the contents of this file consists first of the initial CSS work as
  provided to the RFC Formatter (xml2rfc) work, followed by itemized and
  commented changes found necessary during the development of the v3
  formatters.

*/

/* fonts */
@import url('https://static.ietf.org/fonts/noto-sans/import.css'); /* Sans-serif */
@import url('https://static.ietf.org/fonts/noto-serif/import.css'); /* Serif (print) */
@import url('https://static.ietf.org/fonts/roboto-mono/import.css'); /* Monospace */

:root {
  --font-sans: 'Noto Sans', Arial, Helvetica, sans-serif;
  --font-serif: 'Noto Serif', 'Times', 'Times New Roman', serif;
  --font-mono: 'Roboto Mono', Courier, 'Courier New', monospace;
}

@viewport {
  zoom: 1.0;
}
@-ms-viewport {
  width: extend-to-zoom;
  zoom: 1.0;
}
/* general and mobile first */
html {
}
body {
  max-width: 90%;
  margin: 1.5em auto;
```
