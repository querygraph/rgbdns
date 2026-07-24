---
type: "code-file"
source_path: "scripts/check-obsidian-vault.py"
language: "python"
subsystem: "Developer scripts"
line_count: 208
fragment_count: 7
rgbdns_commit: "472c2087"
---

# scripts/check-obsidian-vault.py

- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]
- Source path: `scripts/check-obsidian-vault.py`
- Lines: 208
- Summary: Validate the generated rgbdns Book Obsidian vault.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-4ce63a83ca29|strip_fenced_code]]: lines 18-29
- [[DNS from First Principles/Fragments/rgbdns-frag-14f680024c75|fragment_payloads]]: lines 30-50
- [[DNS from First Principles/Fragments/rgbdns-frag-40e10e268998|note_key]]: lines 51-54
- [[DNS from First Principles/Fragments/rgbdns-frag-bcaecfe813fa|load_json]]: lines 55-58
- [[DNS from First Principles/Fragments/rgbdns-frag-f9ca4b4c614c|target_exists]]: lines 59-71
- [[DNS from First Principles/Fragments/rgbdns-frag-859c6d21dd44|run]]: lines 72-152
- [[DNS from First Principles/Fragments/rgbdns-frag-3eab80834151|main]]: lines 185-208

## Full Source

```python
#!/usr/bin/env python3
"""Validate the generated rgbdns Book Obsidian vault."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path, PurePosixPath


BOOK_ROOT = "DNS from First Principles"
WIKILINK_RE = re.compile(r"!?\[\[([^\]\n]+)\]\]")
FENCE_RE = re.compile(r"^[ \t]*(```|~~~)")


def strip_fenced_code(text: str) -> str:
    lines: list[str] = []
    in_fence = False
    for line in text.splitlines():
        if FENCE_RE.match(line):
            in_fence = not in_fence
            lines.append("")
            continue
        lines.append("" if in_fence else line)
    return "\n".join(lines)


def fragment_payloads(text: str) -> list[str]:
    """Return rgbdns-fragment payloads that are not examples inside other fences."""
    payloads: list[str] = []
    lines = text.splitlines()
    index = 0
    in_other_fence = False
    while index < len(lines):
        stripped = lines[index].strip()
        if not in_other_fence and stripped == "```rgbdns-fragment":
            index += 1
            body: list[str] = []
            while index < len(lines) and lines[index].strip() != "```":
                body.append(lines[index])
                index += 1
            payloads.append("\n".join(body))
        elif stripped.startswith(("```", "~~~")):
            in_other_fence = not in_other_fence
        index += 1
    return payloads


def note_key(path: Path, root: Path) -> str:
    return path.relative_to(root).with_suffix("").as_posix()


def load_json(path: Path) -> object:
    return json.loads(path.read_text(encoding="utf-8"))


def target_exists(target: str, notes: set[str], files: set[str]) -> bool:
    target = target.split("|", 1)[0].split("#", 1)[0]
    if not target:
        return True
    if target in notes:
        return True
    if target.endswith(".md") and target[:-3] in notes:
        return True
    if target in files:
        return True
    return False


