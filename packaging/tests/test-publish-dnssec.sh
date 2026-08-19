#!/bin/sh
set -eu

test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT HUP INT TERM
mkdir -p "$test_dir/bin" "$test_dir/state"
printf '%s\n' 'unsigned source' >"$test_dir/state/data"
printf '%s\n' 'Kexample:/key:13:1209600:86400:3600' >"$test_dir/dnssec"
printf '%s\n' 'last good' >"$test_dir/state/data.cdb"

cat >"$test_dir/bin/aname" <<'EOF'
#!/bin/sh
test "$#" -eq 3
test -r "$3"
cp "$1" "$2"
EOF
cat >"$test_dir/bin/dnssec-data" <<'EOF'
#!/bin/sh
cp "$1" "$2"
printf '%s\n' signed >>"$2"
EOF
cat >"$test_dir/bin/check" <<'EOF'
#!/bin/sh
grep -q signed "$1"
EOF
cat >"$test_dir/bin/install" <<'EOF'
#!/bin/sh
while [ $# -gt 2 ]; do shift; done
cp "$1" "$2"
EOF
cat >"$test_dir/bin/sync" <<'EOF'
#!/bin/sh
exit 0
EOF
chmod +x "$test_dir/bin/"*

run_publish() {
    DNSSEC_SOURCE=$test_dir/state/data \
    DNSSEC_POLICY=$test_dir/dnssec \
    DNSSEC_OUTPUT=$test_dir/state/data.cdb \
    RGBDNS_ANAME_MATERIALIZE=$test_dir/bin/aname \
    RGBDNS_DNSSEC_DATA=$test_dir/bin/dnssec-data \
    RGBDNS_DNSSEC_CHECK=$test_dir/bin/check \
    RGBDNS_INSTALL=$test_dir/bin/install \
    RGBDNS_SYNC=$test_dir/bin/sync \
        packaging/scripts/publish-dnssec "$test_dir/state"
}

run_publish
grep -q signed "$test_dir/state/data.cdb"

printf '%s\n' 'last good again' >"$test_dir/state/data.cdb"
cat >"$test_dir/bin/check" <<'EOF'
#!/bin/sh
exit 1
EOF
chmod +x "$test_dir/bin/check"
if run_publish; then
    echo 'test-publish-dnssec: expected verification failure' >&2
    exit 1
fi
test "$(cat "$test_dir/state/data.cdb")" = 'last good again'
