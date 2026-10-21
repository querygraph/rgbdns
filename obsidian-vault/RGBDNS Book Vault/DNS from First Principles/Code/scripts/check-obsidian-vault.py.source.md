---
type: "code-file"
source_path: "scripts/check-obsidian-vault.py"
language: "python"
subsystem: "Developer scripts"
line_count: 354
fragment_count: 12
rgbdns_commit: "5ae24772"
---

# scripts/check-obsidian-vault.py

- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]
- Source path: `scripts/check-obsidian-vault.py`
- Lines: 354
- Summary: Validate the generated rgbdns Book Obsidian vault.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-2c86330a96e5|strip_fenced_code]]: lines 27-38
- [[DNS from First Principles/Fragments/rgbdns-frag-1029d555bc2c|fragment_payloads]]: lines 39-59
- [[DNS from First Principles/Fragments/rgbdns-frag-cbf5df7d6289|note_key]]: lines 60-63
- [[DNS from First Principles/Fragments/rgbdns-frag-74446dad54e5|load_json]]: lines 64-67
- [[DNS from First Principles/Fragments/rgbdns-frag-ee9d28fbd8b7|reader_excerpt_lines]]: lines 68-91
- [[DNS from First Principles/Fragments/rgbdns-frag-7944c6ab3faa|local_path_files]]: lines 92-105
- [[DNS from First Principles/Fragments/rgbdns-frag-d186e032be32|check_local_paths]]: lines 106-122
- [[DNS from First Principles/Fragments/rgbdns-frag-32521dbf2817|check_reader_plugin]]: lines 123-161
- [[DNS from First Principles/Fragments/rgbdns-frag-a2525c419e35|check_code_book_contract]]: lines 162-183
- [[DNS from First Principles/Fragments/rgbdns-frag-c62e93f7bb78|target_exists]]: lines 184-196
- [[DNS from First Principles/Fragments/rgbdns-frag-d11bb7a9ad74|run]]: lines 197-277
- [[DNS from First Principles/Fragments/rgbdns-frag-86193e307eca|main]]: lines 331-354

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
READER_PLUGIN_ID = "firstpair-reader"
READER_RELEASE_FILES = ("README.md", "main.js", "manifest.json", "styles.css")
LEGACY_PLUGIN_IDS = ("rgbdns-reader",)
FRAGMENT_KEYS = ("id", "source_path", "code_note", "start_line", "end_line", "symbol", "summary")
# FirstPair Reader 2 (fillExcerpt): first fence after "## Full Source", ended
# by the first line that starts with three backticks.
LOCAL_PATH_RE = re.compile(
    r"/Users/|/home/|/data/data/|(?<![\w.~/])~(?=/|\"|$)|\b[A-Za-z]:\\\\"
)
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


def reader_excerpt_lines(text: str) -> list[str] | None:
    """Return the code lines FirstPair Reader 2 numbers for a code note.

    Mirrors the Reader's fencedLines: the first fence after "## Full Source"
    closes only at a fence of the same character at least as long (CommonMark).
    """
    full = text.find("## Full Source")
    if full < 0:
        return None
    lines = text[full:].split("\n")
    opening = None
    for index, line in enumerate(lines):
        match = re.match(r" {0,3}(`{3,}|~{3,})", line)
        if not match:
            continue
        if opening is None:
            opening = (index, match.group(1))
            continue
        closing = re.match(r" {0,3}(`+|~+)\s*$", line)
        if closing and closing.group(1)[0] == opening[1][0] and len(closing.group(1)) >= len(opening[1]):
            return lines[opening[0] + 1:index]
    return None if opening is None else lines[opening[0] + 1:]


def local_path_files(root: Path) -> list[Path]:
    """Ledgers and manifests that must carry no machine-local paths."""
    candidates: set[Path] = set()
    data_root = root / BOOK_ROOT / "_data"
    if data_root.is_dir():
        candidates.update(path for path in data_root.rglob("*") if path.is_file())
    candidates.update(root.glob("*.json"))
    obsidian = root / ".obsidian"
    if obsidian.is_dir():
        candidates.update(obsidian.glob("*.json"))
        candidates.update(obsidian.glob("plugins/*/manifest.json"))
    return sorted(candidates)


def check_local_paths(root: Path) -> list[str]:
    errors: list[str] = []
    for path in local_path_files(root):
        rel = path.relative_to(root).as_posix()
        try:
            text = path.read_text(encoding="utf-8")
        except (OSError, UnicodeDecodeError) as exc:
            errors.append(f"{rel}: unreadable while scanning for local paths: {exc}")
            continue
        for number, line in enumerate(text.splitlines(), start=1):
            match = LOCAL_PATH_RE.search(line)
            if match:
                errors.append(f"{rel}:{number}: machine-local absolute path {match.group(0)!r}")
                break
    return errors


