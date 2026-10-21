---
type: "code-file"
source_path: "packaging/default/query-report.env"
language: ""
subsystem: "Repository and build"
line_count: 5
fragment_count: 1
rgbdns_commit: "5ae24772"
---

# packaging/default/query-report.env

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `packaging/default/query-report.env`
- Lines: 5
- Summary: Daily rgbdns query report delivery. Configure REPORT_TO before enabling

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-7c11d8addfea|query-report.env]]: lines 1-5

## Full Source

```
# Daily rgbdns query report delivery. Configure REPORT_TO before enabling
# rgbdns-query-report.timer.
REPORT_TO=
REPORT_FROM=
REPORT_SENDMAIL=/usr/sbin/sendmail
```
