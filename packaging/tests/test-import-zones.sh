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
