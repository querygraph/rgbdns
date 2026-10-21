---
type: "code-file"
source_path: "vault.build.json"
language: "json"
subsystem: "Repository and build"
line_count: 25
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# vault.build.json

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `vault.build.json`
- Lines: 25
- Summary: Source file in the Repository and build subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-befc0b109280|vault.build.json]]: lines 1-25

## Full Source

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
