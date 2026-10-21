---
type: "code-fragment"
fragment_id: "rgbdns-frag-8b6ce215d317"
source_path: "RESEND_SENDMAIL.md"
code_note: "DNS from First Principles/Code/RESEND_SENDMAIL.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Choosing between the two"
kind: "heading"
start_line: 684
end_line: 706
---

# Choosing between the two

- Fragment ID: `rgbdns-frag-8b6ce215d317`
- Source file: [[DNS from First Principles/Code/RESEND_SENDMAIL.md.source|RESEND_SENDMAIL.md]]
- Lines: 684-706
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-8b6ce215d317", "codeNote": "DNS from First Principles/Code/RESEND_SENDMAIL.md.source", "sourcePath": "RESEND_SENDMAIL.md", "startLine": 684, "endLine": 706}
```

## Excerpt

<span id="rgbdns-frag-8b6ce215d317" class="rgbdns-fragment-target"></span>
### rgbdns-frag-8b6ce215d317: heading Choosing between the two

````markdown
## Choosing between the two

| Situation | Recommended transport |
|---|---|
| Only rgbdns needs outbound SMTP submission | `msmtp` |
| Existing services already use `/usr/sbin/sendmail` | Native Sendmail |
| You want the smallest configuration and no MTA daemon | `msmtp` |
| You need a local mail queue and conventional MTA behavior | Native Sendmail |
| You want the shortest troubleshooting path | `msmtp` |
| You already operate Sendmail and understand its macros/SASL | Native Sendmail |

Do not configure both transports for the report at once. The active transport
is determined solely by `REPORT_SENDMAIL`:

```sh
sudo grep '^REPORT_SENDMAIL=' /etc/rgbdns/query-report.env
sudo readlink -f "$(sudo awk -F= '/^REPORT_SENDMAIL=/{print $2}' /etc/rgbdns/query-report.env)"
```

`/usr/bin/msmtp` means the report uses msmtp. `/usr/sbin/sendmail` resolving
to the Sendmail executable and logs containing `sm-mta` mean it uses native
Sendmail.

````
