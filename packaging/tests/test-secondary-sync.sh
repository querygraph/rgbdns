#!/bin/sh
set -eu

test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT HUP INT TERM
mkdir -p "$test_dir/state" "$test_dir/bin"

cat >"$test_dir/bin/axfr-get" <<'EOF'
#!/bin/sh
case $1 in
    fail.example) exit 1 ;;
esac
printf 'Z%s:.\\\n' "$1" >"$3"
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
        packaging/scripts/secondary-sync
}

run_sync 'fieldnotes.es example.net'
grep -q '^Zfieldnotes.es:' "$test_dir/state/data"
grep -q '^Zexample.net:' "$test_dir/state/data"
test "$(cat "$test_dir/state/data")" = "$(cat "$test_dir/state/data.cdb")"

cp "$test_dir/state/data.cdb" "$test_dir/expected.cdb"
if run_sync 'fieldnotes.es fail.example'; then
    echo "test-secondary-sync: an AXFR failure unexpectedly succeeded" >&2
    exit 1
fi
test "$(cat "$test_dir/expected.cdb")" = \
    "$(cat "$test_dir/state/data.cdb")"

echo "test-secondary-sync: passed"
