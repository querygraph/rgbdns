---
type: "code-fragment"
fragment_id: "rgbdns-frag-879e3e511e3a"
source_path: "packaging/verify-deb.sh"
code_note: "DNS from First Principles/Code/packaging/verify-deb.sh.source"
language: "bash"
subsystem: "Repository and build"
symbol: "verify-deb.sh"
kind: "file"
start_line: 1
end_line: 81
---

# verify-deb.sh

- Fragment ID: `rgbdns-frag-879e3e511e3a`
- Source file: [[DNS from First Principles/Code/packaging/verify-deb.sh.source|packaging/verify-deb.sh]]
- Lines: 1-81
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-879e3e511e3a", "codeNote": "DNS from First Principles/Code/packaging/verify-deb.sh.source", "sourcePath": "packaging/verify-deb.sh", "startLine": 1, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-879e3e511e3a" class="rgbdns-fragment-target"></span>
### rgbdns-frag-879e3e511e3a: file verify-deb.sh

```bash
#!/bin/sh
set -eu

package=$1

export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install --yes --no-install-recommends "$package" python3

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

for field in Conflicts Replaces; do
    relations=$(dpkg-deb --field "$package" "$field")
    for superseded in \
        axfrdns \
        daemontools \
        djbdns-conf \
        djbdns-utils \
        dnscache \
        rbldns \
        tinydns \
        walldns
    do
        printf '%s\n' "$relations" |
            tr ',' '\n' |
            sed 's/^[[:space:]]*//; s/[[:space:]]*$//' |
            grep -qx "$superseded"
    done
done

while IFS= read -r binary; do
    test -x "/usr/bin/$binary"
done <"$installed_bins"

for unit in \
    rgbdns-tinydns.service \
    rgbdns-secondary-sync.service \
    rgbdns-secondary-sync.timer \
    rgbdns-zones-import.service \
    rgbdns-zones.path \
    rgbdns-data-import.service \
    rgbdns-data.path \
    rgbdns-query-report.service \
    rgbdns-query-report.timer
do
    test -f "/lib/systemd/system/$unit"
done

for helper in \
    /usr/lib/rgbdns/compile-zone \
    /usr/lib/rgbdns/secondary-sync \
    /usr/lib/rgbdns/import-zones \
    /usr/lib/rgbdns/import-data \
    /usr/lib/rgbdns/migrate-zones \
    /usr/lib/rgbdns/migrate-zone-drop \
    /usr/lib/rgbdns/migrate-zone-state \
    /usr/lib/rgbdns/restore-role-units \
    /usr/lib/rgbdns/daily-query-report \
    /usr/sbin/rgbdns-setup
do
    test -x "$helper"
done

grep -q 'state_dir/zones' /usr/lib/rgbdns/secondary-sync
rgbdns-setup --help | grep -q -- '--zones'
rgbdns-setup --help | grep -q -- '--zones-drop'
rgbdns-setup --help | grep -q -- '--data-drop'
rgbdns-setup --help | grep -q -- '--query-log'
```
