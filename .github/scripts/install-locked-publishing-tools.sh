#!/bin/sh
set -eu

arch=$(uname -m)
case $arch in
    arm64) release_arch=aarch64 ;;
    x86_64) release_arch=x86_64 ;;
    *) echo "unsupported macOS architecture: $arch" >&2; exit 2 ;;
esac

stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT HUP INT TERM
brew_bin=$(brew --prefix)/bin

curl --fail --location --silent --show-error \
    "https://github.com/typst/typst/releases/download/v0.15.0/typst-${release_arch}-apple-darwin.tar.xz" \
    --output "$stage/typst.tar.xz"
tar -xJf "$stage/typst.tar.xz" -C "$stage"
install -m 0755 "$stage/typst-${release_arch}-apple-darwin/typst" \
    "$brew_bin/typst"

curl --fail --location --silent --show-error \
    "https://github.com/astral-sh/uv/releases/download/0.11.28/uv-${release_arch}-apple-darwin.tar.gz" \
    --output "$stage/uv.tar.gz"
tar -xzf "$stage/uv.tar.gz" -C "$stage"
install -m 0755 "$stage/uv-${release_arch}-apple-darwin/uv" \
    "$stage/uv-${release_arch}-apple-darwin/uvx" \
    "$brew_bin/"
