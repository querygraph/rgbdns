#!/bin/sh
set -eu

[ $# -eq 1 ] || { echo "usage: verify-rpm.sh PACKAGE.rpm" >&2; exit 2; }
package=$1

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

getent passwd rgbdns
getent group rgbdns
test "$(stat -c %U:%G /var/lib/rgbdns/tinydns)" = rgbdns:rgbdns
test "$(stat -c %a /etc/rgbdns/tinydns.env)" = 640
ls /usr/share/man/man7/rgbdns.7* >/dev/null
systemd-analyze --man=no verify \
    /usr/lib/systemd/system/rgbdns-tinydns.service \
    /usr/lib/systemd/system/rgbdns-secondary-sync.service \
    /usr/lib/systemd/system/rgbdns-secondary-sync.timer

rgbdns-setup --help
