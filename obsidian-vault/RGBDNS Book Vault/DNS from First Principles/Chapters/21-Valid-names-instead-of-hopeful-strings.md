---
type: "chapter"
source_file: "docs/book/rgbdns.md"
---

# Valid names instead of hopeful strings

[`Name`](../../src/name.rs) is the first load-bearing abstraction. It stores a
sequence of byte labels rather than a UTF-8 domain string. Its constructor is
private to the module; all construction passes through parsing or
`from_labels`, and both reach the same validation rule:

```rust
fn validate(labels: &[Vec<u8>]) -> Result<()> {
    if labels.iter().any(|l| l.is_empty() || l.len() > 63) {
        return Err(Error::InvalidName(
            "label must contain 1..=63 octets".into(),
        ));
    }
    let len = 1 + labels.iter().map(|l| l.len() + 1).sum::<usize>();
    if len > 255 {
        return Err(Error::InvalidName("wire name exceeds 255 octets".into()));
    }
    Ok(())
}
```

That small private function changes the rest of the codebase. `Zone` can use
`Name` as a `BTreeMap` key without rechecking label lengths. The packet writer
can calculate `wire_len` without wondering whether it will overflow the DNS
name limit. `parent`, `suffix`, `wildcard`, and `is_subdomain_of` operate on
labels rather than fragile dotted-string suffixes.

DNS identity is case-insensitive but responses should preserve the query’s
case. `Name` therefore retains original bytes while implementing `Eq`, `Hash`,
and `Ord` with ASCII-folded comparisons. In C this invariant must be remembered
by every hash table and comparison call site. Here it belongs to the key type.

This is a zero-surprise form of abstraction. It adds allocations when a name
is built, but it removes repeated parsing and validation later. The benchmarked
hot paths operate on already validated values, while the network boundary
absorbs the cost once.

## Generated Code Fragment Index

These generated links open the collocated rgbdns codebase notes.

```rgbdns-fragment
{"id": "rgbdns-frag-fd362110f0ff", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-fd362110f0ff: struct Name", "sourcePath": "src/name.rs", "startLine": 15, "endLine": 16}
```

```rgbdns-fragment
{"id": "rgbdns-frag-0c521630e572", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-0c521630e572: impl PartialEq", "sourcePath": "src/name.rs", "startLine": 17, "endLine": 17}
```

```rgbdns-fragment
{"id": "rgbdns-frag-ad7b342aa5e0", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-ad7b342aa5e0: fn eq", "sourcePath": "src/name.rs", "startLine": 18, "endLine": 29}
```

```rgbdns-fragment
{"id": "rgbdns-frag-5aebca818d0e", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-5aebca818d0e: impl Eq", "sourcePath": "src/name.rs", "startLine": 30, "endLine": 31}
```

```rgbdns-fragment
{"id": "rgbdns-frag-fe46138a7df6", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-fe46138a7df6: impl Hash", "sourcePath": "src/name.rs", "startLine": 32, "endLine": 32}
```

```rgbdns-fragment
{"id": "rgbdns-frag-d375ebc0f305", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-d375ebc0f305: fn hash", "sourcePath": "src/name.rs", "startLine": 33, "endLine": 43}
```

```rgbdns-fragment
{"id": "rgbdns-frag-1ecb9ab00152", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-1ecb9ab00152: impl PartialOrd", "sourcePath": "src/name.rs", "startLine": 44, "endLine": 44}
```

```rgbdns-fragment
{"id": "rgbdns-frag-5f4c4b9589f4", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-5f4c4b9589f4: fn partial_cmp", "sourcePath": "src/name.rs", "startLine": 45, "endLine": 49}
```

```rgbdns-fragment
{"id": "rgbdns-frag-e4982a174651", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-e4982a174651: impl Ord", "sourcePath": "src/name.rs", "startLine": 50, "endLine": 50}
```

```rgbdns-fragment
{"id": "rgbdns-frag-0dc862d179e7", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-0dc862d179e7: fn cmp", "sourcePath": "src/name.rs", "startLine": 51, "endLine": 64}
```

```rgbdns-fragment
{"id": "rgbdns-frag-5029e4e250ec", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-5029e4e250ec: impl Name", "sourcePath": "src/name.rs", "startLine": 65, "endLine": 65}
```

```rgbdns-fragment
{"id": "rgbdns-frag-695a58a3c146", "codeNote": "DNS from First Principles/Code/src/name.rs.source", "heading": "rgbdns-frag-695a58a3c146: fn root", "sourcePath": "src/name.rs", "startLine": 66, "endLine": 68}
```
