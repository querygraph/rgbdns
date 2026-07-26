#!/usr/bin/env python3
"""Build an Obsidian vault edition for the RGBDNS Rust book."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import shutil
import subprocess
from dataclasses import dataclass, field
from pathlib import Path, PurePosixPath
from typing import Iterable


REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_RGBDNS_ROOT = Path(os.environ.get("RGBDNS_CODE_ROOT", str(REPO_ROOT)))
DEFAULT_OUTPUT = REPO_ROOT / "obsidian-vault" / "RGBDNS Book Vault"
VAULT_BOOK = "DNS from First Principles"
MANUSCRIPT = REPO_ROOT / "docs" / "book" / "rgbdns.md"

SKIP_DIRS = {
    ".git",
    ".mypy_cache",
    ".pytest_cache",
    ".ruff_cache",
    ".tox",
    ".venv",
    ".venvs",
    "__pycache__",
    "dist",
    "node_modules",
    "obsidian-vault",
    "spark-warehouse",
    "target",
}
TEXT_SUFFIXES = {
    ".bash",
    ".css",
    ".dockerignore",
    ".editorconfig",
    ".env",
    ".feature",
    ".gitignore",
    ".html",
    ".js",
    ".json",
    ".lock",
    ".md",
    ".mjs",
    ".proto",
    ".py",
    ".rs",
    ".sh",
    ".toml",
    ".ts",
    ".tsx",
    ".txt",
    ".yaml",
    ".yml",
}
TEXT_NAMES = {
    "Dockerfile",
    "LICENSE",
    "Makefile",
}
RUST_SYMBOL_RE = re.compile(
    r"^\s*(?:pub(?:\([^)]*\))?\s+)?"
    r"(?:(?:async|unsafe|const|extern)\s+)*"
    r"(?P<kind>fn|struct|enum|trait|impl|mod|type|const|static)\b"
    r"(?:\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*))?"
)
PY_SYMBOL_RE = re.compile(
    r"^\s*(?P<kind>class|def|async\s+def)\s+(?P<name>[A-Za-z_][A-Za-z0-9_]*)"
)
MD_HEADING_RE = re.compile(r"^(?P<marks>#{1,6})\s+(?P<name>.+?)\s*$")


@dataclass(slots=True)
class SourceFile:
    path: str
    absolute: Path
    note_path: str
    language: str
    subsystem: str
    crate: str | None
    lines: list[str]
    summary: str
    fragments: list["Fragment"] = field(default_factory=list)


@dataclass(slots=True)
class Fragment:
    id: str
    source_path: str
    note_path: str
    code_note: str
    heading: str
    symbol: str
    kind: str
    language: str
    subsystem: str
    crate: str | None
    start_line: int
    end_line: int
    summary: str


def run_git(args: list[str], cwd: Path) -> str:
    try:
        return subprocess.check_output(["git", *args], cwd=cwd, text=True).strip()
    except Exception:
        return "unknown"


def clean_name(value: str, limit: int = 100) -> str:
    value = re.sub(r'[\\/:*?"<>|#^[\]]+', " ", value)
    value = re.sub(r"\s+", " ", value).strip(" .")
    return (value or "Untitled")[:limit].rstrip()


def slug(value: str, limit: int = 80) -> str:
    value = re.sub(r"[^A-Za-z0-9_.-]+", "-", value).strip("-")
    return (value or "item")[:limit].strip("-")


def yaml_value(value: object) -> str:
    return json.dumps(value, ensure_ascii=True)


def frontmatter(values: dict[str, object]) -> str:
    lines = ["---"]
    for key, value in values.items():
        if value is None:
            continue
        if isinstance(value, (list, tuple, set)):
            lines.append(f"{key}:")
            for item in value:
                lines.append(f"  - {yaml_value(item)}")
        elif isinstance(value, bool):
            lines.append(f"{key}: {'true' if value else 'false'}")
        else:
            lines.append(f"{key}: {yaml_value(value)}")
    lines.append("---")
    return "\n".join(lines)


def wiki(path: str, label: str | None = None) -> str:
    target = path[:-3] if path.endswith(".md") else path
    return f"[[{target}|{label}]]" if label else f"[[{target}]]"


def write_text(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(text.rstrip() + "\n", encoding="utf-8")


def write_note(root: Path, path: str, metadata: dict[str, object], body: str) -> None:
    note = root / (path if path.endswith(".md") else f"{path}.md")
    write_text(note, f"{frontmatter(metadata)}\n\n{body}")


def language_for(path: str) -> str:
    suffix = Path(path).suffix.lower()
    return {
        ".css": "css",
        ".feature": "gherkin",
        ".html": "html",
        ".js": "javascript",
        ".json": "json",
        ".md": "markdown",
        ".mjs": "javascript",
        ".proto": "protobuf",
        ".py": "python",
        ".rs": "rust",
        ".sh": "bash",
        ".toml": "toml",
        ".ts": "typescript",
        ".tsx": "tsx",
        ".yaml": "yaml",
        ".yml": "yaml",
    }.get(suffix, "")


def subsystem_for(path: str) -> tuple[str, str | None]:
    parts = PurePosixPath(path).parts
    name = PurePosixPath(path).name
    if parts and parts[0] == "src":
        if len(parts) >= 2 and parts[1] == "bin":
            return "Command-line programs", name.removesuffix(".rs")
        if name in {"name.rs", "packet.rs"}:
            return "DNS data model and wire codec", "rgbdns"
        if name in {"zone.rs", "server.rs", "cdb.rs"}:
            return "Authoritative service", "rgbdns"
        if name in {"client.rs", "dnscache_config.rs"}:
            return "Resolution and recursion", "rgbdns"
        if name in {"axfr.rs", "transport.rs"}:
            return "Transport and zone transfer", "rgbdns"
        if name in {"conf.rs", "multilog.rs", "setuidgid.rs", "tai64.rs"}:
            return "Operations and supervision", "rgbdns"
        if name in {"pick.rs", "rbl.rs", "wall.rs", "special.rs"}:
            return "Specialized responders", "rgbdns"
        return "Rust library", "rgbdns"
    if parts and parts[0] in {"tests", "benches", "examples"}:
        return "Tests and performance", None
    if parts and parts[0] == "docs":
        return "Documentation", None
    if parts and parts[0] == "scripts":
        return "Developer scripts", None
    if parts and parts[0] == ".github":
        return "Project automation", None
    return "Repository and build", None


def summary_for(path: str, subsystem: str, lines: list[str]) -> str:
    if path.endswith("Cargo.toml"):
        return f"Cargo manifest for {subsystem}."
    for line in lines[:80]:
        stripped = line.strip()
        if stripped.startswith("//!") or stripped.startswith("///"):
            return stripped.lstrip("/! ").strip()
        if stripped.startswith('"""') and len(stripped) > 3:
            return stripped.strip('" ')
        if stripped.startswith("# "):
            return stripped.lstrip("# ").strip()
    return f"Source file in the {subsystem} subsystem."


