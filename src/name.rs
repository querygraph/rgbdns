use crate::{Error, Result};
use std::{fmt, str::FromStr};

/// Canonical DNS name. Labels are stored lower-case because DNS matching is
/// ASCII case-insensitive. The root is represented by an empty label vector.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(Vec<Vec<u8>>);

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
        self.0.len() >= other.0.len() && self.0[self.0.len() - other.0.len()..] == other.0
    }
    pub fn wildcard(&self) -> Self {
        let mut labels = self.0.clone();
        labels.insert(0, b"*".to_vec());
        Self(labels)
    }
    pub(crate) fn from_labels(labels: Vec<Vec<u8>>) -> Result<Self> {
        validate(&labels)?;
        Ok(Self(
            labels
                .into_iter()
                .map(|mut l| {
                    l.make_ascii_lowercase();
                    l
                })
                .collect(),
        ))
    }
    pub(crate) fn wire_len(&self) -> usize {
        1 + self.0.iter().map(|l| l.len() + 1).sum::<usize>()
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
                    label.push(c.to_ascii_lowercase());
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
            "www.example."
        );
        assert_eq!(".".parse::<Name>().unwrap(), Name::root());
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
