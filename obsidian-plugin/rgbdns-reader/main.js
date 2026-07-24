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
    await this.app.workspace.getLeaf(false).openFile(next);
  }

  updateReadingStatus(file) {
    const index = this.chapters.findIndex((chapter) => chapter.path === file?.path);
    const reading = index >= 0;
    document.body.classList.toggle("rgbdns-reading", reading);
    this.status.setText(reading
      ? `DNS from First Principles · ${index + 1}/${this.chapters.length}`
      : "RGBDNS reader");
  }

  async loadFragments() {
    try {
      const text = await this.app.vault.adapter.read(normalizePath(DATA_PATH));
      for (const row of JSON.parse(text)) {
        this.fragments.set(row.id, {
          id: row.id,
          codeNote: row.code_note,
          heading: row.heading,
          symbol: row.symbol,
          sourcePath: row.source_path,
          startLine: row.start_line,
          endLine: row.end_line,
        });
      }
    } catch (error) {
      console.warn("rgbdns-reader: could not load fragments", error);
    }
  }

  async openFragment(fragment, sourcePath) {
    if (!fragment || !fragment.codeNote) {
      new Notice("No code note for this fragment.");
      return;
    }
    window.localStorage.setItem(STORAGE_KEY, fragment.id);
    const link = fragment.heading
      ? `${fragment.codeNote}#${fragment.heading}`
      : fragment.codeNote;
    await this.app.workspace.openLinkText(link, sourcePath || "");
    window.setTimeout(() => this.highlightRequestedFragment(), 350);
  }

  highlightRequestedFragment() {
    const id = window.localStorage.getItem(STORAGE_KEY);
    if (!id) return;
    document.querySelectorAll(".rgbdns-fragment-highlight").forEach((el) => {
      el.classList.remove("rgbdns-fragment-highlight");
    });
    const target = document.getElementById(id);
    if (!target) return;
    const section = target.closest(".markdown-preview-section") || target.parentElement;
    if (section) section.classList.add("rgbdns-fragment-highlight");
    target.scrollIntoView({ behavior: "smooth", block: "center" });
  }
}

module.exports = RgbdnsReaderPlugin;
