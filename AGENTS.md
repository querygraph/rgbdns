# rgbdns repository guidance

## Release versioning

- Follow semantic versioning consistently across every package and document.
- Increment the **minor** component for a new feature. Examples include a new
  DNS behavior, command, transfer capability, service role, or package
  workflow. ANAME preservation over AXFR is the feature release `0.3.0`.
- Increment the **patch** component only for compatible bug fixes, security
  fixes, documentation corrections, and packaging repairs that add no new
  behavior.
- Reserve the **major** component for incompatible public behavior or format
  changes once the public surface is stable.
- Keep `Cargo.toml`, the rgbdns entry in `Cargo.lock`, `debian/changelog`,
  `packaging/rpm/rgbdns.spec`, installed manual version strings, examples,
  deployment guides, and release documentation synchronized.
- Debian package versions use the software version directly. RPM feature and
  bug releases start at release `1`; increment the RPM release only for a
  packaging-only rebuild of the same software version.

## Toolchain

- Use the repository `.tool-versions` through asdf.
- Use uv/uvx for Python tools; do not create ad-hoc pip virtual environments.
- The RFC build is `make -C ietf check`.
- The FirstPair book builder enforces its own locked publishing toolchain. Do
  not silently regenerate committed book artifacts with mismatched tools.

## ANAME and zone transfer

- rgbdns source uses the private tinydns directive
  `Aowner.example:target.example:ttl-cap`.
- rgbdns peers preserve that directive through an explicitly negotiated
  experimental AXFR extension:
  - EDNS option `65001`;
  - capability token `RGA1`;
  - private-use RR TYPE `65401`;
  - RDATA `RGA1` followed by the uncompressed target DNS name;
  - the private RR TTL carries the configured ANAME ceiling.
- TYPE65401 collides with PowerDNS's incompatible private ALIAS encoding.
  Never infer semantics from TYPE65401 without the `RGA1` negotiation and
  payload validation.
- Standard AXFR clients receive no rgbdns private ANAME metadata. BuddyNS and
  other ordinary secondaries cannot reproduce an ANAME-backed zone. Delegate
  such zones only to upgraded rgbdns authorities unless the zone uses standard
  A/AAAA records.
- The published Internet-Draft requests IANA-assigned ANAME and EDNS values as
  `TBD1` and `TBD2`. Do not deploy guessed values. Keep the experimental
  protocol until assignments exist and a versioned migration is implemented.
- Upgrade both primary and secondary for ANAME transfer changes. The secondary
  resolves transferred ANAME targets independently.
- ANAME target lookups are bounded, concurrent misses are coalesced, and
  failures are briefly suppressed to limit recursive retry and cross-provider
  loop amplification.

## Deployment and packaging invariants

- Debian configuration under `/etc/rgbdns` is a conffile and must preserve
  operator changes during upgrade.
- RPM configuration uses `%config(noreplace)` and must preserve operator
  changes during upgrade.
- Systemd units use `RuntimeDirectory=rgbdns`; do not assume `/run/rgbdns`
  exists while the one-shot secondary service is inactive.
- Primary data is imported atomically from `rgbdns.data`.
- Secondary zone lists are imported atomically from `rgbdns.zones`, one zone
  per line. Per-zone synchronization failures retain last-known-good data and
  must not prevent healthy zones from updating.
- The known deployment uses:
  - `a.ns.cron.sh`, public `52.10.53.234`, as the Debian primary;
  - primary VPC address `172.31.60.189` for secondary AXFR;
  - `b.ns.cron.sh`, public `52.38.177.160`, as the openSUSE secondary.
- AXFR allow-lists contain the actual source addresses of secondaries. BuddyNS
  uses public addresses; the EC2 peer should use the VPC path.

## Standards document

- Published revision `-00` is stored in
  `ietf/draft-khrabrov-dnsop-aname-axfr-00.xml`; its adjacent `.txt` and
  `.html` files are the matching publication artifacts.
- Published draft revisions are immutable. Never rewrite a submitted revision
  to reflect later software releases. Copy the latest XML to the next revision,
  update `docName`, revision history, and implementation status there, then
  regenerate matching `.txt` and `.html` artifacts.
- Published draft:
  <https://datatracker.ietf.org/doc/draft-khrabrov-dnsop-aname-axfr/>.
- The draft is an active individual Internet-Draft, not an endorsed RFC.
- Keep implementation claims accurate as of each draft revision, regenerate
  artifacts, and run Datatracker-compatible validation before submitting the
  new revision.

## Blog textpack delivery

- Follow the FirstPair blog textpack guide at
  `~/src/firstpair/publishing/skills/blog-textpack-delivery.md`. A Markdown
  draft or copy pack is not the requested textpack deliverable.
- Keep the canonical post at `docs/blog/<slug>/post.md` and all referenced
  local images inside that post directory. The post must reference the images
  with relative Markdown paths so the builder can bundle them.
- Build the Obsidian/Omnighost-importable zipped TextBundle with:

  ```sh
  REPO_ROOT=/path/to/rgbdns \
  BLOG_DOMAIN=firstpair.press \
  ~/src/firstpair/publishing/scripts/publish-versioned-blog.sh \
    docs/blog/<slug>
  ```

- The repository handoff is
  `docs/blog/<slug>/dist/<slug>.textpack`. Keep only the zipped `.textpack`,
  not an unpacked `.textbundle/`, and record its stable and versioned names in
  `docs/blog/<slug>/dist/VERSION.md`.
- Keep the versioned symlink in the repository alongside the stable pack. Copy
  the versioned pack to `~/icloud/blogs` and verify that it is byte-identical
  to the stable repository artifact with `cmp`.
- Validate that the archive contains `text.markdown`, `info.json`, and every
  referenced image under `assets/`. Confirm that `info.json` contains the
  intended blog domain, slug, tags, payload SHA-256, and Git provenance.
- The FirstPair builder may commit the canonical post and referenced assets to
  establish provenance. Preserve unrelated staged and working-tree changes,
  and report the resulting commits and delivery paths.

## Required validation

For code, protocol, packaging, or release changes, run as applicable:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
packaging/tests/test-secondary-sync.sh
make -C ietf check
git diff --check
```

The direct public-network DNSSEC test remains opt-in.
