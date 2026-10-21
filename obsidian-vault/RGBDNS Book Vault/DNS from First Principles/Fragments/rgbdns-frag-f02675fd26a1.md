---
type: "code-fragment"
fragment_id: "rgbdns-frag-f02675fd26a1"
source_path: "docs/OPENSUSE.md"
code_note: "DNS from First Principles/Code/docs/OPENSUSE.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Complete deployment cron.sh primary with BuddyNS"
kind: "heading"
start_line: 173
end_line: 253
---

# Complete deployment cron.sh primary with BuddyNS

- Fragment ID: `rgbdns-frag-f02675fd26a1`
- Source file: [[DNS from First Principles/Code/docs/OPENSUSE.md.source|docs/OPENSUSE.md]]
- Lines: 173-253
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-f02675fd26a1", "codeNote": "DNS from First Principles/Code/docs/OPENSUSE.md.source", "sourcePath": "docs/OPENSUSE.md", "startLine": 173, "endLine": 253}
```

## Excerpt

<span id="rgbdns-frag-f02675fd26a1" class="rgbdns-fragment-target"></span>
### rgbdns-frag-f02675fd26a1: heading Complete deployment cron.sh primary with BuddyNS

````markdown
## Complete deployment: cron.sh primary with BuddyNS

This example uses the EC2 instance behind Elastic IP `52.10.53.234` as
`a.ns.cron.sh`, the editable primary for `cron.sh`. BuddyNS copies the zone by
AXFR and serves it from its secondary network. The guest normally sees its
private address, so rgbdns listens on `0.0.0.0:53` and AWS performs the Elastic
IP translation.

### Create the primary zone

Create the protected canonical source:

```sh
sudo install -m 0600 /dev/null /root/cron.sh.data
sudoedit /root/cron.sh.data
```

Start with:

```text
Zcron.sh:a.ns.cron.sh:hostmaster.cron.sh:2026072601:16384:2048:1048576:2560:3600
&cron.sh:52.10.53.234:a.ns.cron.sh:3600
&cron.sh::uz5x6wcwzfbjs8fkmkuchydn9339lf7xbxdmnp038cmyjlgg9sprr2.free.ns.buddyns.com:3600
&cron.sh::uz5dkwpjfvfwb9rh1qj93mtup0gw65s6j7vqqumch0r9gzlu8qxx39.free.ns.buddyns.com:3600
&cron.sh::uz56xw8h7fw656bpfv84pctjbl9rbzbqrw4rpzdhtvzyltpjdmx0zq.free.ns.buddyns.com:3600
+a.ns.cron.sh:52.10.53.234:3600
```

Use the secondary names assigned in BuddyBoard if they differ. The empty
address fields on BuddyNS `&` records are intentional; their glue belongs to
BuddyNS. Add the application's A, AAAA, MX, TXT, CAA, or other records and
increment the SOA serial on every publication.

### Configure integrated AXFR

BuddyNS requires every address in its current transfer-source inventory to be
allowed. Recheck BuddyNS's published zone-transfer instructions before
deployment and after provider network changes. As recorded on 2026-07-26, the
IPv4 sources were:

```text
108.61.224.67
116.203.6.3
107.191.99.111
193.109.120.66
5.223.55.119
192.184.93.99
103.25.56.55
216.73.156.203
37.143.61.179
195.20.17.193
45.77.29.133
116.203.0.64
167.88.161.228
199.195.249.208
104.244.78.122
```

Represent them as exact `/32` networks and configure the service:

```sh
BUDDYNS_AXFR_V4='108.61.224.67/32,116.203.6.3/32,107.191.99.111/32,193.109.120.66/32,5.223.55.119/32,192.184.93.99/32,103.25.56.55/32,216.73.156.203/32,37.143.61.179/32,195.20.17.193/32,45.77.29.133/32,116.203.0.64/32,167.88.161.228/32,199.195.249.208/32,104.244.78.122/32'
sudo rgbdns-setup primary \
  --data /root/cron.sh.data \
  --listen-ip 0.0.0.0 \
  --port 53 \
  --allow-nets "$BUDDYNS_AXFR_V4"
```

The command validates and copies the source, compiles `data.cdb`, writes
`/etc/rgbdns/tinydns.env`, enables the unit at boot, and starts it. Inspect the
result:

It also watches `rgbdns.data` in the invoking sudo user's home. Publish future
versions through a temporary name and atomic rename:

```sh
scp rgbdns.data a.ns.cron.sh:rgbdns.data.new
ssh a.ns.cron.sh 'mv rgbdns.data.new rgbdns.data'
```

````
