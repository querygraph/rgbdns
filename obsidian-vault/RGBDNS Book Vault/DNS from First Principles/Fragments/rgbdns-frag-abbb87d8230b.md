---
type: "code-fragment"
fragment_id: "rgbdns-frag-abbb87d8230b"
source_path: "scripts/build-obsidian-vault.py"
code_note: "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "copy_plugin"
kind: "def"
start_line: 621
end_line: 637
---

# copy_plugin

- Fragment ID: `rgbdns-frag-abbb87d8230b`
- Source file: [[DNS from First Principles/Code/scripts/build-obsidian-vault.py.source|scripts/build-obsidian-vault.py]]
- Lines: 621-637
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-abbb87d8230b", "codeNote": "DNS from First Principles/Code/scripts/build-obsidian-vault.py.source", "sourcePath": "scripts/build-obsidian-vault.py", "startLine": 621, "endLine": 637}
```

## Excerpt

<span id="rgbdns-frag-abbb87d8230b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-abbb87d8230b: def copy_plugin

```python
def copy_plugin(output: Path, source: Path, previous_plugins: list[str] | None) -> None:
    plugins = output / ".obsidian" / "plugins"
    for legacy in LEGACY_PLUGIN_IDS:
        if (plugins / legacy).exists():
            shutil.rmtree(plugins / legacy)
    target = plugins / READER_PLUGIN_ID
    if target.exists():
        shutil.rmtree(target)
    target.mkdir(parents=True)
    for name in READER_RELEASE_FILES:
        shutil.copyfile(source / name, target / name)
    write_text(
        output / ".obsidian" / "community-plugins.json",
        json.dumps(migrate_plugin_list(previous_plugins), indent=2),
    )


```
