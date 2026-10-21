---
type: "code-fragment"
fragment_id: "rgbdns-frag-32521dbf2817"
source_path: "scripts/check-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "check_reader_plugin"
kind: "def"
start_line: 123
end_line: 161
---

# check_reader_plugin

- Fragment ID: `rgbdns-frag-32521dbf2817`
- Source file: [[DNS from First Principles/Code/scripts/check-obsidian-vault.py.source|scripts/check-obsidian-vault.py]]
- Lines: 123-161
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-32521dbf2817", "codeNote": "DNS from First Principles/Code/scripts/check-obsidian-vault.py.source", "sourcePath": "scripts/check-obsidian-vault.py", "startLine": 123, "endLine": 161}
```

## Excerpt

<span id="rgbdns-frag-32521dbf2817" class="rgbdns-fragment-target"></span>
### rgbdns-frag-32521dbf2817: def check_reader_plugin

```python
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


```
