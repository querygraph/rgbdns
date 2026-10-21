---
type: "code-file"
source_path: "scripts/test_obsidian_vault.py"
language: "python"
subsystem: "Developer scripts"
line_count: 135
fragment_count: 21
rgbdns_commit: "5ae24772"
---

# scripts/test_obsidian_vault.py

- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]
- Source path: `scripts/test_obsidian_vault.py`
- Lines: 135
- Summary: Unit tests for the Obsidian vault builder and validator helpers.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-3c2e54f9135e|load]]: lines 19-30
- [[DNS from First Principles/Fragments/rgbdns-frag-3b6a4c86f9c4|PluginListMigration]]: lines 31-31
- [[DNS from First Principles/Fragments/rgbdns-frag-9fe1e9fff355|test_clean_output_seeds_empty_list]]: lines 32-34
- [[DNS from First Principles/Fragments/rgbdns-frag-ce2c242a7628|test_legacy_id_replaced_in_place]]: lines 35-40
- [[DNS from First Principles/Fragments/rgbdns-frag-3f2d366ffc26|test_disabled_reader_stays_disabled]]: lines 41-43
- [[DNS from First Principles/Fragments/rgbdns-frag-2af91edb62e1|test_no_duplicate_reader]]: lines 44-50
- [[DNS from First Principles/Fragments/rgbdns-frag-314a89b35607|PortableManifest]]: lines 51-51
- [[DNS from First Principles/Fragments/rgbdns-frag-afe8fcfe0dbc|test_repo_root_is_dot]]: lines 52-54
- [[DNS from First Principles/Fragments/rgbdns-frag-7b59f74b9b8a|test_external_root_is_symbolic]]: lines 55-59
- [[DNS from First Principles/Fragments/rgbdns-frag-74e07395993b|ReaderExcerpt]]: lines 60-60
- [[DNS from First Principles/Fragments/rgbdns-frag-5b741cc6bca6|test_markdown_fences_keep_full_source_whole]]: lines 61-70
- [[DNS from First Principles/Fragments/rgbdns-frag-fbfc363dc163|test_missing_full_source]]: lines 71-74
- [[DNS from First Principles/Fragments/rgbdns-frag-2192bf8fd319|LocalPathScan]]: lines 75-75
- [[DNS from First Principles/Fragments/rgbdns-frag-973ce54d9cdd|test_detects_local_paths_in_ledgers]]: lines 76-89
- [[DNS from First Principles/Fragments/rgbdns-frag-92fe67fc316b|test_portable_values_pass]]: lines 90-100
- [[DNS from First Principles/Fragments/rgbdns-frag-2ff6279909fb|ReaderPluginCheck]]: lines 101-101
- [[DNS from First Principles/Fragments/rgbdns-frag-205b4c6a0f96|make]]: lines 102-109
- [[DNS from First Principles/Fragments/rgbdns-frag-5baf88436b54|test_release_install_passes]]: lines 110-115
- [[DNS from First Principles/Fragments/rgbdns-frag-ca5c911ed7e6|test_legacy_and_sources_fail]]: lines 116-124
- [[DNS from First Principles/Fragments/rgbdns-frag-eef6db7231f6|ReaderSource]]: lines 125-125
- [[DNS from First Principles/Fragments/rgbdns-frag-f3653fa16a6d|test_firstpair_reader_release_is_current]]: lines 126-135

## Full Source

