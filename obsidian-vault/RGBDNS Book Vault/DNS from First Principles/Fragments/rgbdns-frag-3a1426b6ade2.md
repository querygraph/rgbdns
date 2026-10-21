---
type: "code-fragment"
fragment_id: "rgbdns-frag-3a1426b6ade2"
source_path: "packaging/tests/test-daily-query-report.sh"
code_note: "DNS from First Principles/Code/packaging/tests/test-daily-query-report.sh.source"
language: "bash"
subsystem: "Repository and build"
symbol: "test-daily-query-report.sh"
kind: "file"
start_line: 1
end_line: 56
---

# test-daily-query-report.sh

- Fragment ID: `rgbdns-frag-3a1426b6ade2`
- Source file: [[DNS from First Principles/Code/packaging/tests/test-daily-query-report.sh.source|packaging/tests/test-daily-query-report.sh]]
- Lines: 1-56
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-3a1426b6ade2", "codeNote": "DNS from First Principles/Code/packaging/tests/test-daily-query-report.sh.source", "sourcePath": "packaging/tests/test-daily-query-report.sh", "startLine": 1, "endLine": 56}
```

## Excerpt

<span id="rgbdns-frag-3a1426b6ade2" class="rgbdns-fragment-target"></span>
### rgbdns-frag-3a1426b6ade2: file test-daily-query-report.sh

```bash
#!/bin/sh
set -eu

analyzer=${1:-target/debug/rgbdns-log-report}
case $analyzer in /*) ;; *) analyzer=$(pwd)/$analyzer ;; esac
test -x "$analyzer"

root=$(mktemp -d)
trap 'rm -rf "$root"' EXIT HUP INT TERM
mkdir -p "$root/bin" "$root/state"

cat >"$root/state/data" <<'EOF'
Zexample.com:ns.example.com:hostmaster.example.com:1:2:3:4:5:6
Zquiet.test:ns.quiet.test:hostmaster.quiet.test:1:2:3:4:5:6
EOF
cat >"$root/config" <<EOF
REPORT_TO=deliverable@gmail.com
REPORT_FROM=dns@example.net
REPORT_SENDMAIL=$root/bin/sendmail
EOF
cat >"$root/bin/journalctl" <<'EOF'
#!/bin/sh
cat <<'LOGS'
7f000001:e214:0018 + 0001 example.com
7f000001:e215:0019 + 001c www.example.com
08080808:0035:0020 + 0001 api.example.com
09090909:0035:0021 + 0001 api.example.com
7f000001:e216:0022 - 0001 refused.example.com
starting tinydns
LOGS
EOF
cat >"$root/bin/sendmail" <<EOF
#!/bin/sh
cat >'$root/message'
EOF
chmod +x "$root/bin/journalctl" "$root/bin/sendmail"

RGBDNS_REPORT_CONFIG=$root/config \
RGBDNS_STATE_DIR=$root/state \
RGBDNS_LOG_REPORT=$analyzer \
RGBDNS_JOURNALCTL=$root/bin/journalctl \
RGBDNS_TODAY=2026-08-05 \
RGBDNS_REPORT_DATE=2026-08-04 \
    packaging/scripts/daily-query-report

grep -qx 'To: deliverable@gmail.com' "$root/message"
grep -qx 'From: dns@example.net' "$root/message"
grep -qx 'Subject: rgbdns daily query report for 2026-08-04' "$root/message"
grep -q '^Content-Type: multipart/alternative; boundary="rgbdns-report-' "$root/message"
grep -qx 'Content-Type: text/plain; charset=UTF-8; format=fixed' "$root/message"
grep -q '<pre style="font-family: monospace;">' "$root/message"
grep -Eq '^example\.com +4 +3$' "$root/message"
grep -Eq '^quiet\.test +0 +0$' "$root/message"
grep -q 'Total accepted DNS queries: 4' "$root/message"
test "$(grep -n '^example\.com ' "$root/message" | head -n 1 | cut -d: -f1)" \
    -lt "$(grep -n '^quiet\.test ' "$root/message" | head -n 1 | cut -d: -f1)"
```
