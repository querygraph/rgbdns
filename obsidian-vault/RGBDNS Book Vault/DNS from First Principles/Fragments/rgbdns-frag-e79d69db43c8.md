---
type: "code-fragment"
fragment_id: "rgbdns-frag-e79d69db43c8"
source_path: "packaging/tests/test-migrate-zone-drop.sh"
code_note: "DNS from First Principles/Code/packaging/tests/test-migrate-zone-drop.sh.source"
language: "bash"
subsystem: "Repository and build"
symbol: "test-migrate-zone-drop.sh"
kind: "file"
start_line: 1
end_line: 40
---

# test-migrate-zone-drop.sh

- Fragment ID: `rgbdns-frag-e79d69db43c8`
- Source file: [[DNS from First Principles/Code/packaging/tests/test-migrate-zone-drop.sh.source|packaging/tests/test-migrate-zone-drop.sh]]
- Lines: 1-40
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-e79d69db43c8", "codeNote": "DNS from First Principles/Code/packaging/tests/test-migrate-zone-drop.sh.source", "sourcePath": "packaging/tests/test-migrate-zone-drop.sh", "startLine": 1, "endLine": 40}
```

## Excerpt

<span id="rgbdns-frag-e79d69db43c8" class="rgbdns-fragment-target"></span>
### rgbdns-frag-e79d69db43c8: file test-migrate-zone-drop.sh

```bash
#!/bin/sh
set -eu

repo=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT HUP INT TERM

config=$test_dir/etc/rgbdns
state=$test_dir/var/lib/rgbdns
systemd=$test_dir/etc/systemd/system
home=$test_dir/home/operator
mkdir -p "$config" "$home"
owner=$(id -un)
group=$(id -gn)
printf 'fieldnotes.es\nfoto.gs\n' >"$home/rgbdns.zones"
cat >"$config/zones-drop.env" <<EOF
ZONES_DROP=$home/rgbdns.zones
ZONES_DROP_OWNER=$owner
EOF

RGBDNS_CONFIG_DIR=$config \
RGBDNS_STATE_DIR=$state \
RGBDNS_SYSTEMD_DIR=$systemd \
RGBDNS_GROUP=$group \
RGBDNS_CONFIG_OWNER=$owner:$group \
RGBDNS_HOME_ROOT=$test_dir/home \
RGBDNS_IMPORT_ZONES=$repo/packaging/scripts/import-zones \
    "$repo/packaging/scripts/migrate-zone-drop"

managed=$state/incoming/rgbdns.zones
test "$(cat "$managed")" = "$(printf 'fieldnotes.es\nfoto.gs')"
test "$(cat "$state/tinydns/zones")" = "$(printf 'fieldnotes.es\nfoto.gs')"
grep -qx "ZONES_DROP=$managed" "$config/zones-drop.env"
grep -qx "ZONES_DROP_OWNER=$owner" "$config/zones-drop.env"
grep -qx "PathChanged=$managed" \
    "$systemd/rgbdns-zones.path.d/path.conf"
test "$(stat -c %U "$managed")" = "$owner"
test "$(stat -c %G "$managed")" = "$group"

echo "test-migrate-zone-drop: passed"
```
