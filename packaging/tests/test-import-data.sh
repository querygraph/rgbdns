#!/bin/sh
set -eu

test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT HUP INT TERM
mkdir -p "$test_dir/state" "$test_dir/bin"
drop=$test_dir/rgbdns.data
owner=$(id -un)

# Keep the test independent of the host's /etc/rgbdns.
RGBDNS_DNSSEC_ENV=$test_dir/absent-dnssec.env
RGBDNS_SERVICE_USER=
export RGBDNS_DNSSEC_ENV RGBDNS_SERVICE_USER
unset DATA_DROP_IMPORT 2>/dev/null || true

fail() {
    echo "test-import-data: $*" >&2
    exit 1
}

cat >"$test_dir/bin/compile-zone" <<'STUB'
#!/bin/sh
if grep -q '^INVALID$' "$RGBDNS_STATE_DIR/data"; then
    exit 1
fi
cp "$RGBDNS_STATE_DIR/data" "$RGBDNS_STATE_DIR/data.cdb"
STUB
chmod +x "$test_dir/bin/compile-zone"

# --- Unsigned primary: compile-zone output replaces data and data.cdb.
printf '.example.net:192.0.2.1:a\n' >"$drop"
DATA_DROP=$drop \
DATA_DROP_OWNER=$owner \
RGBDNS_STATE_DIR=$test_dir/state \
RGBDNS_COMPILE_ZONE=$test_dir/bin/compile-zone \
    packaging/scripts/import-data
test "$(cat "$test_dir/state/data")" = '.example.net:192.0.2.1:a'
test "$(cat "$test_dir/state/data.cdb")" = '.example.net:192.0.2.1:a'

printf 'INVALID\n' >"$drop"
if DATA_DROP=$drop \
    DATA_DROP_OWNER=$owner \
    RGBDNS_STATE_DIR=$test_dir/state \
    RGBDNS_COMPILE_ZONE=$test_dir/bin/compile-zone \
        packaging/scripts/import-data
then
    fail "invalid data unexpectedly succeeded"
fi
test "$(cat "$test_dir/state/data")" = '.example.net:192.0.2.1:a'
test "$(cat "$test_dir/state/data.cdb")" = '.example.net:192.0.2.1:a'

rm -f "$drop"
ln -s "$test_dir/state/data" "$drop"
if DATA_DROP=$drop \
    DATA_DROP_OWNER=$owner \
    RGBDNS_STATE_DIR=$test_dir/state \
    RGBDNS_COMPILE_ZONE=$test_dir/bin/compile-zone \
        packaging/scripts/import-data
then
    fail "symlink unexpectedly succeeded"
fi
rm -f "$drop"

# --- DNSSEC primary: compile-zone only verifies the signed CDB, so it must not
# be used; the upload is validated with tinydns-data in the stage, only data is
# replaced, and the published signed data.cdb is left for rgbdns-dnssec-publish.
cat >"$test_dir/bin/compile-zone-verify-only" <<'STUB'
#!/bin/sh
echo "compile-zone must not validate uploads on a DNSSEC primary" >&2
exit 99
STUB
cat >"$test_dir/bin/tinydns-data" <<'STUB'
#!/bin/sh
test -r data || exit 111
if grep -q '^INVALID$' data; then
    exit 111
fi
cp data data.cdb
STUB
chmod +x "$test_dir/bin/compile-zone-verify-only" "$test_dir/bin/tinydns-data"
: >"$test_dir/dnssec.env"
signed=$test_dir/signed
mkdir -p "$signed"
printf '.example.net:192.0.2.1:a\n' >"$signed/data"
printf 'SIGNED\n' >"$signed/data.cdb"

import_signed() {
    DATA_DROP=$drop \
    DATA_DROP_OWNER=$owner \
    RGBDNS_STATE_DIR=$signed \
    RGBDNS_COMPILE_ZONE=$test_dir/bin/compile-zone-verify-only \
    RGBDNS_DNSSEC_ENV=$test_dir/dnssec.env \
    RGBDNS_TINYDNS_DATA=$test_dir/bin/tinydns-data \
        packaging/scripts/import-data
}

printf '.example.net:192.0.2.2:a\n' >"$drop"
import_signed || fail "valid upload failed on a DNSSEC primary"
test "$(cat "$signed/data")" = '.example.net:192.0.2.2:a' ||
    fail "DNSSEC source was not replaced"
test "$(cat "$signed/data.cdb")" = 'SIGNED' ||
    fail "an unsigned database replaced the signed data.cdb"
leftovers=$(find "$signed" -mindepth 1 -name '.primary-import.*')
[ -z "$leftovers" ] || fail "stage left behind: $leftovers"

printf 'INVALID\n' >"$drop"
if import_signed; then
    fail "invalid upload unexpectedly succeeded on a DNSSEC primary"
fi
test "$(cat "$signed/data")" = '.example.net:192.0.2.2:a' ||
    fail "invalid DNSSEC upload replaced the source"
test "$(cat "$signed/data.cdb")" = 'SIGNED'

# --- DATA_DROP_IMPORT flag (ExecCondition and direct runs).
check_enabled() {
    set +e
    env "$@" packaging/scripts/import-data --check-enabled >/dev/null 2>&1
    status=$?
    set -e
    echo "$status"
}
[ "$(check_enabled DATA_DROP_IMPORT=)" = 0 ] || fail "empty flag should enable"
[ "$(check_enabled DATA_DROP_IMPORT=enabled)" = 0 ] || fail "enabled should enable"
[ "$(check_enabled DATA_DROP_IMPORT=disabled)" = 1 ] || fail "disabled should skip with 1"
[ "$(check_enabled DATA_DROP_IMPORT=bogus)" = 255 ] || fail "invalid flag should fail with 255"

printf '.example.net:192.0.2.3:a\n' >"$drop"
DATA_DROP_IMPORT=disabled \
DATA_DROP=$drop \
DATA_DROP_OWNER=$owner \
RGBDNS_STATE_DIR=$test_dir/state \
RGBDNS_COMPILE_ZONE=$test_dir/bin/compile-zone \
    packaging/scripts/import-data >/dev/null || fail "disabled direct run should exit 0"
test "$(cat "$test_dir/state/data")" = '.example.net:192.0.2.1:a' ||
    fail "disabled import changed the state"

echo "test-import-data: passed"