def should_include(path: Path, root: Path) -> bool:
    rel = path.relative_to(root)
    if any(part in SKIP_DIRS for part in rel.parts):
        return False
    if not path.is_file():
        return False
    if path.name in TEXT_NAMES:
        return True
    if path.suffix.lower() in TEXT_SUFFIXES:
        return True
    return False


def read_text(path: Path) -> str | None:
    if path.stat().st_size > 1_500_000:
        return None
    try:
        data = path.read_bytes()
    except OSError:
        return None
    if b"\x00" in data:
        return None
    try:
        return data.decode("utf-8")
    except UnicodeDecodeError:
        try:
            return data.decode("utf-8", errors="replace")
        except Exception:
            return None


def code_note_path(path: str) -> str:
    safe = "/".join(clean_name(part, 120) for part in PurePosixPath(path).parts)
    return f"{VAULT_BOOK}/Code/{safe}.source"


def fragment_id(path: str, start: int, end: int, symbol: str) -> str:
    raw = f"{path}:{start}:{end}:{symbol}".encode("utf-8")
    return f"rgbdns-frag-{hashlib.sha1(raw).hexdigest()[:12]}"


def extract_fragments(source: SourceFile) -> list[Fragment]:
    matches: list[tuple[int, str, str]] = []
    regex = RUST_SYMBOL_RE if source.language == "rust" else PY_SYMBOL_RE if source.language == "python" else None
    if regex:
        for index, line in enumerate(source.lines, start=1):
            match = regex.match(line)
            if not match:
                continue
            kind = match.group("kind").replace(" ", "-")
            name = match.groupdict().get("name") or "impl"
            matches.append((index, kind, name))
    elif source.language == "markdown":
        for index, line in enumerate(source.lines, start=1):
            match = MD_HEADING_RE.match(line)
            if match and len(match.group("marks")) <= 2:
                matches.append((index, "heading", clean_name(match.group("name"), 80)))

    if not matches and source.lines:
        matches.append((1, "file", Path(source.path).name))

    fragments: list[Fragment] = []
    for i, (start, kind, name) in enumerate(matches[:80]):
        next_start = matches[i + 1][0] if i + 1 < len(matches) else len(source.lines) + 1
        end = min(next_start - 1, start + 80, len(source.lines))
        fid = fragment_id(source.path, start, end, f"{kind}:{name}")
        heading = f"{fid}: {kind} {name}"
        fragments.append(Fragment(
            id=fid,
            source_path=source.path,
            note_path=f"{VAULT_BOOK}/Fragments/{fid}",
            code_note=source.note_path,
            heading=heading,
            symbol=name,
            kind=kind,
            language=source.language,
            subsystem=source.subsystem,
            crate=source.crate,
            start_line=start,
            end_line=end,
            summary=f"{kind} `{name}` in `{source.path}` lines {start}-{end}.",
        ))
    return fragments


