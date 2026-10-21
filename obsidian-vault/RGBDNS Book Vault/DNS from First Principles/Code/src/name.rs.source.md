---
type: "code-file"
source_path: "src/name.rs"
language: "rust"
subsystem: "DNS data model and wire codec"
crate: "rgbdns"
line_count: 250
fragment_count: 33
rgbdns_commit: "5ae24772"
---

# src/name.rs

- Subsystem: [[DNS from First Principles/Subsystems/DNS data model and wire codec|DNS data model and wire codec]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]
- Source path: `src/name.rs`
- Lines: 250
- Summary: Validated DNS name.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-fd362110f0ff|Name]]: lines 15-16
- [[DNS from First Principles/Fragments/rgbdns-frag-0c521630e572|PartialEq]]: lines 17-17
- [[DNS from First Principles/Fragments/rgbdns-frag-ad7b342aa5e0|eq]]: lines 18-29
- [[DNS from First Principles/Fragments/rgbdns-frag-5aebca818d0e|Eq]]: lines 30-31
- [[DNS from First Principles/Fragments/rgbdns-frag-fe46138a7df6|Hash]]: lines 32-32
- [[DNS from First Principles/Fragments/rgbdns-frag-d375ebc0f305|hash]]: lines 33-43
- [[DNS from First Principles/Fragments/rgbdns-frag-1ecb9ab00152|PartialOrd]]: lines 44-44
- [[DNS from First Principles/Fragments/rgbdns-frag-5f4c4b9589f4|partial_cmp]]: lines 45-49
- [[DNS from First Principles/Fragments/rgbdns-frag-e4982a174651|Ord]]: lines 50-50
- [[DNS from First Principles/Fragments/rgbdns-frag-0dc862d179e7|cmp]]: lines 51-64
- [[DNS from First Principles/Fragments/rgbdns-frag-5029e4e250ec|Name]]: lines 65-65
- [[DNS from First Principles/Fragments/rgbdns-frag-695a58a3c146|root]]: lines 66-68
- [[DNS from First Principles/Fragments/rgbdns-frag-c46204f4378e|labels]]: lines 69-71
- [[DNS from First Principles/Fragments/rgbdns-frag-7c89404221eb|is_root]]: lines 72-74
- [[DNS from First Principles/Fragments/rgbdns-frag-874de4715a3a|parent]]: lines 75-77
- [[DNS from First Principles/Fragments/rgbdns-frag-d43022c23aa8|is_subdomain_of]]: lines 78-92
- [[DNS from First Principles/Fragments/rgbdns-frag-4e2018a99a96|wildcard]]: lines 93-97
- [[DNS from First Principles/Fragments/rgbdns-frag-9a4884d9edd2|from_labels]]: lines 98-101
- [[DNS from First Principles/Fragments/rgbdns-frag-bb1fd64a4514|wire_len]]: lines 102-104
- [[DNS from First Principles/Fragments/rgbdns-frag-3736c8d8aac5|suffix]]: lines 105-107
- [[DNS from First Principles/Fragments/rgbdns-frag-aa983a03bcc1|to_wire]]: lines 108-116
- [[DNS from First Principles/Fragments/rgbdns-frag-3ce4c6e253d7|without_wildcard]]: lines 117-122
- [[DNS from First Principles/Fragments/rgbdns-frag-b7c53377ea2d|with_wildcard]]: lines 123-127
- [[DNS from First Principles/Fragments/rgbdns-frag-c7f726a96454|validate]]: lines 128-140
- [[DNS from First Principles/Fragments/rgbdns-frag-1778a1a30575|FromStr]]: lines 141-141
- [[DNS from First Principles/Fragments/rgbdns-frag-8961d425e88c|Err]]: lines 142-142
- [[DNS from First Principles/Fragments/rgbdns-frag-a9f1ae579e37|from_str]]: lines 143-193
- [[DNS from First Principles/Fragments/rgbdns-frag-11e2c8b8b069|fmt]]: lines 194-194
- [[DNS from First Principles/Fragments/rgbdns-frag-2671eb38d646|fmt]]: lines 195-215
- [[DNS from First Principles/Fragments/rgbdns-frag-b756d5bb3c25|tests]]: lines 216-218
- [[DNS from First Principles/Fragments/rgbdns-frag-d9ef6352cee6|root_and_case]]: lines 219-236
- [[DNS from First Principles/Fragments/rgbdns-frag-b1201af3a743|escapes]]: lines 237-244
- [[DNS from First Principles/Fragments/rgbdns-frag-b384fa6572b4|limits]]: lines 245-250

## Full Source

