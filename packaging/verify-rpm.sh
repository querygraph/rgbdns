#!/bin/sh
set -eu

[ $# -eq 1 ] || { echo "usage: verify-rpm.sh PACKAGE.rpm" >&2; exit 2; }
package=$1

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
    /usr/lib/systemd/system/rgbdns-tinydns.service \
    /usr/lib/systemd/system/rgbdns-secondary-sync.service \
    /usr/lib/systemd/system/rgbdns-secondary-sync.timer
do
    test -e "$path"
done

grep -qx 'RuntimeDirectory=rgbdns' \
    /usr/lib/systemd/system/rgbdns-secondary-sync.service
grep -q '/run/rgbdns/secondary.lock' \
    /usr/lib/rgbdns/secondary-sync
grep -q 'ZONES is required' \
    /usr/lib/rgbdns/secondary-sync

getent passwd rgbdns
getent group rgbdns
test "$(stat -c %U:%G /var/lib/rgbdns/tinydns)" = rgbdns:rgbdns
test "$(stat -c %a /etc/rgbdns/tinydns.env)" = 640
systemd-analyze --man=no verify \
    /usr/lib/systemd/system/rgbdns-tinydns.service \
    /usr/lib/systemd/system/rgbdns-secondary-sync.service \
    /usr/lib/systemd/system/rgbdns-secondary-sync.timer

rgbdns-setup --help | grep -q -- '--zones'