def inventory_codebase(rgbdns_root: Path) -> list[SourceFile]:
    files: list[SourceFile] = []
    for path in sorted(rgbdns_root.rglob("*")):
        if not should_include(path, rgbdns_root):
            continue
        text = read_text(path)
        if text is None:
            continue
        rel = path.relative_to(rgbdns_root).as_posix()
        subsystem, crate = subsystem_for(rel)
        lines = text.splitlines()
        source = SourceFile(
            path=rel,
            absolute=path,
            note_path=code_note_path(rel),
            language=language_for(rel),
            subsystem=subsystem,
            crate=crate,
            lines=lines,
            summary=summary_for(rel, subsystem, lines),
        )
        source.fragments = extract_fragments(source)
        files.append(source)
    return files


def manuscript_chapters() -> list[tuple[str, str, str]]:
    """Split the canonical single-file manuscript into stable Obsidian notes."""
    text = MANUSCRIPT.read_text(encoding="utf-8")
    lines = text.splitlines()
    starts: list[int] = []
    fence: str | None = None
    for index, line in enumerate(lines):
        stripped = line.lstrip()
        marker = stripped[:3]
        if marker in {"```", "~~~"}:
            if fence is None:
                fence = marker
            elif marker == fence:
                fence = None
            continue
        if fence is None and line.startswith("# "):
            starts.append(index)

    chapters: list[tuple[str, str, str]] = []
    for index, start in enumerate(starts, start=1):
        end = starts[index] if index < len(starts) else len(lines)
        chunk = "\n".join(lines[start:end]).rstrip()
        first = chunk.splitlines()[0]
        title = re.sub(r"\s+\{[^}]+\}\s*$", "", first[2:]).strip()
        note = f"{VAULT_BOOK}/Chapters/{index:02d}-{slug(title, 72)}"
        chapters.append((title, note, chunk.rstrip()))
    return chapters


