---
type: "code-fragment"
fragment_id: "rgbdns-frag-ffe077349307"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "16. Observe request and transfer logs"
kind: "heading"
start_line: 653
end_line: 682
---

# 16. Observe request and transfer logs

- Fragment ID: `rgbdns-frag-ffe077349307`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 653-682
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-ffe077349307", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 653, "endLine": 682}
```

## Excerpt

<span id="rgbdns-frag-ffe077349307" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ffe077349307: heading 16. Observe request and transfer logs

````markdown
## 16. Observe request and transfer logs

Request logging is enabled by default. Follow primary activity:

```sh
sudo journalctl -fu rgbdns-tinydns.service \
  -u rgbdns-data-import.service
```

Follow secondary activity:

```sh
sudo journalctl -fu rgbdns-tinydns.service \
  -u rgbdns-zones-import.service \
  -u rgbdns-secondary-sync.service
```

Disable per-request logging only when explicitly intended:

```ini
QUERY_LOG=0
```

Restart tinydns after manually changing that environment file.

### Send the daily per-domain report

On the primary, configure a sendmail-compatible SMTP transport and write:

```text
````
