---
type: "code-fragment"
fragment_id: "rgbdns-frag-d1bf855d599e"
source_path: "RGBDNS_LETS.md"
code_note: "DNS from First Principles/Code/RGBDNS_LETS.md.source"
language: "markdown"
subsystem: "Repository and build"
symbol: "Validation matrix"
kind: "heading"
start_line: 263
end_line: 295
---

# Validation matrix

- Fragment ID: `rgbdns-frag-d1bf855d599e`
- Source file: [[DNS from First Principles/Code/RGBDNS_LETS.md.source|RGBDNS_LETS.md]]
- Lines: 263-295
- Subsystem: [[DNS from First Principles/Subsystems/Repository and build|Repository and build]]

```rgbdns-fragment
{"id": "rgbdns-frag-d1bf855d599e", "codeNote": "DNS from First Principles/Code/RGBDNS_LETS.md.source", "sourcePath": "RGBDNS_LETS.md", "startLine": 263, "endLine": 295}
```

## Excerpt

<span id="rgbdns-frag-d1bf855d599e" class="rgbdns-fragment-target"></span>
### rgbdns-frag-d1bf855d599e: heading Validation matrix

````markdown
## Validation matrix

The feature is not complete until tests cover:

- RFC 2136 message decoding and response codes;
- RFC 2845 known-answer TSIG vectors and response signing;
- malformed, missing, duplicate, misplaced, expired, future, bad-key, bad-MAC,
  replayed, and unsupported-algorithm requests;
- zone, owner, type, class, TTL, TXT length, state-size, and count policy;
- atomic multi-value add, value delete, RRset delete, and prerequisites;
- simultaneous wildcard/non-wildcard values at one owner;
- concurrent update serialization;
- durable restart and canonical re-import;
- SOA serial monotonicity and wrap behavior;
- authoritative UDP/TCP lookup and integrated AXFR visibility;
- secondary refusal and primary-only configuration;
- package verification on Debian and RPM;
- Certbot-compatible RFC 2136 configuration against a local integration
  fixture and a documented opt-in Let's Encrypt staging test.

The repository validation commands remain:

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
packaging/tests/test-secondary-sync.sh
make -C ietf check
git diff --check
```

The direct public-network Let's Encrypt staging test is opt-in because it
requires delegated public DNS and consumes external service capacity.
````
