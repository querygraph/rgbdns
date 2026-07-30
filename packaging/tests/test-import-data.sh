#!/bin/sh
set -eu

test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT HUP INT TERM
mkdir -p "$test_dir/state" "$test_dir/bin"
drop=$test_dir/rgbdns.data
owner=$(id -un)

cat >"$test_dir/bin/compile-zone" <<'EOF'
#!/bin/sh
if grep -q '^INVALID$' "$RGBDNS_STATE_DIR/data"; then
    exit 1
fi
cp "$RGBDNS_STATE_DIR/data" "$RGBDNS_STATE_DIR/data.cdb"
EOF
chmod +x "$test_dir/bin/compile-zone"

printf '.example.net:192.0.2.1:a\n' >"$drop"
DATA_DROP=$drop \
DATA_DROP_OWNER=$owner \
RGBDNS_STATE_DIR=$test_dir/state \
RGBDNS_COMPILE_ZONE=$test_dir/bin/compile-zone \
RGBDNS_SERVICE_USER='' \
    packaging/scripts/import-data
test "$(cat "$test_dir/state/data")" = '.example.net:192.0.2.1:a'
test "$(cat "$test_dir/state/data.cdb")" = '.example.net:192.0.2.1:a'

printf 'INVALID\n' >"$drop"
if DATA_DROP=$drop \
    DATA_DROP_OWNER=$owner \
    RGBDNS_STATE_DIR=$test_dir/state \
    RGBDNS_COMPILE_ZONE=$test_dir/bin/compile-zone \
    RGBDNS_SERVICE_USER='' \
        packaging/scripts/import-data
then
    echo "test-import-data: invalid data unexpectedly succeeded" >&2
    exit 1
fi
test "$(cat "$test_dir/state/data")" = '.example.net:192.0.2.1:a'
test "$(cat "$test_dir/state/data.cdb")" = '.example.net:192.0.2.1:a'

rm -f "$drop"
ln -s "$test_dir/state/data" "$drop"
if DATA_DROP=$drop \
    DATA_DROP_OWNER=$owner \
    RGBDNS_STATE_DIR=$test_dir/state \
    RGBDNS_COMPILE_ZONE=$test_dir/bin/compile-zone \
    RGBDNS_SERVICE_USER='' \
        packaging/scripts/import-data
then
    echo "test-import-data: symlink unexpectedly succeeded" >&2
    exit 1
fi

echo "test-import-data: passed"
