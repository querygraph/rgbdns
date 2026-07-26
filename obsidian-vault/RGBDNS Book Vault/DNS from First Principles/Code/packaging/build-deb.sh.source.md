---
type: "code-file"
source_path: "packaging/build-deb.sh"
language: "bash"
subsystem: "Repository and build"
line_count: 13
fragment_count: 1
rgbdns_commit: "79502939"
---

# packaging/build-deb.sh

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `packaging/build-deb.sh`
- Lines: 13
- Summary: Source file in the Repository and build subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-0cc16f57b712|build-deb.sh]]: lines 1-13

## Full Source

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
