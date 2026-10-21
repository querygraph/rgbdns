---
type: "code-fragment"
fragment_id: "rgbdns-frag-bc29009fe2bf"
source_path: "docs/OPENSUSE.md"
code_note: "DNS from First Principles/Code/docs/OPENSUSE.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "Build the RPM"
kind: "heading"
start_line: 48
end_line: 92
---

# Build the RPM

- Fragment ID: `rgbdns-frag-bc29009fe2bf`
- Source file: [[DNS from First Principles/Code/docs/OPENSUSE.md.source|docs/OPENSUSE.md]]
- Lines: 48-92
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-bc29009fe2bf", "codeNote": "DNS from First Principles/Code/docs/OPENSUSE.md.source", "sourcePath": "docs/OPENSUSE.md", "startLine": 48, "endLine": 92}
```

## Excerpt

<span id="rgbdns-frag-bc29009fe2bf" class="rgbdns-fragment-target"></span>
### rgbdns-frag-bc29009fe2bf: heading Build the RPM

````markdown
## Build the RPM

On a Leap 16.0 x86_64 build host:

```sh
sudo zypper --non-interactive install \
  git cargo rust python3 rpm-build systemd-rpm-macros
git clone https://github.com/querygraph/rgbdns.git
cd rgbdns
packaging/build-rpm.sh
rpm -qip dist/rpmbuild/RPMS/x86_64/rgbdns-0.6.5-1.x86_64.rpm
rpm -qlp dist/rpmbuild/RPMS/x86_64/rgbdns-0.6.5-1.x86_64.rpm
```

`build-rpm.sh` creates a clean rpmbuild tree under `dist/rpmbuild`, archives
the tracked and non-ignored working tree, and builds both source and binary RPMs. The spec uses
`cargo build --release --locked --bins` and runs the complete release test
suite. It discovers binary targets through Cargo metadata rather than keeping
a second program list.

The `Build openSUSE RPM` GitHub Actions workflow performs the same build in an
openSUSE Leap 16.0 container, installs and verifies the RPM, and publishes the
binary and source packages as the
`rgbdns-opensuse-leap16-x86_64` workflow artifact:

```sh
gh workflow run build-rpm.yml --ref master
gh run watch
gh run download RUN_ID \
  -n rgbdns-opensuse-leap16-x86_64 \
  -D dist/cloud-rpm
```

Inspect an artifact before installation:

```sh
rpm -K dist/cloud-rpm/rgbdns-0.6.5-1.x86_64.rpm
rpm -qip dist/cloud-rpm/rgbdns-0.6.5-1.x86_64.rpm
rpm -qlp dist/cloud-rpm/rgbdns-0.6.5-1.x86_64.rpm
```

The development RPM is not repository-signed. `rpm -K` still verifies the
embedded payload digest, but production distribution should place it in a
signed, access-controlled RPM repository.

````
