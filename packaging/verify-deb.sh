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