def check_reader_plugin(root: Path) -> list[str]:
    errors: list[str] = []
    plugins = root / ".obsidian" / "plugins"
    for legacy in LEGACY_PLUGIN_IDS:
        if (plugins / legacy).exists():
            errors.append(f"legacy plugin still installed: .obsidian/plugins/{legacy}")
    plugin = plugins / READER_PLUGIN_ID
    for name in READER_RELEASE_FILES:
        if not (plugin / name).is_file():
            errors.append(f"missing FirstPair Reader release file: .obsidian/plugins/{READER_PLUGIN_ID}/{name}")
    if plugin.is_dir():
        extra = sorted(
            path.relative_to(plugin).as_posix()
            for path in plugin.rglob("*")
            if path.is_file() and path.relative_to(plugin).as_posix() not in {*READER_RELEASE_FILES, "data.json"}
        )
        if extra:
            errors.append(f"FirstPair Reader folder holds non-release files: {', '.join(extra)}")
        try:
            manifest = load_json(plugin / "manifest.json")
            if manifest.get("id") != READER_PLUGIN_ID:
                errors.append(f"plugin manifest id is {manifest.get('id')!r}, expected {READER_PLUGIN_ID!r}")
        except Exception as exc:
            errors.append(f"could not read FirstPair Reader manifest: {exc}")
    enabled_path = root / ".obsidian" / "community-plugins.json"
    try:
        enabled = load_json(enabled_path)
    except Exception as exc:
        errors.append(f"could not read .obsidian/community-plugins.json: {exc}")
    else:
        if not isinstance(enabled, list) or not all(isinstance(item, str) for item in enabled):
            errors.append(".obsidian/community-plugins.json is not a list of plugin ids")
        else:
            for legacy in LEGACY_PLUGIN_IDS:
                if legacy in enabled:
                    errors.append(f".obsidian/community-plugins.json still enables {legacy}")
    return errors


def check_code_book_contract(root: Path) -> list[str]:
    """FirstPair Reader 2 code books: <Book>/_data/fragments.json beside <Book>/Chapters/."""
    errors: list[str] = []
    book = root / BOOK_ROOT
    if not (book / "_data" / "fragments.json").is_file():
        errors.append(f"code book has no {BOOK_ROOT}/_data/fragments.json")
    chapters = sorted((book / "Chapters").glob("*.md")) if (book / "Chapters").is_dir() else []
    if not chapters:
        errors.append(f"code book has no chapter notes in {BOOK_ROOT}/Chapters/")
    if not (book / "Book.md").is_file():
        errors.append(f"code book has no launcher {BOOK_ROOT}/Book.md")
    # Reader 2 discovers the vault root first, then top-level folders in name
    # order; nothing may claim the Reader before the book folder.
    for shadow in ("_data", "Reader/reader-index.json"):
        if (root / shadow).exists():
            errors.append(f"vault root holds {shadow}, which FirstPair Reader would read instead of the book")
    for folder in sorted(path for path in root.iterdir() if path.is_dir() and not path.name.startswith(".")):
        if folder.name != BOOK_ROOT and (folder / "_data").exists():
            errors.append(f"{folder.name}/_data competes with the book for FirstPair Reader discovery")
    return errors


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

    errors.extend(check_reader_plugin(root))
    errors.extend(check_code_book_contract(root))
    errors.extend(check_local_paths(root))

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
    excerpt_cache: dict[str, list[str] | None] = {}
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
        missing_keys = [key for key in FRAGMENT_KEYS if key not in fragment]
        if missing_keys:
            errors.append(f"fragment {fid} lacks Reader fields: {', '.join(missing_keys)}")
        start = fragment.get("start_line")
        end = fragment.get("end_line")
        if not isinstance(start, int) or not isinstance(end, int) or start < 1 or end < start:
            errors.append(f"fragment {fid} has invalid line range {start}-{end}")
            continue
        code_note = fragment.get("code_note")
        if code_note not in notes:
            continue
        if code_note not in excerpt_cache:
            excerpt_cache[code_note] = reader_excerpt_lines(
                (root / f"{code_note}.md").read_text(encoding="utf-8")
            )
        excerpt = excerpt_cache[code_note]
        if excerpt is None:
            errors.append(f"fragment {fid}: {code_note} has no fenced block after '## Full Source'")
        elif len(excerpt) < end:
            errors.append(
                f"fragment {fid}: FirstPair Reader would read {len(excerpt)} source lines "
                f"from {code_note}, fewer than end_line {end}"
            )

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
