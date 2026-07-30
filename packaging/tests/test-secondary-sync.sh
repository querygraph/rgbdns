#!/bin/sh
set -eu

test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT HUP INT TERM
mkdir -p "$test_dir/state" "$test_dir/bin"

cat >"$test_dir/bin/axfr-get" <<'EOF'
#!/bin/sh
[ "$1" != "${FAIL_ZONE:-}" ] || exit 1
printf 'Z%s:.\\%s\n' "$1" "${GENERATION:-1}" >"$3"
: >"$4"
EOF

cat >"$test_dir/bin/tinydns-data" <<'EOF'
#!/bin/sh
cp data data.cdb
EOF

cat >"$test_dir/bin/flock" <<'EOF'
#!/bin/sh
exit 0
EOF

chmod +x "$test_dir/bin/axfr-get" "$test_dir/bin/tinydns-data" \
    "$test_dir/bin/flock"

run_sync() {
    ZONES=$1 \
    PRIMARY=192.0.2.1 \
    RGBDNS_STATE_DIR="$test_dir/state" \
    RGBDNS_LOCK_FILE="$test_dir/secondary.lock" \
    RGBDNS_AXFR_GET="$test_dir/bin/axfr-get" \
    RGBDNS_TINYDNS_DATA="$test_dir/bin/tinydns-data" \
    RGBDNS_FLOCK="$test_dir/bin/flock" \
    FAIL_ZONE="${FAIL_ZONE:-}" \
    GENERATION="${GENERATION:-1}" \
        packaging/scripts/secondary-sync
}

run_sync 'fieldnotes.es example.net'
grep -q '^Zfieldnotes.es:.*1$' "$test_dir/state/data"
grep -q '^Zexample.net:.*1$' "$test_dir/state/data"
test "$(cat "$test_dir/state/data")" = "$(cat "$test_dir/state/data.cdb")"

# A routine failure retains that zone's prior snapshot while other zones
# advance and the combined database is activated.
FAIL_ZONE=example.net
GENERATION=2
export FAIL_ZONE GENERATION
run_sync 'fieldnotes.es example.net'
grep -q '^Zfieldnotes.es:.*2$' "$test_dir/state/data"
grep -q '^Zexample.net:.*1$' "$test_dir/state/data"

# A newly configured zone with no successful snapshot blocks activation.
cp "$test_dir/state/data.cdb" "$test_dir/expected.cdb"
FAIL_ZONE=missing.example
GENERATION=3
export FAIL_ZONE GENERATION
if run_sync 'fieldnotes.es example.net missing.example'; then
    echo "test-secondary-sync: an AXFR failure unexpectedly succeeded" >&2
    exit 1
fi
test "$(cat "$test_dir/expected.cdb")" = \
    "$(cat "$test_dir/state/data.cdb")"

echo "test-secondary-sync: passed"
