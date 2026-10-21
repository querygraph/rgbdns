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
