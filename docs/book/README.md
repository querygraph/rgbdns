# Book source

Build the complete book from the repository root:

```sh
docs/book/build.sh
```

The wrapper follows the FirstPair repository contract: it delegates to the
central builder when present and otherwise uses the checked-in Pandoc/Typst
fallback. Both paths treat `rgbdns.md` and `book.build.json` as canonical
source. A build creates `rgbdns.pdf`, `rgbdns.epub`, `rgbdns.html`, and
`VERSION.md` in `docs/book/dist/`; it never publishes them.

Validate the generated package with:

```sh
docs/book/validate.sh
```
