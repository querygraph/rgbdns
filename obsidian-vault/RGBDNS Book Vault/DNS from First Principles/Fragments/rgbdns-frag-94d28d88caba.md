---
type: "code-fragment"
fragment_id: "rgbdns-frag-94d28d88caba"
source_path: "docs/blog/rgbdns-dnssec/post.md"
code_note: "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "The private key stops at the primary"
kind: "heading"
start_line: 51
end_line: 93
---

# The private key stops at the primary

- Fragment ID: `rgbdns-frag-94d28d88caba`
- Source file: [[DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source|docs/blog/rgbdns-dnssec/post.md]]
- Lines: 51-93
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-94d28d88caba", "codeNote": "DNS from First Principles/Code/docs/blog/rgbdns-dnssec/post.md.source", "sourcePath": "docs/blog/rgbdns-dnssec/post.md", "startLine": 51, "endLine": 93}
```

## Excerpt

<span id="rgbdns-frag-94d28d88caba" class="rgbdns-fragment-target"></span>
### rgbdns-frag-94d28d88caba: heading The private key stops at the primary

````markdown
## The private key stops at the primary

Key creation is an explicit root operation:

```sh
sudo install -d -o root -g root -m 0700 /etc/rgbdns/keys
sudo rgbsec-keygen example.com \
  /etc/rgbdns/keys/example.com.pk8 \
  | sudo tee /etc/rgbdns/dnssec
sudo chown root:rgbdns /etc/rgbdns/dnssec
sudo chmod 0640 /etc/rgbdns/dnssec
```

The key is PKCS#8 DER, mode 0600, and backed up separately. The line printed on
stdout is public policy, not secret material. It records the zone, absolute key
path, algorithm, signature lifetime, refresh window, and inception skew.

The publication graph stays inspectable:

```text
source -> ACME overlay -> selected ANAME materialization
       -> rgbsec-data -> rgbsec-check -> atomic activation
```

Every stage writes a sibling temporary file and renames only after success. A
failed lookup, signature, compile, reload, or verification leaves the previous
CDB active. `tinydns` only serves records from that finished CDB.

The packaged setup enables a privileged publisher every twelve hours and an
unprivileged checker every hour:

```sh
sudo rgbdns-setup primary \
  --data /srv/dns/rgbdns.data \
  --listen-ip 0.0.0.0 \
  --allow-nets 10.0.2.10/32 \
  --dnssec-policy /etc/rgbdns/dnssec
```

The root-only publisher reads the key. The `rgbdns` checker reads the active
public snapshot. That separation matters more than whether the commands happen
to share a Rust crate.

````
