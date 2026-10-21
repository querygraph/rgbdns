---
type: "code-file"
source_path: "packaging/tests/test-migrate-zone-state.sh"
language: "bash"
subsystem: "Repository and build"
line_count: 25
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# packaging/tests/test-migrate-zone-state.sh

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `packaging/tests/test-migrate-zone-state.sh`
- Lines: 25
- Summary: Source file in the Repository and build subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-2e07bb693523|test-migrate-zone-state.sh]]: lines 1-25

## Full Source

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
