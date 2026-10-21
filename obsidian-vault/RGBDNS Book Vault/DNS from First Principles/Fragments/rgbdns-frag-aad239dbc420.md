---
type: "code-fragment"
fragment_id: "rgbdns-frag-aad239dbc420"
source_path: "docs/OPENSUSE.md"
code_note: "DNS from First Principles/Code/docs/OPENSUSE.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Launch the current Marketplace image"
kind: "heading"
start_line: 17
end_line: 47
---

# Launch the current Marketplace image

- Fragment ID: `rgbdns-frag-aad239dbc420`
- Source file: [[DNS from First Principles/Code/docs/OPENSUSE.md.source|docs/OPENSUSE.md]]
- Lines: 17-47
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-aad239dbc420", "codeNote": "DNS from First Principles/Code/docs/OPENSUSE.md.source", "sourcePath": "docs/OPENSUSE.md", "startLine": 17, "endLine": 47}
```

## Excerpt

<span id="rgbdns-frag-aad239dbc420" class="rgbdns-fragment-target"></span>
### rgbdns-frag-aad239dbc420: heading Launch the current Marketplace image

````markdown
## Launch the current Marketplace image

1. In the EC2 console, choose **Launch instance**.
2. Under **Application and OS Images**, choose **Browse more AMIs**, then
   **AWS Marketplace AMIs**.
3. Select the openSUSE-published **openSUSE Leap** product and its current
   **Leap 16.0 (x86_64)** delivery option.
4. Choose an x86_64 instance with enough memory to compile Rust. A small
   production DNS instance can run in modest memory, but building is more
   comfortable on a separate build host or a temporary instance with at least
   4 GiB.
5. Require IMDSv2, use an encrypted EBS volume, attach the SSH key, and assign
   the instance an Elastic IP if it will be an Internet authority.
6. Allow SSH only from administrator addresses. Add public UDP 53 and TCP 53.
   Ordinary DNS requires both; AXFR shares the TCP listener and is restricted
   separately inside rgbdns.

The Marketplace image's SSH user is `ec2-user`:

```sh
ssh -i ~/.ssh/KEY.pem ec2-user@PUBLIC_ADDRESS
sudo zypper refresh
sudo zypper --non-interactive update
sudo zypper --non-interactive install bind-utils
cat /etc/os-release
uname -m
```

Reboot if the update installs a new kernel, then reconnect. The examples below
assume `x86_64`. Build an architecture-matching RPM for any other target.

````