def choose_chapter_fragments(title: str, fragments: list[Fragment], limit: int = 12) -> list[Fragment]:
    title_words = set(re.findall(r"[a-z0-9]+", title.lower()))
    mapping = {
        "name": {"src/name.rs"},
        "names": {"src/name.rs"},
        "packet": {"src/packet.rs"},
        "messages": {"src/packet.rs"},
        "wire": {"src/packet.rs", "src/transport.rs"},
        "authority": {"src/zone.rs", "src/server.rs", "src/cdb.rs"},
        "authoritative": {"src/zone.rs", "src/server.rs", "src/cdb.rs"},
        "recursion": {"src/dnscache_config.rs", "src/bin/dnscache.rs"},
        "dnssec": {"src/dnscache_config.rs", "src/bin/dnscache.rs"},
        "transfer": {"src/axfr.rs"},
        "client": {"src/client.rs"},
        "security": {"src/packet.rs", "tests/wire_security.rs", "tests/packet_properties.rs"},
        "performance": {"benches/dns_core.rs", "docs/performance.md"},
        "testing": {"tests/rfc_conformance.rs", "tests/wire_security.rs", "tests/packet_properties.rs"},
        "supervision": {"src/conf.rs", "src/multilog.rs", "src/setuidgid.rs"},
        "rust": {"src/name.rs", "src/packet.rs", "src/server.rs"},
        "codebase": {"src/lib.rs", "src/name.rs", "src/packet.rs", "src/zone.rs", "src/server.rs"},
        "cdb": {"src/cdb.rs", "src/zone.rs"},
        "transport": {"src/transport.rs", "src/client.rs", "src/axfr.rs"},
    }
    paths: set[str] = set()
    for word in title_words:
        paths.update(mapping.get(word, set()))
    if not paths:
        paths = {"src/lib.rs", "README.md"}
    selected = [fragment for fragment in fragments if fragment.source_path in paths]
    selected.sort(key=lambda fragment: (fragment.source_path, fragment.start_line))
    return selected[:limit]


def render_fragment_block(fragment: Fragment) -> str:
    payload = {
        "id": fragment.id,
        "codeNote": fragment.code_note,
        "heading": fragment.heading,
        "sourcePath": fragment.source_path,
        "startLine": fragment.start_line,
        "endLine": fragment.end_line,
    }
    return "```rgbdns-fragment\n" + json.dumps(payload, ensure_ascii=True) + "\n```"


def render_chapter(title: str, text: str, fragments: list[Fragment]) -> str:
    related = choose_chapter_fragments(title, fragments)
    body = [text, "", "## Generated Code Fragment Index", ""]
    if related:
        body.append("These generated links open the collocated rgbdns codebase notes.")
        body.append("")
        for fragment in related:
            body.append(render_fragment_block(fragment))
            body.append("")
    else:
        body.append("No generated fragments were matched for this chapter.")
    return "\n".join(body)


def render_source_file(source: SourceFile) -> str:
    lines = [
        f"# {source.path}",
        "",
        f"- Subsystem: [[{VAULT_BOOK}/Subsystems/{clean_name(source.subsystem)}|{source.subsystem}]]",
    ]
    if source.crate:
        lines.append(f"- Component: [[{VAULT_BOOK}/Components/{source.crate}|{source.crate}]]")
    lines.extend([
        f"- Source path: `{source.path}`",
        f"- Lines: {len(source.lines)}",
        f"- Summary: {source.summary}",
        "",
        "## Extracted Fragments",
        "",
    ])
    for fragment in source.fragments[:80]:
        lines.append(f"- [[{fragment.note_path}|{fragment.symbol}]]: lines {fragment.start_line}-{fragment.end_line}")
    lines.extend(["", "## Full Source", "", f"```{source.language}"])
    lines.extend(source.lines)
    lines.append("```")
    return "\n".join(lines)


def render_fragment_note(fragment: Fragment, source: SourceFile) -> str:
    excerpt = source.lines[fragment.start_line - 1:fragment.end_line]
    lines = [
        f"# {fragment.symbol}",
        "",
        f"- Fragment ID: `{fragment.id}`",
        f"- Source file: [[{fragment.code_note}|{fragment.source_path}]]",
        f"- Lines: {fragment.start_line}-{fragment.end_line}",
        f"- Subsystem: [[{VAULT_BOOK}/Subsystems/{clean_name(fragment.subsystem)}|{fragment.subsystem}]]",
    ]
    if fragment.crate:
        lines.append(f"- Component: [[{VAULT_BOOK}/Components/{fragment.crate}|{fragment.crate}]]")
    lines.extend([
        "",
        render_fragment_block(fragment),
        "",
        "## Excerpt",
        "",
        f'<span id="{fragment.id}" class="rgbdns-fragment-target"></span>',
        f"### {fragment.heading}",
        "",
        f"```{fragment.language}",
        *excerpt,
        "```",
    ])
    return "\n".join(lines)


