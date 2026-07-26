#!/usr/bin/env python3
"""Print binary target names from `cargo metadata` JSON, one per line."""

import json
import sys


metadata = json.load(sys.stdin)
package = next(package for package in metadata["packages"] if package["name"] == "rgbdns")
names = {
    target["name"]
    for target in package["targets"]
    if "bin" in target["kind"]
}
for name in sorted(names):
    print(name)

