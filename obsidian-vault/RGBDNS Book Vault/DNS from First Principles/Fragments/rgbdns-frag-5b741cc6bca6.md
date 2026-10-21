---
type: "code-fragment"
fragment_id: "rgbdns-frag-5b741cc6bca6"
source_path: "scripts/test_obsidian_vault.py"
code_note: "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source"
language: "python"
subsystem: "Developer scripts"
symbol: "test_markdown_fences_keep_full_source_whole"
kind: "def"
start_line: 61
end_line: 70
---

# test_markdown_fences_keep_full_source_whole

- Fragment ID: `rgbdns-frag-5b741cc6bca6`
- Source file: [[DNS from First Principles/Code/scripts/test_obsidian_vault.py.source|scripts/test_obsidian_vault.py]]
- Lines: 61-70
- Subsystem: [[DNS from First Principles/Subsystems/Developer scripts|Developer scripts]]

```rgbdns-fragment
{"id": "rgbdns-frag-5b741cc6bca6", "codeNote": "DNS from First Principles/Code/scripts/test_obsidian_vault.py.source", "sourcePath": "scripts/test_obsidian_vault.py", "startLine": 61, "endLine": 70}
```

## Excerpt

<span id="rgbdns-frag-5b741cc6bca6" class="rgbdns-fragment-target"></span>
### rgbdns-frag-5b741cc6bca6: def test_markdown_fences_keep_full_source_whole

```python
    def test_markdown_fences_keep_full_source_whole(self) -> None:
        lines = ["# Title", "", "```sh", "make", "```", "", "after", "````", "x", "````", "end"]
        fence = builder.fence_for(lines)
        note = "\n".join(["# f", "", "## Full Source", "", f"{fence}markdown", *lines, fence])
        excerpt = checker.reader_excerpt_lines(note)
        self.assertIsNotNone(excerpt)
        self.assertEqual(excerpt, lines, "source lines stay verbatim inside the longer fence")
        self.assertEqual(excerpt[6], "after")
        self.assertGreater(len(fence), 4)

```
