---
type: "code-fragment"
fragment_id: "rgbdns-frag-40627baa9dd5"
source_path: "src/conf.rs"
code_note: "DNS from First Principles/Code/src/conf.rs.source"
language: "rust"
subsystem: "Operations and supervision"
crate: "rgbdns"
symbol: "run_script"
kind: "fn"
start_line: 208
end_line: 222
---

# run_script

- Fragment ID: `rgbdns-frag-40627baa9dd5`
- Source file: [[DNS from First Principles/Code/src/conf.rs.source|src/conf.rs]]
- Lines: 208-222
- Subsystem: [[DNS from First Principles/Subsystems/Operations and supervision|Operations and supervision]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]

```rgbdns-fragment
{"id": "rgbdns-frag-40627baa9dd5", "codeNote": "DNS from First Principles/Code/src/conf.rs.source", "heading": "rgbdns-frag-40627baa9dd5: fn run_script", "sourcePath": "src/conf.rs", "startLine": 208, "endLine": 222}
```

## Excerpt

<span id="rgbdns-frag-40627baa9dd5" class="rgbdns-fragment-target"></span>
### rgbdns-frag-40627baa9dd5: fn run_script

```rust
fn run_script(directory: &Path, user: &str, binary: &str) -> Result<String> {
    Ok(format!(
        "#!/bin/sh\nset -eu\nROOT=$(cat {dir}/env/ROOT)\nIP=$(cat {dir}/env/IP)\nexport ROOT IP DATA=data.cdb\n\
         [ ! -f {dir}/env/BASE ] || export BASE=$(cat {dir}/env/BASE)\n\
         [ ! -f {dir}/env/ROOTS ] || export ROOTS=$(cat {dir}/env/ROOTS)\n\
         [ ! -f {dir}/env/CACHESIZE ] || export CACHESIZE=$(cat {dir}/env/CACHESIZE)\n\
         [ ! -f {dir}/env/ALLOW_NETS ] || export ALLOW_NETS=$(cat {dir}/env/ALLOW_NETS)\n\
         cd \"$ROOT\"\nexec {setuidgid} {user} {binary}\n",
        dir = shell_quote(&directory.to_string_lossy()),
        user = shell_quote(user),
        binary = shell_quote(&executable(binary)?.to_string_lossy()),
        setuidgid = shell_quote(&executable("setuidgid")?.to_string_lossy()),
    ))
}

```
