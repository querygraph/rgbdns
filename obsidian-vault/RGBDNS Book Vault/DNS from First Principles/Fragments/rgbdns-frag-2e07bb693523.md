---
type: "code-fragment"
fragment_id: "rgbdns-frag-2e07bb693523"
source_path: "packaging/tests/test-migrate-zone-state.sh"
code_note: "DNS from First Principles/Code/packaging/tests/test-migrate-zone-state.sh.source"
language: "bash"
subsystem: "Repository and build"
symbol: "test-migrate-zone-state.sh"
kind: "file"
start_line: 1
end_line: 25
---

# test-migrate-zone-state.sh

- Fragment ID: `rgbdns-frag-2e07bb693523`
- Source file: [[DNS from First Principles/Code/packaging/tests/test-migrate-zone-state.sh.source|packaging/tests/test-migrate-zone-state.sh]]
- Lines: 1-25
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-2e07bb693523", "codeNote": "DNS from First Principles/Code/packaging/tests/test-migrate-zone-state.sh.source", "sourcePath": "packaging/tests/test-migrate-zone-state.sh", "startLine": 1, "endLine": 25}
```

## Excerpt

<span id="rgbdns-frag-2e07bb693523" class="rgbdns-fragment-target"></span>
### rgbdns-frag-2e07bb693523: file test-migrate-zone-state.sh

```bash
#!/bin/sh
set -eu

repo=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT HUP INT TERM

legacy=$test_dir/etc/rgbdns/zones
zones_file=$test_dir/var/lib/rgbdns/zones
owner=$(id -un)
group=$(id -gn)
mkdir -p "${legacy%/*}"
printf 'fieldnotes.es\nfoto.gs\n' >"$legacy"

RGBDNS_LEGACY_ZONES_FILE=$legacy \
RGBDNS_ZONES_FILE=$zones_file \
RGBDNS_INSTALL_OWNER=$owner:$group \
RGBDNS_STATE_OWNER=$owner:$group \
    "$repo/packaging/scripts/migrate-zone-state"

test ! -e "$legacy"
test "$(cat "$zones_file")" = "$(printf 'fieldnotes.es\nfoto.gs')"
test "$(stat -c %U:%G "$zones_file")" = "$owner:$group"

echo "test-migrate-zone-state: passed"
```
