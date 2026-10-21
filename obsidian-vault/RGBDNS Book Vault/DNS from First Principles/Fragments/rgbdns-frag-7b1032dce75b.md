---
type: "code-fragment"
fragment_id: "rgbdns-frag-7b1032dce75b"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "18. Troubleshoot by boundary"
kind: "heading"
start_line: 886
end_line: 956
---

# 18. Troubleshoot by boundary

- Fragment ID: `rgbdns-frag-7b1032dce75b`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 886-956
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-7b1032dce75b", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 886, "endLine": 956}
```

## Excerpt

<span id="rgbdns-frag-7b1032dce75b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-7b1032dce75b: heading 18. Troubleshoot by boundary

````markdown
## 18. Troubleshoot by boundary

### AXFR ends with `end of file`

First test ordinary DNS over TCP:

```sh
dig @172.31.60.189 fieldnotes.es SOA +tcp +norecurse
```

Then test AXFR from the authorized secondary:

```sh
dig @172.31.60.189 fieldnotes.es AXFR
```

If private AXFR works but AXFR through `52.10.53.234` does not, keep
`PRIMARY=172.31.60.189`; the private path is the intended AWS-to-AWS route.
BuddyNS continues to use the public primary address.

Check the primary's effective allow-list:

```sh
sudo grep '^ALLOW_NETS=' /etc/rgbdns/tinydns.env
sudo journalctl -u rgbdns-tinydns.service -n 100 --no-pager
```

### A one-shot is `inactive (dead)`

For `rgbdns-secondary-sync.service`, this is success when:

```text
Result=success
ExecMainStatus=0
ActiveState=inactive
SubState=dead
```

### A path upload did nothing

Check the configured path and owner:

```sh
sudo cat /etc/rgbdns/data-drop.env 2>/dev/null || true
sudo cat /etc/rgbdns/zones-drop.env 2>/dev/null || true
systemctl status rgbdns-data.path rgbdns-zones.path --no-pager
```

Ensure the final file is owned by the configured login user and publish with
the `.new` plus rename sequence.

### The unit changed on disk

After a package upgrade:

```sh
sudo systemctl daemon-reload
```

This reloads systemd unit definitions; it does not alter rgbdns configuration
or zone data.

### Confirm listening sockets

```sh
sudo ss -lntup '( sport = :53 )'
```

The authoritative process should own both UDP and TCP port 53. AXFR is handled
through the integrated TCP listener; do not start a second `axfrdns` service
on the same address and port.
````