```python
#!/usr/bin/env python3
"""Unit tests for the Obsidian vault builder and validator helpers.

Run with: python3 -m unittest scripts/test_obsidian_vault.py
They build no vault; fixtures are small temporary folders.
"""

from __future__ import annotations

import importlib.util
import json
import tempfile
import unittest
from pathlib import Path

SCRIPTS = Path(__file__).resolve().parent


def load(name: str, filename: str):
    spec = importlib.util.spec_from_file_location(name, SCRIPTS / filename)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


builder = load("rgbdns_vault_builder", "build-obsidian-vault.py")
checker = load("rgbdns_vault_checker", "check-obsidian-vault.py")


class PluginListMigration(unittest.TestCase):
    def test_clean_output_seeds_empty_list(self) -> None:
        self.assertEqual(builder.migrate_plugin_list(None), [])

    def test_legacy_id_replaced_in_place(self) -> None:
        self.assertEqual(
            builder.migrate_plugin_list(["dataview", "rgbdns-reader", "outliner"]),
            ["dataview", "firstpair-reader", "outliner"],
        )

    def test_disabled_reader_stays_disabled(self) -> None:
        self.assertEqual(builder.migrate_plugin_list(["dataview"]), ["dataview"])

    def test_no_duplicate_reader(self) -> None:
        self.assertEqual(
            builder.migrate_plugin_list(["firstpair-reader", "rgbdns-reader"]),
            ["firstpair-reader"],
        )


class PortableManifest(unittest.TestCase):
    def test_repo_root_is_dot(self) -> None:
        self.assertEqual(builder.portable_root(builder.REPO_ROOT), ".")

    def test_external_root_is_symbolic(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            self.assertEqual(builder.portable_root(Path(temporary)), "$RGBDNS_CODE_ROOT")


class ReaderExcerpt(unittest.TestCase):
    def test_markdown_fences_keep_full_source_whole(self) -> None:
        lines = ["# Title", "", "```sh", "make", "```", "", "after", "````", "x", "````", "end"]
        fence = builder.fence_for(lines)
        note = "\n".join(["# f", "", "## Full Source", "", f"{fence}markdown", *lines, fence])
        excerpt = checker.reader_excerpt_lines(note)
        self.assertIsNotNone(excerpt)
        self.assertEqual(excerpt, lines, "source lines stay verbatim inside the longer fence")
        self.assertEqual(excerpt[6], "after")
        self.assertGreater(len(fence), 4)

    def test_missing_full_source(self) -> None:
        self.assertIsNone(checker.reader_excerpt_lines("# note\n\n```rust\nfn x() {}\n```\n"))


class LocalPathScan(unittest.TestCase):
    def test_detects_local_paths_in_ledgers(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            data = root / checker.BOOK_ROOT / "_data"
            data.mkdir(parents=True)
            (data / "manifest.json").write_text(
                json.dumps({"rgbdns_root": "/data/data/com.termux/files/home/src/rgbdns"}),
                encoding="utf-8",
            )
            (data / "files.json").write_text(json.dumps([{"summary": "~~~ fences"}]), encoding="utf-8")
            (root / "FIRSTPAIR-VAULT-MANIFEST.json").write_text('{"root": "~/src/x"}', encoding="utf-8")
            errors = checker.check_local_paths(root)
            self.assertEqual(len(errors), 2, errors)

    def test_portable_values_pass(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            data = root / checker.BOOK_ROOT / "_data"
            data.mkdir(parents=True)
            (data / "manifest.json").write_text(
                json.dumps({"rgbdns_root": ".", "vault_name": "RGBDNS Book Vault"}), encoding="utf-8"
            )
            self.assertEqual(checker.check_local_paths(root), [])


class ReaderPluginCheck(unittest.TestCase):
    def make(self, root: Path, plugins: list[str], files: tuple[str, ...]) -> None:
        plugin = root / ".obsidian" / "plugins" / "firstpair-reader"
        plugin.mkdir(parents=True)
        for name in files:
            content = json.dumps({"id": "firstpair-reader"}) if name == "manifest.json" else ""
            (plugin / name).write_text(content, encoding="utf-8")
        (root / ".obsidian" / "community-plugins.json").write_text(json.dumps(plugins), encoding="utf-8")

    def test_release_install_passes(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.make(root, ["firstpair-reader"], checker.READER_RELEASE_FILES)
            self.assertEqual(checker.check_reader_plugin(root), [])

    def test_legacy_and_sources_fail(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self.make(root, ["rgbdns-reader"], (*checker.READER_RELEASE_FILES, "package.json"))
            (root / ".obsidian" / "plugins" / "rgbdns-reader").mkdir()
            errors = checker.check_reader_plugin(root)
            self.assertEqual(len(errors), 3, errors)


class ReaderSource(unittest.TestCase):
    def test_firstpair_reader_release_is_current(self) -> None:
        if not (builder.FIRSTPAIR_ROOT / "publishing" / "vault" / "plugin" / "firstpair-reader").is_dir():
            self.skipTest("FirstPair checkout not available")
        source = builder.reader_plugin_source()
        manifest = json.loads((source / "manifest.json").read_text(encoding="utf-8"))
        self.assertEqual(manifest["id"], "firstpair-reader")


if __name__ == "__main__":
    unittest.main()
```
