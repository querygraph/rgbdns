#!/bin/sh
set -eu

test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT HUP INT TERM
env_file=$test_dir/secondary.env
zones_file=$test_dir/zones

cat >"$env_file" <<'EOF'
ZONES=fieldnotes.es example.net
PRIMARY=192.0.2.1
EOF

RGBDNS_SECONDARY_ENV=$env_file \
RGBDNS_ZONES_FILE=$zones_file \
RGBDNS_IMPORT_ZONES="$PWD/packaging/scripts/import-zones" \
RGBDNS_INSTALL_OWNER='' \
    packaging/scripts/migrate-zones

test "$(cat "$zones_file")" = "$(printf 'fieldnotes.es\nexample.net')"
test "$(cat "$env_file")" = 'PRIMARY=192.0.2.1'

# The migration is idempotent and does not replace canonical state.
printf 'replacement.invalid\n' >>"$env_file"
RGBDNS_SECONDARY_ENV=$env_file \
RGBDNS_ZONES_FILE=$zones_file \
RGBDNS_IMPORT_ZONES="$PWD/packaging/scripts/import-zones" \
RGBDNS_INSTALL_OWNER='' \
    packaging/scripts/migrate-zones
test "$(cat "$zones_file")" = "$(printf 'fieldnotes.es\nexample.net')"

echo "test-migrate-zones: passed"
