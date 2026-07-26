#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
cd "$repo_root"

command -v cargo >/dev/null
command -v dpkg-deb >/dev/null
command -v python3 >/dev/null

version=$(python3 -c 'import tomllib; print(tomllib.load(open("Cargo.toml","rb"))["package"]["version"])')
architecture=${RGBDNS_DEB_ARCH:-$(dpkg --print-architecture)}
output_dir=${RGBDNS_DEB_OUTPUT:-"$repo_root/dist"}
package_root=$(mktemp -d)
trap 'rm -rf "$package_root"' EXIT HUP INT TERM

cargo build --release --locked --bins

root="$package_root/rgbdns_${version}_${architecture}"
mkdir -p "$root/DEBIAN" "$root/usr/bin" "$root/etc/rgbdns" \
  "$root/lib/systemd/system" "$root/var/lib/rgbdns/tinydns" \
  "$root/usr/share/doc/rgbdns/examples"

python3 - <<'PY' >"$package_root/binaries"
import json, subprocess
metadata = json.loads(subprocess.check_output(
    ["cargo", "metadata", "--format-version", "1", "--no-deps"],
    text=True,
))
package = next(p for p in metadata["packages"] if p["name"] == "rgbdns")
for target in package["targets"]:
    if "bin" in target["kind"]:
        print(target["name"])
PY

while IFS= read -r binary; do
    install -m 0755 "target/release/$binary" "$root/usr/bin/$binary"
done <"$package_root/binaries"

install -m 0644 packaging/debian/rgbdns-tinydns.service \
  "$root/lib/systemd/system/rgbdns-tinydns.service"
install -m 0644 packaging/debian/tinydns.env "$root/etc/rgbdns/tinydns.env"
install -m 0644 packaging/debian/data.example \
  "$root/usr/share/doc/rgbdns/examples/data"
install -m 0644 README.md CHANGELOG.md "$root/usr/share/doc/rgbdns/"

cat >"$root/DEBIAN/control" <<EOF
Package: rgbdns
Version: $version
Section: net
Priority: optional
Architecture: $architecture
Maintainer: rgbdns contributors
Depends: libc6
Description: memory-safe Rust reimplementation of the djbdns suite
 Includes authoritative UDP/TCP DNS, CDB zone compilation, iterative caching,
 transfer tools, diagnostics, and djbdns-compatible service utilities.
EOF

cat >"$root/DEBIAN/conffiles" <<'EOF'
/etc/rgbdns/tinydns.env
EOF

install -m 0755 packaging/debian/postinst "$root/DEBIAN/postinst"
install -m 0755 packaging/debian/prerm "$root/DEBIAN/prerm"

mkdir -p "$output_dir"
dpkg-deb --root-owner-group --build "$root" \
  "$output_dir/rgbdns_${version}_${architecture}.deb"
dpkg-deb --info "$output_dir/rgbdns_${version}_${architecture}.deb"
printf '%s\n' "$output_dir/rgbdns_${version}_${architecture}.deb"
