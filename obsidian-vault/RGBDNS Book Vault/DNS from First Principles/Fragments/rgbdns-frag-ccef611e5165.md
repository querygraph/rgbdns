---
type: "code-fragment"
fragment_id: "rgbdns-frag-ccef611e5165"
source_path: "packaging/tests/test-import-data.sh"
code_note: "DNS from First Principles/Code/packaging/tests/test-import-data.sh.source"
language: "bash"
subsystem: "Repository and build"
symbol: "test-import-data.sh"
kind: "file"
start_line: 1
end_line: 81
---

# test-import-data.sh

- Fragment ID: `rgbdns-frag-ccef611e5165`
- Source file: [[DNS from First Principles/Code/packaging/tests/test-import-data.sh.source|packaging/tests/test-import-data.sh]]
- Lines: 1-81
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-ccef611e5165", "codeNote": "DNS from First Principles/Code/packaging/tests/test-import-data.sh.source", "sourcePath": "packaging/tests/test-import-data.sh", "startLine": 1, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-ccef611e5165" class="rgbdns-fragment-target"></span>
### rgbdns-frag-ccef611e5165: file test-import-data.sh

```bash
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
```
