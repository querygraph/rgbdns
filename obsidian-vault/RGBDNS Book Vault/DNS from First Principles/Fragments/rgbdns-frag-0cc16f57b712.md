---
type: "code-fragment"
fragment_id: "rgbdns-frag-0cc16f57b712"
source_path: "packaging/build-deb.sh"
code_note: "DNS from First Principles/Code/packaging/build-deb.sh.source"
language: "bash"
subsystem: "Repository and build"
symbol: "build-deb.sh"
kind: "file"
start_line: 1
end_line: 13
---

# build-deb.sh

- Fragment ID: `rgbdns-frag-0cc16f57b712`
- Source file: [[DNS from First Principles/Code/packaging/build-deb.sh.source|packaging/build-deb.sh]]
- Lines: 1-13
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-0cc16f57b712", "codeNote": "DNS from First Principles/Code/packaging/build-deb.sh.source", "heading": "rgbdns-frag-0cc16f57b712: file build-deb.sh", "sourcePath": "packaging/build-deb.sh", "startLine": 1, "endLine": 13}
```

## Excerpt

<span id="rgbdns-frag-0cc16f57b712" class="rgbdns-fragment-target"></span>
### rgbdns-frag-0cc16f57b712: file build-deb.sh

```bash
#!/bin/sh
set -eu

repo=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$repo"

case $(uname -o 2>/dev/null || uname -s) in
    Android) echo "build-deb: run on Debian/Ubuntu, not Android/Termux" >&2; exit 1 ;;
esac
command -v dpkg-buildpackage >/dev/null 2>&1 ||
    { echo "build-deb: install build-essential, cargo, rustc, and debhelper" >&2; exit 1; }

dpkg-buildpackage --build=binary --no-sign "$@"
```
