---
type: "code-fragment"
fragment_id: "rgbdns-frag-bc8396806643"
source_path: "packaging/build-rpm.sh"
code_note: "DNS from First Principles/Code/packaging/build-rpm.sh.source"
language: "bash"
subsystem: "Repository and build"
symbol: "build-rpm.sh"
kind: "file"
start_line: 1
end_line: 38
---

# build-rpm.sh

- Fragment ID: `rgbdns-frag-bc8396806643`
- Source file: [[DNS from First Principles/Code/packaging/build-rpm.sh.source|packaging/build-rpm.sh]]
- Lines: 1-38
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-bc8396806643", "codeNote": "DNS from First Principles/Code/packaging/build-rpm.sh.source", "sourcePath": "packaging/build-rpm.sh", "startLine": 1, "endLine": 38}
```

## Excerpt

<span id="rgbdns-frag-bc8396806643" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bc8396806643: file build-rpm.sh

```bash
#!/bin/sh
set -eu

repo=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$repo"

case $(uname -o 2>/dev/null || uname -s) in
    Android) echo "build-rpm: run on openSUSE, not Android/Termux" >&2; exit 1 ;;
esac
command -v rpmbuild >/dev/null 2>&1 ||
    { echo "build-rpm: install rpm-build, cargo, rust, python3, and systemd-rpm-macros" >&2; exit 1; }

version=$(sed -n 's/^version = "\([^"]*\)"/\1/p' Cargo.toml | head -n 1)
[ -n "$version" ] || { echo "build-rpm: cannot read Cargo package version" >&2; exit 1; }
spec_version=$(sed -n 's/^Version:[[:space:]]*//p' packaging/rpm/rgbdns.spec)
[ "$version" = "$spec_version" ] || {
    echo "build-rpm: Cargo version $version does not match RPM version $spec_version" >&2
    exit 1
}

topdir=${RPM_TOPDIR:-"$repo/dist/rpmbuild"}
mkdir -p "$topdir"/BUILD "$topdir"/BUILDROOT "$topdir"/RPMS \
    "$topdir"/SOURCES "$topdir"/SPECS "$topdir"/SRPMS

archive="$topdir/SOURCES/rgbdns-$version.tar.gz"
file_list="$topdir/SOURCES/rgbdns-$version.files"
git -c "safe.directory=$repo" \
    ls-files --cached --others --exclude-standard -z >"$file_list"
if [ "${RGBDNS_SKIP_RUST_BUILD:-0}" = 1 ]; then
    find target/release -maxdepth 1 -type f -perm -0100 -printf 'target/release/%f\0' >>"$file_list"
fi
tar --null --files-from="$file_list" --transform "s,^,rgbdns-$version/," \
    --create --gzip --file "$archive"
cp packaging/rpm/rgbdns.spec "$topdir/SPECS/rgbdns.spec"

exec rpmbuild -ba \
    --define "_topdir $topdir" \
    "$topdir/SPECS/rgbdns.spec" "$@"
```
