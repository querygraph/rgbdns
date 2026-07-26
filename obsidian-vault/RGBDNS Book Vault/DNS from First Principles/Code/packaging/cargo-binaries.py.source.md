---
type: "code-file"
source_path: "packaging/cargo-binaries.py"
language: "python"
subsystem: "Repository and build"
line_count: 17
fragment_count: 1
rgbdns_commit: "79502939"
---

# packaging/cargo-binaries.py

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `packaging/cargo-binaries.py`
- Lines: 17
- Summary: Print binary target names from `cargo metadata` JSON, one per line.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-3212e074767a|cargo-binaries.py]]: lines 1-17

## Full Source

```python
#!/usr/bin/env python3
"""Print binary target names from `cargo metadata` JSON, one per line."""

import json
import sys


metadata = json.load(sys.stdin)
package = next(package for package in metadata["packages"] if package["name"] == "rgbdns")
names = {
    target["name"]
    for target in package["targets"]
    if "bin" in target["kind"]
}
for name in sorted(names):
    print(name)

```