def run(root: Path) -> list[str]:
    errors: list[str] = []
    files = {path.relative_to(root).as_posix() for path in root.rglob("*") if path.is_file()}
    notes = {note_key(path, root) for path in root.rglob("*.md")}

    required = [
        "README",
        "Home",
        f"{BOOK_ROOT}/Book",
        f"{BOOK_ROOT}/Indices/Code Files",
        f"{BOOK_ROOT}/Indices/Fragments",
        f"{BOOK_ROOT}/_data/manifest.json",
        f"{BOOK_ROOT}/_data/files.json",
        f"{BOOK_ROOT}/_data/fragments.json",
        f"{BOOK_ROOT}/_data/symbols.json",
        f"{BOOK_ROOT}/_data/links.json",
        f"{BOOK_ROOT}/_data/units.jsonl",
        ".obsidian/plugins/rgbdns-reader/main.js",
        ".obsidian/plugins/rgbdns-reader/manifest.json",
        ".obsidian/plugins/rgbdns-reader/styles.css",
        "Assets/rgbdns-cover.png",
        "Assets/rgbdns-headboard.png",
    ]
    for relative in required:
        if (
            relative.endswith(".json")
            or relative.endswith(".jsonl")
            or relative.endswith(".js")
            or relative.endswith(".css")
            or relative.endswith(".png")
        ):
            if relative not in files:
                errors.append(f"missing required file: {relative}")
        elif relative not in notes:
            errors.append(f"missing required note: {relative}.md")

    data_root = root / BOOK_ROOT / "_data"
    try:
        manifest = load_json(data_root / "manifest.json")
        code_files = load_json(data_root / "files.json")
        fragments = load_json(data_root / "fragments.json")
        symbols = load_json(data_root / "symbols.json")
        links = load_json(data_root / "links.json")
        units = [
            json.loads(line)
            for line in (data_root / "units.jsonl").read_text(encoding="utf-8").splitlines()
            if line.strip()
        ]
    except Exception as exc:
        errors.append(f"could not read data ledgers: {exc}")
        return errors

    if manifest.get("chapter_count", 0) < 28:
        errors.append("manifest reports fewer than 28 chapters")
    if manifest.get("code_file_count", 0) < 60:
        errors.append("manifest reports fewer than 60 code files")
    if manifest.get("fragment_count", 0) < 100:
        errors.append("manifest reports fewer than 100 fragments")
    if len(code_files) != manifest.get("code_file_count"):
        errors.append("files.json count does not match manifest")
    if len(fragments) != manifest.get("fragment_count"):
        errors.append("fragments.json count does not match manifest")
    if len(symbols) < len(fragments):
        errors.append("symbols.json should have at least one row per fragment")
    if len(links) < len(fragments):
        errors.append("links.json should have at least one edge per fragment")
    expected_units = manifest.get("chapter_count", 0) + len(code_files) + len(fragments)
    if len(units) != expected_units:
        errors.append(f"units.jsonl count {len(units)} does not match expected {expected_units}")

    fragment_ids = set()
    for fragment in fragments:
        fid = fragment.get("id")
        if not fid:
            errors.append("fragment row has no id")
            continue
        if fid in fragment_ids:
            errors.append(f"duplicate fragment id: {fid}")
        fragment_ids.add(fid)
        for key in ("note_path", "code_note"):
            value = fragment.get(key)
            if value not in notes:
                errors.append(f"fragment {fid} points to missing {key}: {value}")
        start = fragment.get("start_line")
        end = fragment.get("end_line")
        if not isinstance(start, int) or not isinstance(end, int) or start < 1 or end < start:
            errors.append(f"fragment {fid} has invalid line range {start}-{end}")

    for path in sorted(root.rglob("*.md")):
        rel = path.relative_to(root).as_posix()
        text = path.read_text(encoding="utf-8")
        link_text = strip_fenced_code(text)
        if "\t" in link_text:
            errors.append(f"{rel}: contains tabs")
        for match in WIKILINK_RE.finditer(link_text):
            if not target_exists(match.group(1), notes, files):
                errors.append(f"{rel}: broken wikilink {match.group(0)}")
        for payload_text in fragment_payloads(text):
            try:
                payload = json.loads(payload_text)
            except json.JSONDecodeError as exc:
                errors.append(f"{rel}: invalid rgbdns-fragment JSON: {exc}")
                continue
            fid = payload.get("id")
            if fid not in fragment_ids:
                errors.append(f"{rel}: rgbdns-fragment block points to unknown id {fid}")
            code_note = payload.get("codeNote")
            if code_note not in notes:
                errors.append(f"{rel}: rgbdns-fragment block points to missing note {code_note}")

    return errors


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("vault", type=Path)
    args = parser.parse_args()
    root = args.vault.resolve()
    errors = run(root)
    if errors:
        for error in errors[:200]:
            print(error, file=sys.stderr)
        if len(errors) > 200:
            print(f"... {len(errors) - 200} more errors", file=sys.stderr)
        return 1
    manifest = load_json(root / BOOK_ROOT / "_data" / "manifest.json")
    print(
        "vault ok: "
        f"{manifest['chapter_count']} chapters, "
        f"{manifest['code_file_count']} code files, "
        f"{manifest['fragment_count']} fragments"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```
