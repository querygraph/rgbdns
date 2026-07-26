# Debian package and systemd service

Build an amd64 Debian package on Debian 12 or Ubuntu:

```sh
packaging/debian/build-deb.sh
```

The package installs the rgbdns program family into `/usr/bin`, the
authoritative service as `rgbdns-tinydns.service`, configuration in
`/etc/rgbdns/tinydns.env`, and persistent zone state in
`/var/lib/rgbdns/tinydns`.

Installation deliberately does not start the service with example data. Edit
`/var/lib/rgbdns/tinydns/data`, compile it, and start the service:

```sh
sudo dpkg -i dist/rgbdns_0.1.0_amd64.deb
sudoedit /var/lib/rgbdns/tinydns/data
cd /var/lib/rgbdns/tinydns
sudo -u rgbdns /usr/bin/tinydns-data
sudo systemctl enable --now rgbdns-tinydns.service
sudo systemctl status rgbdns-tinydns.service
```

The unit runs as the unprivileged `rgbdns` account and receives only
`CAP_NET_BIND_SERVICE` so it can bind UDP and TCP port 53. It reads an
immutable CDB at runtime; editing the source file alone does not change served
answers. Recompile and restart after each zone change:

```sh
cd /var/lib/rgbdns/tinydns
sudo -u rgbdns /usr/bin/tinydns-data
sudo systemctl restart rgbdns-tinydns.service
```

Before enabling the service, verify that no other daemon is occupying port 53:

```sh
sudo ss -lntup | grep ':53 ' || true
```

Allow both UDP and TCP port 53 through the host firewall and cloud security
group. On EC2, bind `IP=0.0.0.0` or the instance private address; the public
Elastic IP is translated by AWS and is not normally assigned to a local
interface.