def render_index(title: str, items: Iterable[tuple[str, str]]) -> str:
    lines = [f"# {title}", ""]
    for label, path in sorted(items):
        lines.append(f"- {wiki(path, label)}")
    return "\n".join(lines)


def render_vault_readme(manifest: dict[str, object]) -> str:
    return "\n".join([
        "# DNS from First Principles — Obsidian Vault",
        "",
        "This vault is the reader edition of *DNS from First Principles* by Alexy Khrabrov.",
        "It packages the complete book together with the complete text/code surface of rgbdns.",
        "",
        "## Contents",
        "",
        f"- Book chapters: {manifest['chapter_count']}",
        f"- rgbdns code-file notes: {manifest['code_file_count']}",
        f"- Generated code fragments: {manifest['fragment_count']}",
        f"- rgbdns source commit: `{manifest['rgbdns_commit']}`",
        f"- Book source commit: `{manifest['book_commit']}`",
        "",
        "## Start Here",
        "",
        "- Open `Home.md` in Obsidian.",
        "- Follow `DNS from First Principles/Book.md` for the chapter map.",
        "- Use `DNS from First Principles/Indices/Code Files.md` for source navigation.",
        "- Use `DNS from First Principles/Indices/Fragments.md` for symbol excerpts.",
        "",
        "## Fragment Navigation",
        "",
        "The vault includes a local Obsidian plugin named `rgbdns-reader`.",
        "When enabled, generated `rgbdns-fragment` cards can open the related code-file note",
        "and highlight the selected fragment. The plugin is bundled inside `.obsidian/plugins/`",
        "and does not require a build step.",
        "",
        "## Data Ledgers",
        "",
        "Machine-readable ledgers live under `DNS from First Principles/_data/`:",
        "",
        "- `manifest.json` records the build inputs and counts.",
        "- `files.json` records included rgbdns source files.",
        "- `fragments.json` records fragment IDs and line ranges.",
        "- `symbols.json` records extracted Rust and Markdown symbols.",
        "- `links.json` records generated graph edges.",
        "- `units.jsonl` is the publishing compatibility ledger used by FirstPair.",
        "",
        "The vault is generated from source. Rebuild it from the book repository rather than",
        "hand-editing generated notes.",
    ])


def copy_plugin(output: Path) -> None:
    source = REPO_ROOT / "obsidian-plugin" / "rgbdns-reader"
    target = output / ".obsidian" / "plugins" / "rgbdns-reader"
    if target.exists():
        shutil.rmtree(target)
    shutil.copytree(source, target)
    write_text(output / ".obsidian" / "community-plugins.json", json.dumps(["rgbdns-reader"], indent=2))


def copy_book_assets(output: Path) -> None:
    assets = output / "Assets"
    assets.mkdir(parents=True, exist_ok=True)
    for name in ("rgbdns-cover.png", "rgbdns-headboard.png"):
        shutil.copy2(REPO_ROOT / "cover" / name, assets / name)


