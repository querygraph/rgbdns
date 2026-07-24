---
type: "code-fragment"
fragment_id: "rgbdns-frag-50588c7cfc6b"
source_path: "obsidian-plugin/rgbdns-reader/main.js"
code_note: "DNS from First Principles/Code/obsidian-plugin/rgbdns-reader/main.js.source"
language: "javascript"
subsystem: "Repository and build"
symbol: "main.js"
kind: "file"
start_line: 1
end_line: 81
---

# main.js

- Fragment ID: `rgbdns-frag-50588c7cfc6b`
- Source file: [[DNS from First Principles/Code/obsidian-plugin/rgbdns-reader/main.js.source|obsidian-plugin/rgbdns-reader/main.js]]
- Lines: 1-81
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-50588c7cfc6b", "codeNote": "DNS from First Principles/Code/obsidian-plugin/rgbdns-reader/main.js.source", "heading": "rgbdns-frag-50588c7cfc6b: file main.js", "sourcePath": "obsidian-plugin/rgbdns-reader/main.js", "startLine": 1, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-50588c7cfc6b" class="rgbdns-fragment-target"></span>
### rgbdns-frag-50588c7cfc6b: file main.js

```javascript
const { Plugin, Notice, normalizePath } = require("obsidian");

const BOOK_ROOT = "DNS from First Principles";
const DATA_PATH = `${BOOK_ROOT}/_data/fragments.json`;
const STORAGE_KEY = "rgbdns-fragment-highlight";

class RgbdnsReaderPlugin extends Plugin {
  async onload() {
    this.fragments = new Map();
    this.chapters = [];
    await this.loadFragments();
    this.loadChapters();
    this.status = this.addStatusBarItem();
    this.status.addClass("rgbdns-reader-status");
    this.status.setText("RGBDNS reader");
    this.addRibbonIcon("book-open", "Open DNS from First Principles", () => {
      this.app.workspace.openLinkText(`${BOOK_ROOT}/Book`, "");
    });

    this.registerMarkdownCodeBlockProcessor("rgbdns-fragment", async (source, el, ctx) => {
      let payload;
      try {
        payload = JSON.parse(source.trim());
      } catch (error) {
        el.createEl("pre", { text: `Invalid rgbdns-fragment payload: ${error.message}` });
        return;
      }
      const fragment = this.fragments.get(payload.id) || payload;
      const card = el.createDiv({ cls: "rgbdns-fragment-card" });
      const title = card.createDiv({ cls: "rgbdns-fragment-title" });
      title.createSpan({ text: fragment.symbol || fragment.id || "unknown fragment" });
      const meta = card.createDiv({ cls: "rgbdns-fragment-meta" });
      meta.setText(`${fragment.sourcePath || fragment.source_path || ""}:${fragment.startLine || fragment.start_line || "?"}-${fragment.endLine || fragment.end_line || "?"}`);
      const button = card.createEl("button", { text: "Open code fragment" });
      button.addEventListener("click", async () => {
        await this.openFragment(fragment, ctx.sourcePath);
      });
    });

    this.addCommand({
      id: "open-rgbdns-book",
      name: "Open book map",
      callback: () => this.app.workspace.openLinkText(`${BOOK_ROOT}/Book`, ""),
    });
    this.addCommand({
      id: "next-rgbdns-chapter",
      name: "Open next chapter",
      callback: () => this.moveChapter(1),
    });
    this.addCommand({
      id: "previous-rgbdns-chapter",
      name: "Open previous chapter",
      callback: () => this.moveChapter(-1),
    });

    this.registerEvent(this.app.workspace.on("file-open", (file) => {
      this.updateReadingStatus(file);
      window.setTimeout(() => this.highlightRequestedFragment(), 250);
    }));
  }

  onunload() {
    document.body.classList.remove("rgbdns-reading");
  }

  loadChapters() {
    this.chapters = this.app.vault
      .getMarkdownFiles()
      .filter((file) => file.path.startsWith(`${BOOK_ROOT}/Chapters/`))
      .sort((left, right) => left.path.localeCompare(right.path));
  }

  async moveChapter(delta) {
    const active = this.app.workspace.getActiveFile();
    let index = this.chapters.findIndex((file) => file.path === active?.path);
    if (index < 0) index = delta > 0 ? -1 : this.chapters.length;
    const next = this.chapters[index + delta];
    if (!next) {
      new Notice(delta > 0 ? "This is the last chapter." : "This is the first chapter.");
      return;
    }
```
