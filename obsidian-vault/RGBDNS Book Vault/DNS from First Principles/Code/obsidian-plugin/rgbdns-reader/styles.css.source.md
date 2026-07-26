---
type: "code-file"
source_path: "obsidian-plugin/rgbdns-reader/styles.css"
language: "css"
subsystem: "Repository and build"
line_count: 48
fragment_count: 1
rgbdns_commit: "79502939"
---

# obsidian-plugin/rgbdns-reader/styles.css

- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]
- Source path: `obsidian-plugin/rgbdns-reader/styles.css`
- Lines: 48
- Summary: Source file in the Repository and build subsystem.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-2f4368432de7|styles.css]]: lines 1-48

## Full Source

```css
.rgbdns-fragment-card {
  border: 1px solid color-mix(in srgb, var(--interactive-accent) 35%, var(--background-modifier-border));
  border-left: 4px solid var(--interactive-accent);
  border-radius: 8px;
  padding: 0.75rem 0.85rem;
  margin: 0.8rem 0;
  background: color-mix(in srgb, var(--interactive-accent) 5%, var(--background-secondary));
}

.rgbdns-fragment-title {
  font-family: var(--font-monospace);
  font-size: 0.86rem;
  font-weight: 700;
  margin-bottom: 0.25rem;
}

.rgbdns-fragment-meta {
  color: var(--text-muted);
  font-family: var(--font-monospace);
  font-size: 0.78rem;
  margin-bottom: 0.45rem;
}

.rgbdns-fragment-card button {
  cursor: pointer;
  border-radius: 999px;
  padding-inline: 0.8rem;
}

.rgbdns-fragment-highlight {
  outline: 2px solid var(--interactive-accent);
  outline-offset: 4px;
  background: color-mix(in srgb, var(--interactive-accent) 12%, transparent);
}

.rgbdns-reader-status {
  font-variant-numeric: tabular-nums;
}

body.rgbdns-reading .markdown-reading-view .markdown-preview-sizer,
body.rgbdns-reading .markdown-source-view .cm-contentContainer {
  max-width: 760px;
  margin-inline: auto;
}

body.rgbdns-reading .markdown-preview-view {
  line-height: 1.68;
}
```
