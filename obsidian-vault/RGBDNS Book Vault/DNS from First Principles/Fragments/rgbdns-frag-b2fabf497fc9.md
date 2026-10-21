---
type: "code-fragment"
fragment_id: "rgbdns-frag-b2fabf497fc9"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Operator model"
kind: "heading"
start_line: 45
end_line: 81
---

# Operator model

- Fragment ID: `rgbdns-frag-b2fabf497fc9`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 45-81
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-b2fabf497fc9", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 45, "endLine": 81}
```

## Excerpt

<span id="rgbdns-frag-b2fabf497fc9" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b2fabf497fc9: heading Operator model

````markdown
## Operator model

The primary remains the only writable authority. Canonical configuration is
still delivered as `rgbdns.data`. ACME state is stored separately under the
tinydns state directory:

```text
/var/lib/rgbdns/tinydns/data                 canonical activated text
/var/lib/rgbdns/tinydns/data.cdb             canonical compiled database
/var/lib/rgbdns/tinydns/acme-overlay.data    durable generated TXT overlay
/var/lib/rgbdns/tinydns/acme-serials         durable per-zone SOA serials
/var/lib/rgbdns/tinydns/acme-update.lock     cross-process publication lock
```

The generated files are rgbdns state, not conffiles. Operators must not edit
them. Challenge credentials live in `/etc/rgbdns/acme-update.conf`, owned by
root, group `rgbdns`, mode 0640. Packages preserve this file during upgrades.

The daemon constructs and serves one logical snapshot from canonical data plus
the ACME overlay. Each accepted update performs this transaction:

1. Authenticate and authorize the complete request without changing state.
2. Lock the update state and reload the durable overlay.
3. Apply additions and value-specific deletions to an in-memory candidate.
4. Advance affected zone serials monotonically.
5. Write candidate overlay and serial files beside their destinations, `fsync`
   each file, rename them atomically, and `fsync` the state directory.
6. Build and validate a new logical `Zone` snapshot.
7. Atomically replace the daemon's shared snapshot.
8. Return a signed NOERROR response.

No successful response may precede durable state and live publication. A
failure leaves both the old durable files and the old live snapshot usable.
Canonical imports take the same lock, validate the canonical candidate merged
with the current overlay, activate canonical files, and notify/restart the
daemon without deleting ACME state.

````
