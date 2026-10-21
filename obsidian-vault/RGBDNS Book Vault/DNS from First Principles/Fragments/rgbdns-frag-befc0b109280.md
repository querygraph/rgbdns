---
type: "code-fragment"
fragment_id: "rgbdns-frag-befc0b109280"
source_path: "vault.build.json"
code_note: "DNS from First Principles/Code/vault.build.json.source"
language: "json"
subsystem: "Repository and build"
symbol: "vault.build.json"
kind: "file"
start_line: 1
end_line: 25
---

# vault.build.json

- Fragment ID: `rgbdns-frag-befc0b109280`
- Source file: [[DNS from First Principles/Code/vault.build.json.source|vault.build.json]]
- Lines: 1-25
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-befc0b109280", "codeNote": "DNS from First Principles/Code/vault.build.json.source", "sourcePath": "vault.build.json", "startLine": 1, "endLine": 25}
```

## Excerpt

<span id="rgbdns-frag-befc0b109280" class="rgbdns-fragment-target"></span>
### rgbdns-frag-befc0b109280: file vault.build.json

```json
{
  "$schema": "https://firstpair.org/schemas/vault.build.schema.json",
  "schemaVersion": 1,
  "slug": "rgbdns",
  "title": "DNS from First Principles",
  "profile": "code",
  "sourceCommit": "HEAD",
  "plugin": true,
  "nativeDriver": {
    "build": ["python3", "scripts/build-obsidian-vault.py", "--output", "{output}"],
    "validate": ["python3", "scripts/check-obsidian-vault.py", "{output}"]
  },
  "guide": { "bookSpecific": "docs/OBSIDIAN-VAULT.md" },
  "reader": [
    { "id": "book", "title": "DNS from First Principles", "source": "docs/book/rgbdns.md", "preview": true }
  ],
  "products": {
    "desktop": {
      "output": "obsidian-vault/RGBDNS Book Candidate",
      "edition": "full",
      "maxFiles": 20000,
      "maxBytes": 500000000
    }
  }
}
```