def build_vault(rgbdns_root: Path, output: Path) -> dict[str, object]:
    if output.exists():
        shutil.rmtree(output)
    output.mkdir(parents=True)

    rgbdns_commit = run_git(["rev-parse", "--short=8", "HEAD"], rgbdns_root)
    book_commit = run_git(["rev-parse", "--short=8", "HEAD"], REPO_ROOT)
    sources = inventory_codebase(rgbdns_root)
    fragments = [fragment for source in sources for fragment in source.fragments]

    write_note(output, "Home", {
        "type": "vault",
        "book": "DNS from First Principles",
        "rgbdns_commit": rgbdns_commit,
        "book_commit": book_commit,
    }, "\n".join([
        "# DNS from First Principles",
        "",
        "![[Assets/rgbdns-headboard.png]]",
        "",
        f"Open {wiki(f'{VAULT_BOOK}/Book', 'the book map')} to start.",
        "",
        "This generated Obsidian vault collocates the book text with the current rgbdns codebase.",
        "Use fragment cards to jump from explanatory text to highlighted code excerpts.",
    ]))

    chapter_links: list[tuple[str, str]] = []
    for title, note_path, text in manuscript_chapters():
        body = render_chapter(title, text, fragments)
        chapter_links.append((title, note_path))
        write_note(output, note_path, {
            "type": "chapter",
            "source_file": MANUSCRIPT.relative_to(REPO_ROOT).as_posix(),
        }, body)

    write_note(output, f"{VAULT_BOOK}/Book", {
        "type": "book",
        "rgbdns_commit": rgbdns_commit,
        "book_commit": book_commit,
        "chapter_count": len(chapter_links),
        "code_file_count": len(sources),
        "fragment_count": len(fragments),
    }, "\n".join([
        "# DNS from First Principles",
        "",
        "## Chapters",
        "",
        *[f"- {wiki(path, title)}" for title, path in chapter_links],
        "",
        "## Codebase",
        "",
        f"- {wiki(f'{VAULT_BOOK}/Indices/Code Files', 'Code files')}",
        f"- {wiki(f'{VAULT_BOOK}/Indices/Fragments', 'Code fragments')}",
        f"- {wiki(f'{VAULT_BOOK}/Indices/Components', 'Components')}",
        f"- {wiki(f'{VAULT_BOOK}/Indices/Subsystems', 'Subsystems')}",
    ]))

    crate_items: dict[str, list[SourceFile]] = {}
    subsystem_items: dict[str, list[SourceFile]] = {}
    for source in sources:
        if source.crate:
            crate_items.setdefault(source.crate, []).append(source)
        subsystem_items.setdefault(source.subsystem, []).append(source)
        write_note(output, source.note_path, {
            "type": "code-file",
            "source_path": source.path,
            "language": source.language,
            "subsystem": source.subsystem,
            "crate": source.crate,
            "line_count": len(source.lines),
            "fragment_count": len(source.fragments),
            "rgbdns_commit": rgbdns_commit,
        }, render_source_file(source))

    sources_by_path = {source.path: source for source in sources}
    for fragment in fragments:
        source = sources_by_path[fragment.source_path]
        write_note(output, fragment.note_path, {
            "type": "code-fragment",
            "fragment_id": fragment.id,
            "source_path": fragment.source_path,
            "code_note": fragment.code_note,
            "language": fragment.language,
            "subsystem": fragment.subsystem,
            "crate": fragment.crate,
            "symbol": fragment.symbol,
            "kind": fragment.kind,
            "start_line": fragment.start_line,
            "end_line": fragment.end_line,
        }, render_fragment_note(fragment, source))

    for crate, crate_sources in crate_items.items():
        write_note(output, f"{VAULT_BOOK}/Components/{crate}", {
            "type": "component",
            "component": crate,
            "file_count": len(crate_sources),
        }, render_index(crate, [(source.path, source.note_path) for source in crate_sources]))
    for subsystem, subsystem_sources in subsystem_items.items():
        write_note(output, f"{VAULT_BOOK}/Subsystems/{clean_name(subsystem)}", {
            "type": "subsystem",
            "subsystem": subsystem,
            "file_count": len(subsystem_sources),
        }, render_index(subsystem, [(source.path, source.note_path) for source in subsystem_sources]))

    write_note(output, f"{VAULT_BOOK}/Indices/Code Files", {"type": "index"}, render_index(
        "Code Files", [(source.path, source.note_path) for source in sources]
    ))
    write_note(output, f"{VAULT_BOOK}/Indices/Fragments", {"type": "index"}, render_index(
        "Code Fragments", [(f"{fragment.id}: {fragment.symbol}", fragment.note_path) for fragment in fragments]
    ))
    write_note(output, f"{VAULT_BOOK}/Indices/Components", {"type": "index"}, render_index(
        "Components", [(crate, f"{VAULT_BOOK}/Components/{crate}") for crate in crate_items]
    ))
    write_note(output, f"{VAULT_BOOK}/Indices/Subsystems", {"type": "index"}, render_index(
        "Subsystems", [(name, f"{VAULT_BOOK}/Subsystems/{clean_name(name)}") for name in subsystem_items]
    ))

    data_dir = output / VAULT_BOOK / "_data"
    data_dir.mkdir(parents=True, exist_ok=True)
    files_json = [
        {
            "path": source.path,
            "note_path": source.note_path,
            "language": source.language,
            "subsystem": source.subsystem,
            "crate": source.crate,
            "line_count": len(source.lines),
            "summary": source.summary,
            "fragments": [fragment.id for fragment in source.fragments],
        }
        for source in sources
    ]
    fragments_json = [
        {
            "id": fragment.id,
            "source_path": fragment.source_path,
            "note_path": fragment.note_path,
            "code_note": fragment.code_note,
            "heading": fragment.heading,
            "symbol": fragment.symbol,
            "kind": fragment.kind,
            "language": fragment.language,
            "subsystem": fragment.subsystem,
            "crate": fragment.crate,
            "start_line": fragment.start_line,
            "end_line": fragment.end_line,
            "summary": fragment.summary,
        }
        for fragment in fragments
    ]
    links_json = []
    for source in sources:
        for fragment in source.fragments:
            links_json.append({"from": source.note_path, "to": fragment.note_path, "type": "file-fragment"})
            links_json.append({"from": fragment.note_path, "to": source.note_path, "type": "fragment-file"})
    symbols_json = [
        {
            "name": fragment.symbol,
            "kind": fragment.kind,
            "fragment_id": fragment.id,
            "source_path": fragment.source_path,
            "start_line": fragment.start_line,
            "end_line": fragment.end_line,
        }
        for fragment in fragments
    ]
    units = (
        [
            {
                "id": f"chapter:{path}",
                "kind": "chapter",
                "note_path": path,
                "title": title,
            }
            for title, path in chapter_links
        ]
        + [
            {
                "id": f"file:{source.path}",
                "kind": "code-file",
                "note_path": source.note_path,
                "source_path": source.path,
                "language": source.language,
                "subsystem": source.subsystem,
                "crate": source.crate,
            }
            for source in sources
        ]
        + [
            {
                "id": fragment.id,
                "kind": "code-fragment",
                "note_path": fragment.note_path,
                "source_path": fragment.source_path,
                "code_note": fragment.code_note,
                "symbol": fragment.symbol,
                "start_line": fragment.start_line,
                "end_line": fragment.end_line,
            }
            for fragment in fragments
        ]
    )

    for name, payload in {
        "files.json": files_json,
        "fragments.json": fragments_json,
        "symbols.json": symbols_json,
        "links.json": links_json,
    }.items():
        write_text(data_dir / name, json.dumps(payload, indent=2, ensure_ascii=True))
    write_text(
        data_dir / "units.jsonl",
        "\n".join(json.dumps(unit, ensure_ascii=True) for unit in units),
    )

    copy_plugin(output)
    copy_book_assets(output)

    manifest = {
        "book": "DNS from First Principles",
        "rgbdns_root": str(rgbdns_root),
        "rgbdns_commit": rgbdns_commit,
        "book_commit": book_commit,
        "chapter_count": len(chapter_links),
        "code_file_count": len(sources),
        "fragment_count": len(fragments),
        "output": str(output),
    }
    write_text(data_dir / "manifest.json", json.dumps(manifest, indent=2, ensure_ascii=True))
    write_text(output / "README.md", render_vault_readme(manifest))
    return manifest


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--rgbdns-root", type=Path, default=DEFAULT_RGBDNS_ROOT)
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    args = parser.parse_args()
    manifest = build_vault(args.rgbdns_root.resolve(), args.output.resolve())
    print(json.dumps(manifest, indent=2, ensure_ascii=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
