# FirstPair Reader

The offline reader for every FirstPair vault. It makes no network requests,
and it writes nothing to the vault unless you switch on its editing tools.
With the plugin disabled, the vault's Markdown notes remain a complete book.

## What it reads

The Reader finds the vault's index on its own: `_data/reader.json` (schema
`firstpair-reader.v2`), an aligned edition's `_data/parallel-reader.json`, a
1.x page list, a Cicero-style `reader.json`, a `Reader/reader-index.json`, a
mobile edition library, or a code book (`_data/fragments.json` beside a
`Chapters/` folder). See `contract.js`; `FEATURES.md` maps every feature of the
readers this one replaced.

## Reading

- **Rail** — Previous · Up · Back · Top · TOC · Next (plus Previous word and
  Next word when the vault has dictionaries), at the bottom or the top.
  Previous and Next name their destinations. Up climbs page → part →
  contents → library; TOC shows the contents with your page marked; Back
  returns to exactly where you were, including the footnote you came from;
  Top can be undone with Back. Every control is also a command.
- **Static notes open the Reader** at the same place: the launcher, contents,
  part, and page notes.
- **Libraries** list editions by family; pages are cut from the Part notes.
- **Footnotes, headings, and book links** stay inside the Reader. Source
  passages open in place or, with the setting, in a new tab. A quotation that
  links one reviewed source passage becomes a rail: tap the whole block.
- **Find the source of the selected quote** (command) matches a quotation
  against the vault's reviewed quote index.
- **Code books** show fragment cards; Show code lists the numbered lines,
  Open file opens the code note with those lines marked.

## Languages and dictionaries

Aligned editions put the source text first. Switch translations on and off,
rotate a language's translations from its header, or add a second column of
the same language. Layout follows the screen (Auto), or stays in Columns or
Stacked. On a phone, Columns become tabs: tap one or swipe the text, and the
chosen language opens first on every page.

Tap a source word, or step through words with the double chevrons, for its
dictionary entries in every shown language. Multilingual source tables get one
language bar per note; at least one language stays on.

## Editing tools (optional)

Settings → **Editing tools** adds a **Sentence & words** toggle to each page.
It puts a **Words** button beside every paragraph, list item, and heading;
pick words (tap or drag on a phone, checkboxes on a Mac) and attach a note:
typed, recorded with **🎙 Voice**, or both. Noted spans are coloured, up to
three may overlap (overlapping ones in different colours, shared words
striped; voice notes in their own green/pink/yellow palette), and tapping a
coloured word reopens its note. This is the same
editor module FirstPair Triptych uses (`shared/span-editor.js`), so the two
behave alike. A recording is saved and verified
under `Preferences/voice/` before anything refers to it; transcription (local
`mlx_whisper` on a Mac, or an OpenAI-compatible service) follows and never
replaces it. Notes go to the shared `Preferences/triptych-decisions.json` with
its `.sha256` check and `EDITING-INSTRUCTIONS.md` digest, the same files
FirstPair Triptych uses. A note whose text has changed is listed below the
page, never moved onto other words.

## For builders

Author `main-source.js` and its modules, then bundle the single release file:

```sh
python3 publishing/scripts/bundle-obsidian-plugin.py publishing/vault/plugin/firstpair-reader
```

The bundle embeds the shared modules (`../shared/*.js`, including the span
editor, and FirstPair Triptych's `review-state.js`), and `styles-source.css`
is expanded with `../shared/span-editor.css` into `styles.css`. Vaults install `manifest.json`, `main.js`,
`styles.css`, and this README, disabled by default.
