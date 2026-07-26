---
type: "code-file"
source_path: "packaging/verify-deb.sh"
language: "bash"
subsystem: "Repository and build"
line_count: 43
fragment_count: 1
rgbdns_commit: "79502939"
---

# packaging/verify-deb.sh

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `packaging/verify-deb.sh`
- Lines: 43
- Summary: Source file in the Repository and build subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-79b8154f3b7b|verify-deb.sh]]: lines 1-43

## Full Source

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
