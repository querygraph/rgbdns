---
type: "code-fragment"
fragment_id: "rgbdns-frag-3212e074767a"
source_path: "packaging/cargo-binaries.py"
code_note: "DNS from First Principles/Code/packaging/cargo-binaries.py.source"
language: "python"
subsystem: "Repository and build"
symbol: "cargo-binaries.py"
kind: "file"
start_line: 1
end_line: 17
---

# cargo-binaries.py

- Fragment ID: `rgbdns-frag-3212e074767a`
- Source file: [[DNS from First Principles/Code/packaging/cargo-binaries.py.source|packaging/cargo-binaries.py]]
- Lines: 1-17
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-3212e074767a", "codeNote": "DNS from First Principles/Code/packaging/cargo-binaries.py.source", "heading": "rgbdns-frag-3212e074767a: file cargo-binaries.py", "sourcePath": "packaging/cargo-binaries.py", "startLine": 1, "endLine": 17}
```

## Excerpt

<span id="rgbdns-frag-3212e074767a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3212e074767a: file cargo-binaries.py

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
