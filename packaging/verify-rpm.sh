#!/bin/sh
set -eu

[ $# -eq 1 ] || { echo "usage: verify-rpm.sh PACKAGE.rpm" >&2; exit 2; }
package=$1

rpm --query --package --provides "$package" |
    grep -qx 'group(rgbdns)'
rpm --query --package --requires "$package" |
    grep -qx 'group(rgbdns)'
rpm --query --package --list "$package" |
    grep -Eq '^/usr/share/man/man7/rgbdns\.7(\.gz)?$'
for superseded in daemontools djbdns; do
    rpm --query --package --conflicts "$package" |
        grep -qx "$superseded"
    rpm --query --package --obsoletes "$package" |
        grep -qx "$superseded"
done
zypper --non-interactive --no-gpg-checks install "$package"
rpm --verify rgbdns
rpm --query --info rgbdns

for path in \
    /usr/sbin/rgbdns-setup \
    /usr/lib/rgbdns/compile-zone \
    /usr/lib/rgbdns/secondary-sync \
    /usr/lib/rgbdns/import-zones \
    /usr/lib/rgbdns/import-data \
    /usr/lib/rgbdns/migrate-zones \
    /usr/lib/rgbdns/migrate-zone-drop \
    /usr/lib/rgbdns/restore-role-units \
    /usr/lib/systemd/system/rgbdns-tinydns.service \
    /usr/lib/systemd/system/rgbdns-secondary-sync.service \
    /usr/lib/systemd/system/rgbdns-secondary-sync.timer \
    /usr/lib/systemd/system/rgbdns-zones-import.service \
    /usr/lib/systemd/system/rgbdns-zones.path \
    /usr/lib/systemd/system/rgbdns-data-import.service \
    /usr/lib/systemd/system/rgbdns-data.path
do
    test -e "$path"
done

grep -qx 'RuntimeDirectory=rgbdns' \
    /usr/lib/systemd/system/rgbdns-secondary-sync.service
grep -q '/run/rgbdns/secondary.lock' \
    /usr/lib/rgbdns/secondary-sync
grep -q '/etc/rgbdns/zones' \
    /usr/lib/rgbdns/secondary-sync

getent passwd rgbdns
getent group rgbdns
test "$(stat -c %U:%G /var/lib/rgbdns/tinydns)" = rgbdns:rgbdns
test "$(stat -c %U:%G /etc/rgbdns)" = root:rgbdns
test "$(stat -c %a /etc/rgbdns)" = 750
test "$(stat -c %U:%G /etc/rgbdns/tinydns.env)" = root:rgbdns
test "$(stat -c %a /etc/rgbdns/tinydns.env)" = 640
grep -qx 'QUERY_LOG=1' /etc/rgbdns/tinydns.env
grep -q 'enable --now rgbdns-data.path' \
    /usr/lib/rgbdns/restore-role-units
grep -q 'enable --now rgbdns-secondary-sync.timer' \
    /usr/lib/rgbdns/restore-role-units
grep -q 'enable --now rgbdns-tinydns.service' \
    /usr/lib/rgbdns/restore-role-units
! grep -Fq ': >"$stage"' /usr/lib/rgbdns/import-zones
! grep -Fq '>>"$stage"' /usr/lib/rgbdns/import-zones
systemd-analyze --man=no verify \
    /usr/lib/systemd/system/rgbdns-tinydns.service \
    /usr/lib/systemd/system/rgbdns-secondary-sync.service \
    /usr/lib/systemd/system/rgbdns-secondary-sync.timer \
    /usr/lib/systemd/system/rgbdns-zones-import.service \
    /usr/lib/systemd/system/rgbdns-zones.path \
    /usr/lib/systemd/system/rgbdns-data-import.service \
    /usr/lib/systemd/system/rgbdns-data.path

rgbdns-setup --help | grep -q -- '--zones'
rgbdns-setup --help | grep -q -- '--zones-drop'
rgbdns-setup --help | grep -q -- '--data-drop'
rgbdns-setup --help | grep -q -- '--query-log'
