---
type: "code-fragment"
fragment_id: "rgbdns-frag-f3184d924ae0"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "TSIG profile"
kind: "heading"
start_line: 139
end_line: 158
---

# TSIG profile

- Fragment ID: `rgbdns-frag-f3184d924ae0`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 139-158
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-f3184d924ae0", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 139, "endLine": 158}
```

## Excerpt

<span id="rgbdns-frag-f3184d924ae0" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f3184d924ae0: heading TSIG profile

```markdown
## TSIG profile

The request must contain exactly one TSIG RR as the last Additional record.
TSIG is decoded without interpreting its embedded names as ordinary opaque
RDATA. The server verifies:

- configured key name and HMAC-SHA256 algorithm;
- original ID equal to the DNS message ID;
- signed time within the request fudge, with a server maximum fudge of 300
  seconds;
- MAC over the exact RFC 2845 request input using constant-time comparison;
- MAC length of 32 bytes;
- no trailing or duplicate TSIG.

Successful and DNS-error responses are TSIG-signed using the request MAC.
Authentication failures use NOTAUTH with TSIG error BADKEY, BADSIG, or BADTIME
when a safe signed response is possible. Malformed messages receive FORMERR.
Secrets are never logged. Logs identify only the key name, zone, owner, action,
result, and client address.

```
