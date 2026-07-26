---
type: "code-fragment"
fragment_id: "rgbdns-frag-604b6ff56e55"
source_path: "docs/book/rgbdns.md"
code_note: "DNS from First Principles/Code/docs/book/rgbdns.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Case study cron.sh primary and BuddyNS secondaries"
kind: "heading"
start_line: 1080
end_line: 1141
---

# Case study cron.sh primary and BuddyNS secondaries

- Fragment ID: `rgbdns-frag-604b6ff56e55`
- Source file: [[DNS from First Principles/Code/docs/book/rgbdns.md.source|docs/book/rgbdns.md]]
- Lines: 1080-1141
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-604b6ff56e55", "codeNote": "DNS from First Principles/Code/docs/book/rgbdns.md.source", "heading": "rgbdns-frag-604b6ff56e55: heading Case study cron.sh primary and BuddyNS secondaries", "sourcePath": "docs/book/rgbdns.md", "startLine": 1080, "endLine": 1141}
```

## Excerpt

<span id="rgbdns-frag-604b6ff56e55" class="rgbdns-fragment-target"></span>
### rgbdns-frag-604b6ff56e55: heading Case study cron.sh primary and BuddyNS secondaries

```markdown
## Case study: cron.sh primary and BuddyNS secondaries

Consider a Debian EC2 instance behind Elastic IP `52.10.53.234`. It is the
editable primary for `cron.sh`, published as `a.ns.cron.sh`, while BuddyNS
copies and serves the zone as a secondary. This arrangement illustrates the
whole operational chain: package installation, zone authority, glue,
single-address AXFR, delegation, supervision, and updates.

At the network boundary, allow public UDP and TCP port 53. UDP carries most
queries; TCP is required both for ordinary retry behavior and for AXFR. Do not
restrict all TCP DNS to the secondary provider. The server distinguishes AXFR
questions on the shared stream and applies a narrow source allow-list only to
those transfers. On EC2 the guest usually sees a private address rather than
the Elastic IP, so listening on `0.0.0.0:53` lets AWS translate traffic for the
public address.

Build the native package on a Debian or Ubuntu machine of the same
architecture, copy it to the server, and install it:

```sh
sudo apt update
sudo apt install -y build-essential cargo debhelper rustc git
git clone https://github.com/querygraph/rgbdns.git
cd rgbdns
packaging/build-deb.sh
scp ../rgbdns_0.1.1_amd64.deb admin@52.10.53.234:/tmp/
ssh admin@52.10.53.234
sudo apt install -y /tmp/rgbdns_0.1.1_amd64.deb
```

Installation creates the non-login `rgbdns` account, the protected
configuration and state directories, and the hardened systemd units. It does
not start a nameserver. That separation prevents package installation from
publishing placeholder data.

The primary source includes the SOA, the in-bailiwick primary and its glue, and
the account-assigned BuddyNS names. In schematic form:

```text
Zcron.sh:a.ns.cron.sh:hostmaster.cron.sh:2026072601:16384:2048:1048576:2560:3600
&cron.sh:52.10.53.234:a.ns.cron.sh:3600
&cron.sh::<BuddyNS name 1>:3600
&cron.sh::<BuddyNS name 2>:3600
&cron.sh::<BuddyNS name 3>:3600
+a.ns.cron.sh:52.10.53.234:3600
```

Replace the placeholders with the names shown in BuddyBoard. The complete,
directly compilable source is in the Debian deployment guide. Store it as
`/root/cron.sh.data`, add the application records, and increment the SOA serial
on every publication. The empty address fields on the BuddyNS NS lines are
intentional: glue for those names belongs to BuddyNS, not `cron.sh`.

BuddyNS publishes the addresses from which its cluster initiates transfers.
Its current documentation says every published source must be allowed. For an
IPv4-only primary, express the published IPv4 addresses as exact `/32`
networks:

```sh
BUDDYNS_AXFR_V4='108.61.224.67/32,116.203.6.3/32'
BUDDYNS_AXFR_V4="$BUDDYNS_AXFR_V4,107.191.99.111/32"
BUDDYNS_AXFR_V4="$BUDDYNS_AXFR_V4,193.109.120.66/32"
```
