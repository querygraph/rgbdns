---
type: "code-fragment"
fragment_id: "rgbdns-frag-79b8154f3b7b"
source_path: "packaging/verify-deb.sh"
code_note: "DNS from First Principles/Code/packaging/verify-deb.sh.source"
language: "bash"
subsystem: "Repository and build"
symbol: "verify-deb.sh"
kind: "file"
start_line: 1
end_line: 43
---

# verify-deb.sh

- Fragment ID: `rgbdns-frag-79b8154f3b7b`
- Source file: [[DNS from First Principles/Code/packaging/verify-deb.sh.source|packaging/verify-deb.sh]]
- Lines: 1-43
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-79b8154f3b7b", "codeNote": "DNS from First Principles/Code/packaging/verify-deb.sh.source", "heading": "rgbdns-frag-79b8154f3b7b: file verify-deb.sh", "sourcePath": "packaging/verify-deb.sh", "startLine": 1, "endLine": 43}
```

## Excerpt

<span id="rgbdns-frag-79b8154f3b7b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-79b8154f3b7b: file verify-deb.sh

```bash
#!/bin/sh
set -eu

package=$1

export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install --yes "$package" python3

dpkg-query --show --showformat='${Status}\n' rgbdns |
    grep -qx 'install ok installed'

installed_bins=$(mktemp)
trap 'rm -f "$installed_bins"' EXIT HUP INT TERM

dpkg-deb --fsys-tarfile "$package" |
    tar -tf - |
    sed -n 's#^\./usr/bin/##p' |
    sed '/^$/d' |
    sort >"$installed_bins"

dpkg-deb --control "$package" /tmp/rgbdns-control
test -s /tmp/rgbdns-control/control

while IFS= read -r binary; do
    test -x "/usr/bin/$binary"
done <"$installed_bins"

for unit in \
    rgbdns-tinydns.service \
    rgbdns-secondary-sync.service \
    rgbdns-secondary-sync.timer
do
    test -f "/lib/systemd/system/$unit"
done

for helper in \
    /usr/lib/rgbdns/compile-zone \
    /usr/lib/rgbdns/secondary-sync \
    /usr/sbin/rgbdns-setup
do
    test -x "$helper"
done
```
