---
type: "code-fragment"
fragment_id: "rgbdns-frag-3601a76284aa"
source_path: "packaging/tests/test-restore-role-units.sh"
code_note: "DNS from First Principles/Code/packaging/tests/test-restore-role-units.sh.source"
language: "bash"
subsystem: "Repository and build"
symbol: "test-restore-role-units.sh"
kind: "file"
start_line: 1
end_line: 67
---

# test-restore-role-units.sh

- Fragment ID: `rgbdns-frag-3601a76284aa`
- Source file: [[DNS from First Principles/Code/packaging/tests/test-restore-role-units.sh.source|packaging/tests/test-restore-role-units.sh]]
- Lines: 1-67
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-3601a76284aa", "codeNote": "DNS from First Principles/Code/packaging/tests/test-restore-role-units.sh.source", "sourcePath": "packaging/tests/test-restore-role-units.sh", "startLine": 1, "endLine": 67}
```

## Excerpt

<span id="rgbdns-frag-3601a76284aa" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3601a76284aa: file test-restore-role-units.sh

```bash
#!/bin/sh
set -eu

root=$(mktemp -d)
trap 'rm -rf "$root"' EXIT HUP INT TERM
mkdir -p "$root/bin" "$root/config" "$root/run/systemd/system"
log=$root/systemctl.log

cat >"$root/bin/systemctl" <<'EOF'
#!/bin/sh
printf '%s\n' "$*" >>"$RGBDNS_TEST_LOG"
EOF
chmod +x "$root/bin/systemctl"

run_restore() {
    : >"$log"
    RGBDNS_SYSTEMCTL=$root/bin/systemctl \
    RGBDNS_CONFIG_DIR=$root/config \
    RGBDNS_SYSTEMD_RUNTIME=$root/run/systemd/system \
    RGBDNS_TEST_LOG=$log \
        packaging/scripts/restore-role-units
}

touch "$root/config/data-drop.env"
run_restore
grep -qx 'daemon-reload' "$log"
grep -qx 'disable --now rgbdns-secondary-sync.timer rgbdns-zones.path' "$log"
grep -qx 'enable --now rgbdns-tinydns.service' "$log"
grep -qx 'enable --now rgbdns-data.path' "$log"
! grep -q 'enable --now rgbdns-secondary-sync.timer' "$log"

touch "$root/config/dnssec.env"
run_restore
grep -qx 'enable --now rgbdns-dnssec-publish.timer' "$log"
grep -qx 'enable --now rgbdns-dnssec-check.timer' "$log"
rm "$root/config/dnssec.env"

rm "$root/config/data-drop.env"
touch "$root/config/secondary.env" "$root/config/zones-drop.env"
run_restore
grep -qx 'daemon-reload' "$log"
grep -qx 'disable --now rgbdns-data.path' "$log"
grep -qx 'disable --now rgbdns-dnssec-publish.timer rgbdns-dnssec-check.timer' "$log"
grep -qx 'enable --now rgbdns-tinydns.service' "$log"
grep -qx 'enable --now rgbdns-secondary-sync.timer' "$log"
grep -qx 'enable --now rgbdns-zones.path' "$log"
! grep -q 'enable --now rgbdns-data.path' "$log"

rm "$root/config/zones-drop.env"
run_restore
grep -qx 'enable --now rgbdns-secondary-sync.timer' "$log"
! grep -q 'enable --now rgbdns-zones.path' "$log"

rm "$root/config/secondary.env"
run_restore
grep -qx 'daemon-reload' "$log"
test "$(wc -l <"$log")" -eq 1
! grep -q 'rgbdns-tinydns.service' "$log"

rm -rf "$root/run/systemd/system"
: >"$log"
RGBDNS_SYSTEMCTL=$root/bin/systemctl \
RGBDNS_CONFIG_DIR=$root/config \
RGBDNS_SYSTEMD_RUNTIME=$root/run/systemd/system \
RGBDNS_TEST_LOG=$log \
    packaging/scripts/restore-role-units
test ! -s "$log"
```
