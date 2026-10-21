---
type: "code-fragment"
fragment_id: "rgbdns-frag-b4805cb0b9fc"
source_path: "docs/RGBDNS_SETUP.md"
code_note: "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source"
language: "markdown"
subsystem: "Documentation"
symbol: "3. Download the current packages"
kind: "heading"
start_line: 81
end_line: 128
---

# 3. Download the current packages

- Fragment ID: `rgbdns-frag-b4805cb0b9fc`
- Source file: [[DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source|docs/RGBDNS_SETUP.md]]
- Lines: 81-128
- Subsystem: [[DNS from First Principles/Subsystems/Documentation|Documentation]]

```rgbdns-fragment
{"id": "rgbdns-frag-b4805cb0b9fc", "codeNote": "DNS from First Principles/Code/docs/RGBDNS_SETUP.md.source", "sourcePath": "docs/RGBDNS_SETUP.md", "startLine": 81, "endLine": 128}
```

## Excerpt

<span id="rgbdns-frag-b4805cb0b9fc" class="rgbdns-fragment-target"></span>
### rgbdns-frag-b4805cb0b9fc: heading 3. Download the current packages

````markdown
## 3. Download the current packages

Run these commands on a workstation with `gh` authenticated to GitHub. They
select the newest successful `master` run through the API, without depending
on a `gh run list --status` option.

Download the Debian artifact:

```sh
DEB_RUN=$(
  gh api --method GET \
    repos/querygraph/rgbdns/actions/workflows/build-deb.yml/runs \
    -f branch=master -f status=success -f per_page=1 \
    --jq '.workflow_runs[0].id'
)

mkdir -p "$HOME/rgbdns-deb"
gh run download "$DEB_RUN" \
  -R querygraph/rgbdns \
  -n rgbdns-debian-amd64 \
  -D "$HOME/rgbdns-deb"
```

Download the openSUSE artifact:

```sh
RPM_RUN=$(
  gh api --method GET \
    repos/querygraph/rgbdns/actions/workflows/build-rpm.yml/runs \
    -f branch=master -f status=success -f per_page=1 \
    --jq '.workflow_runs[0].id'
)

mkdir -p "$HOME/rgbdns-rpm"
gh run download "$RPM_RUN" \
  -R querygraph/rgbdns \
  -n rgbdns-opensuse-leap16-x86_64 \
  -D "$HOME/rgbdns-rpm"
```

The 0.6.5 artifacts are:

```text
rgbdns_0.6.5_amd64.deb
RPMS/x86_64/rgbdns-0.6.5-1.x86_64.rpm
SRPMS/rgbdns-0.6.5-1.src.rpm
```

````
