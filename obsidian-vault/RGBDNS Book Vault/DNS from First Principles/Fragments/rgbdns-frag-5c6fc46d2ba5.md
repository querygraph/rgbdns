---
type: "code-fragment"
fragment_id: "rgbdns-frag-5c6fc46d2ba5"
source_path: "packaging/tests/test-import-zones.sh"
code_note: "DNS from First Principles/Code/packaging/tests/test-import-zones.sh.source"
language: "bash"
subsystem: "Repository and build"
symbol: "test-import-zones.sh"
kind: "file"
start_line: 1
end_line: 56
---

# test-import-zones.sh

- Fragment ID: `rgbdns-frag-5c6fc46d2ba5`
- Source file: [[DNS from First Principles/Code/packaging/tests/test-import-zones.sh.source|packaging/tests/test-import-zones.sh]]
- Lines: 1-56
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-5c6fc46d2ba5", "codeNote": "DNS from First Principles/Code/packaging/tests/test-import-zones.sh.source", "sourcePath": "packaging/tests/test-import-zones.sh", "startLine": 1, "endLine": 56}
```

## Excerpt

<span id="rgbdns-frag-5c6fc46d2ba5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5c6fc46d2ba5: file test-import-zones.sh

```bash
#!/bin/sh
set -eu

test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT HUP INT TERM
drop=$test_dir/rgbdns.zones
canonical=$test_dir/etc/zones
owner=$(id -un)
mkdir -p "$test_dir/etc"

cat >"$drop" <<'EOF'
# managed secondary zones
FieldNotes.ES.

example.net
fieldnotes.es
EOF

ZONES_DROP=$drop \
ZONES_DROP_OWNER=$owner \
RGBDNS_ZONES_FILE=$canonical \
RGBDNS_INSTALL_OWNER='' \
    packaging/scripts/import-zones

test "$(cat "$canonical")" = "$(printf 'fieldnotes.es\nexample.net')"
! grep -Fq ': >"$stage"' packaging/scripts/import-zones
! grep -Fq '>>"$stage"' packaging/scripts/import-zones
grep -Fq 'printf '\''%s\n'\'' "$zone" >&8' packaging/scripts/import-zones

cp "$canonical" "$test_dir/expected"
printf 'example.net another.example\n' >"$drop"
if ZONES_DROP=$drop \
    ZONES_DROP_OWNER=$owner \
    RGBDNS_ZONES_FILE=$canonical \
    RGBDNS_INSTALL_OWNER='' \
        packaging/scripts/import-zones
then
    echo "test-import-zones: malformed list unexpectedly succeeded" >&2
    exit 1
fi
test "$(cat "$test_dir/expected")" = "$(cat "$canonical")"

rm -f "$drop"
ln -s "$test_dir/expected" "$drop"
if ZONES_DROP=$drop \
    ZONES_DROP_OWNER=$owner \
    RGBDNS_ZONES_FILE=$canonical \
    RGBDNS_INSTALL_OWNER='' \
        packaging/scripts/import-zones
then
    echo "test-import-zones: symlink unexpectedly succeeded" >&2
    exit 1
fi
test "$(cat "$test_dir/expected")" = "$(cat "$canonical")"

echo "test-import-zones: passed"
```
