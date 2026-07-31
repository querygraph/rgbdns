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
    rgbdns-data.path
do
    test -f "/lib/systemd/system/$unit"
done

for helper in \
    /usr/lib/rgbdns/compile-zone \
    /usr/lib/rgbdns/secondary-sync \
    /usr/lib/rgbdns/import-zones \
    /usr/lib/rgbdns/import-data \
    /usr/lib/rgbdns/migrate-zones \
    /usr/lib/rgbdns/restore-role-units \
    /usr/sbin/rgbdns-setup
do
    test -x "$helper"
done

grep -q '/etc/rgbdns/zones' /usr/lib/rgbdns/secondary-sync
rgbdns-setup --help | grep -q -- '--zones'
rgbdns-setup --help | grep -q -- '--zones-drop'
rgbdns-setup --help | grep -q -- '--data-drop'
rgbdns-setup --help | grep -q -- '--query-log'
grep -qx 'QUERY_LOG=1' /etc/rgbdns/tinydns.env
test "$(stat -c %U:%G /etc/rgbdns)" = root:rgbdns
test "$(stat -c %a /etc/rgbdns)" = 750
test "$(stat -c %U:%G /etc/rgbdns/tinydns.env)" = root:rgbdns
grep -q 'enable --now rgbdns-data.path' \
    /usr/lib/rgbdns/restore-role-units
grep -q 'enable --now rgbdns-secondary-sync.timer' \
    /usr/lib/rgbdns/restore-role-units
grep -q 'enable --now rgbdns-tinydns.service' \
    /usr/lib/rgbdns/restore-role-units
! grep -Fq ': >"$stage"' /usr/lib/rgbdns/import-zones
