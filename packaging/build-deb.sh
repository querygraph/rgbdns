#!/bin/sh
set -eu

repo=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
cd "$repo"

case $(uname -o 2>/dev/null || uname -s) in
    Android) echo "build-deb: run on Debian/Ubuntu, not Android/Termux" >&2; exit 1 ;;
esac
command -v dpkg-buildpackage >/dev/null 2>&1 ||
    { echo "build-deb: install build-essential, cargo, rustc, and debhelper" >&2; exit 1; }

dpkg-buildpackage --build=binary --no-sign "$@"
