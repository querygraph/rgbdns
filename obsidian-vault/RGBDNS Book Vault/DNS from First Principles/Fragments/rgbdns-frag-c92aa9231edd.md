---
type: "code-fragment"
fragment_id: "rgbdns-frag-c92aa9231edd"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "reader_plugin_source"
kind: "def"
start_line: 561
end_line: 588
---

# reader_plugin_source

- Fragment ID: `rgbdns-frag-c92aa9231edd`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 561-588
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-c92aa9231edd", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 561, "endLine": 588}
```

## Excerpt

<span id="rgbdns-frag-c92aa9231edd" class="rgbdns-fragment-target"></span>
### rgbdns-frag-c92aa9231edd: def reader_plugin_source

```python
def reader_plugin_source() -> Path:
    """Return the FirstPair Reader 2 release folder, failing if it is unusable."""
    source = FIRSTPAIR_ROOT / "publishing" / "vault" / "plugin" / READER_PLUGIN_ID
    missing = [name for name in READER_RELEASE_FILES if not (source / name).is_file()]
    if missing:
        raise SystemExit(
            f"FirstPair Reader release files missing from {source}: {', '.join(missing)} "
            "(set FIRSTPAIR_ROOT to a FirstPair checkout)"
        )
    manifest = json.loads((source / "manifest.json").read_text(encoding="utf-8"))
    if manifest.get("id") != READER_PLUGIN_ID:
        raise SystemExit(f"{source}/manifest.json has id {manifest.get('id')!r}, expected {READER_PLUGIN_ID!r}")
    bundler = FIRSTPAIR_ROOT / "publishing" / "scripts" / "bundle-obsidian-plugin.py"
    if not bundler.is_file():
        raise SystemExit(f"FirstPair plugin bundler missing: {bundler}")
    check = subprocess.run(
        [sys.executable, str(bundler), str(source), "--check"],
        capture_output=True,
        text=True,
    )
    if check.returncode != 0:
        raise SystemExit(
            "FirstPair Reader main.js is stale; rebuild it with "
            f"python3 {bundler} {source}\n{check.stdout}{check.stderr}".rstrip()
        )
    return source


```