```rust
use crate::{Error, Result};
use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    str::FromStr,
};

/// Validated DNS name.
///
/// Original ASCII case is retained for RFC 4343 response preservation while
/// equality, ordering, and hashing use the DNS case-insensitive comparison
/// rules. The root is represented by an empty label vector.
#[derive(Clone, Debug, Default)]
pub struct Name(Vec<Vec<u8>>);

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.0.len() == other.0.len()
            && self.0.iter().zip(&other.0).all(|(left, right)| {
                left.len() == right.len()
                    && left
                        .iter()
                        .zip(right)
                        .all(|(a, b)| a.eq_ignore_ascii_case(b))
            })
    }
}

impl Eq for Name {}

impl Hash for Name {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.len().hash(state);
        for label in &self.0 {
            label.len().hash(state);
            for byte in label {
                byte.to_ascii_lowercase().hash(state);
            }
        }
    }
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Name {
    fn cmp(&self, other: &Self) -> Ordering {
        for (left, right) in self.0.iter().zip(&other.0) {
            let ordering = left
                .iter()
                .map(u8::to_ascii_lowercase)
                .cmp(right.iter().map(u8::to_ascii_lowercase));
            if ordering != Ordering::Equal {
                return ordering;
            }
        }
        self.0.len().cmp(&other.0.len())
    }
}

impl Name {
    pub fn root() -> Self {
        Self::default()
    }
    pub fn labels(&self) -> impl Iterator<Item = &[u8]> {
        self.0.iter().map(Vec::as_slice)
    }
    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }
    pub fn parent(&self) -> Option<Self> {
        (!self.0.is_empty()).then(|| Self(self.0[1..].to_vec()))
    }
    pub fn is_subdomain_of(&self, other: &Self) -> bool {
        if self.0.len() < other.0.len() {
            return false;
        }
        self.0[self.0.len() - other.0.len()..]
            .iter()
            .zip(&other.0)
            .all(|(left, right)| {
                left.len() == right.len()
                    && left
                        .iter()
                        .zip(right)
                        .all(|(a, b)| a.eq_ignore_ascii_case(b))
            })
    }
    pub fn wildcard(&self) -> Self {
        let mut labels = self.0.clone();
        labels.insert(0, b"*".to_vec());
        Self(labels)
    }
    pub(crate) fn from_labels(labels: Vec<Vec<u8>>) -> Result<Self> {
        validate(&labels)?;
        Ok(Self(labels))
    }
    pub(crate) fn wire_len(&self) -> usize {
        1 + self.0.iter().map(|l| l.len() + 1).sum::<usize>()
    }
    pub(crate) fn suffix(&self, first_label: usize) -> Self {
        Self(self.0[first_label..].to_vec())
    }
    pub(crate) fn to_wire(&self) -> Vec<u8> {
        let mut wire = Vec::with_capacity(self.wire_len());
        for label in &self.0 {
            wire.push(label.len() as u8);
            wire.extend(label);
        }
        wire.push(0);
        wire
    }
    pub(crate) fn without_wildcard(&self) -> Option<Self> {
        self.0
            .first()
            .filter(|label| label.as_slice() == b"*")
            .map(|_| Self(self.0[1..].to_vec()))
    }
    pub(crate) fn with_wildcard(&self) -> Self {
        self.wildcard()
    }
}

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

impl FromStr for Name {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        if input.is_empty() || input == "." {
            return Ok(Self::root());
        }
        let s = input.strip_suffix('.').unwrap_or(input);
        let mut labels = Vec::new();
        let mut label = Vec::new();
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            match bytes[i] {
                b'.' => {
                    if label.is_empty() {
                        return Err(Error::InvalidName(input.into()));
                    }
                    labels.push(std::mem::take(&mut label));
                    i += 1;
                }
                b'\\' => {
                    i += 1;
                    if i == bytes.len() {
                        return Err(Error::InvalidName(input.into()));
                    }
                    if i + 2 < bytes.len() && bytes[i..i + 3].iter().all(u8::is_ascii_digit) {
                        let n = (bytes[i] - b'0') as u16 * 100
                            + (bytes[i + 1] - b'0') as u16 * 10
                            + (bytes[i + 2] - b'0') as u16;
                        if n > 255 {
                            return Err(Error::InvalidName(input.into()));
                        }
                        label.push(n as u8);
                        i += 3;
                    } else {
                        label.push(bytes[i]);
                        i += 1;
                    }
                }
                c => {
                    label.push(c);
                    i += 1;
                }
            }
        }
        if label.is_empty() {
            return Err(Error::InvalidName(input.into()));
        }
        labels.push(label);
        Self::from_labels(labels)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            return f.write_str(".");
        }
        for (i, label) in self.0.iter().enumerate() {
            if i != 0 {
                f.write_str(".")?;
            }
            for &c in label {
                match c {
                    b'.' | b'\\' => write!(f, "\\{}", c as char)?,
                    0x21..=0x7e => write!(f, "{}", c as char)?,
                    _ => write!(f, "\\{c:03}")?,
                }
            }
        }
        f.write_str(".")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn root_and_case() {
        assert_eq!(
            "WWW.Example".parse::<Name>().unwrap().to_string(),
            "WWW.Example."
        );
        assert_eq!(
            "WWW.Example".parse::<Name>().unwrap(),
            "www.example".parse::<Name>().unwrap()
        );
        assert_eq!(".".parse::<Name>().unwrap(), Name::root());
        assert!(
            "MiSsInG.Bitcoin.Science"
                .parse::<Name>()
                .unwrap()
                .is_subdomain_of(&"bitcoin.science".parse().unwrap())
        );
    }
    #[test]
    fn escapes() {
        assert_eq!(
            r"a\.b.example".parse::<Name>().unwrap().to_string(),
            r"a\.b.example."
        );
        assert!(r"\999".parse::<Name>().is_err());
    }
    #[test]
    fn limits() {
        assert!("x".repeat(64).parse::<Name>().is_err());
        let long = (0..4).map(|_| "x".repeat(63)).collect::<Vec<_>>().join(".");
        assert!(long.parse::<Name>().is_err());
    }
}
```
