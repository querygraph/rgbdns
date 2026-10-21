---
type: "code-file"
source_path: "src/zone.rs"
language: "rust"
subsystem: "Authoritative service"
crate: "rgbdns"
line_count: 1348
fragment_count: 66
rgbdns_commit: "5ae24772"
---

# src/zone.rs

- Subsystem: [[DNS from First Principles/Subsystems/Authoritative service|Authoritative service]]
- Component: [[DNS from First Principles/Components/rgbdns|rgbdns]]
- Source path: `src/zone.rs`
- Lines: 1348
- Summary: The later of two SOA serials in RFC 1982 serial-number order.

## Extracted Fragments

- [[DNS from First Principles/Fragments/rgbdns-frag-ebf4f59215f2|Zone]]: lines 11-24
- [[DNS from First Principles/Fragments/rgbdns-frag-e1d80abe1755|Aname]]: lines 25-30
- [[DNS from First Principles/Fragments/rgbdns-frag-5c488af2e254|RecordMetadata]]: lines 31-34
- [[DNS from First Principles/Fragments/rgbdns-frag-d3d879f22833|MaterializationInput]]: lines 35-36
- [[DNS from First Principles/Fragments/rgbdns-frag-79ea8b5692b8|newer_serial]]: lines 37-44
- [[DNS from First Principles/Fragments/rgbdns-frag-37e0a52d0465|Zone]]: lines 45-47
- [[DNS from First Principles/Fragments/rgbdns-frag-2a770356b45a|with_acme_overlay]]: lines 48-99
- [[DNS from First Principles/Fragments/rgbdns-frag-fc2281affe0c|soa_serial]]: lines 100-109
- [[DNS from First Principles/Fragments/rgbdns-frag-3b7d91408492|is_authoritative]]: lines 110-112
- [[DNS from First Principles/Fragments/rgbdns-frag-636ad5773610|is_dnssec_signed]]: lines 113-119
- [[DNS from First Principles/Fragments/rgbdns-frag-8607097cd461|from_file]]: lines 120-132
- [[DNS from First Principles/Fragments/rgbdns-frag-c3fab1f9f5c9|parse]]: lines 133-135
- [[DNS from First Principles/Fragments/rgbdns-frag-a8c938b0a480|parse_with_serial]]: lines 136-151
- [[DNS from First Principles/Fragments/rgbdns-frag-2d6178010e02|validate_aliases]]: lines 152-199
- [[DNS from First Principles/Fragments/rgbdns-frag-230fef373ce0|add]]: lines 200-214
- [[DNS from First Principles/Fragments/rgbdns-frag-0495392f2c71|record_entries]]: lines 215-225
- [[DNS from First Principles/Fragments/rgbdns-frag-373e095ef9a2|location_entries]]: lines 226-230
- [[DNS from First Principles/Fragments/rgbdns-frag-8865b8fe1f9d|aname_entries]]: lines 231-233
- [[DNS from First Principles/Fragments/rgbdns-frag-ca3600304ff7|transfer_anames]]: lines 234-252
- [[DNS from First Principles/Fragments/rgbdns-frag-defe520b4c5c|aname]]: lines 253-255
- [[DNS from First Principles/Fragments/rgbdns-frag-1dc9d0701169|has_anames]]: lines 256-258
- [[DNS from First Principles/Fragments/rgbdns-frag-faeef6810168|has_aname_in_zone]]: lines 259-262
- [[DNS from First Principles/Fragments/rgbdns-frag-ddb2ef73889a|has_qualified_data_in_zone]]: lines 263-274
- [[DNS from First Principles/Fragments/rgbdns-frag-d16edb4202b0|authoritative_names]]: lines 275-277
- [[DNS from First Principles/Fragments/rgbdns-frag-cd7a8aca66f3|records_outside]]: lines 278-287
- [[DNS from First Principles/Fragments/rgbdns-frag-7f94160c210b|materialization_input]]: lines 288-306
- [[DNS from First Principles/Fragments/rgbdns-frag-053279bf2ac4|transfer]]: lines 307-330
- [[DNS from First Principles/Fragments/rgbdns-frag-890deacdeaf9|from_compiled_records]]: lines 331-369
- [[DNS from First Principles/Fragments/rgbdns-frag-32b94254ea9e|add_line]]: lines 370-450
- [[DNS from First Principles/Fragments/rgbdns-frag-d145f8bfd76e|lookup]]: lines 682-684
- [[DNS from First Principles/Fragments/rgbdns-frag-d1f5895a12fa|lookup_from]]: lines 685-687
- [[DNS from First Principles/Fragments/rgbdns-frag-58007b39d528|lookup_for]]: lines 688-768
- [[DNS from First Principles/Fragments/rgbdns-frag-a4e8a21459c7|soa]]: lines 793-797
- [[DNS from First Principles/Fragments/rgbdns-frag-354e5009ee9b|visible_records]]: lines 798-823
- [[DNS from First Principles/Fragments/rgbdns-frag-b66581e6e555|client_location]]: lines 824-841
- [[DNS from First Principles/Fragments/rgbdns-frag-c0bdf4828d47|name_exists]]: lines 842-853
- [[DNS from First Principles/Fragments/rgbdns-frag-155bc7530752|dnssec_signatures]]: lines 854-890
- [[DNS from First Principles/Fragments/rgbdns-frag-6e877f2c2a4e|dnssec_nsec_at]]: lines 891-908
- [[DNS from First Principles/Fragments/rgbdns-frag-11a440e2eae5|dnssec_nsec_zone]]: lines 909-944
- [[DNS from First Principles/Fragments/rgbdns-frag-73e9ad084753|rrsig_covered]]: lines 945-951
- [[DNS from First Principles/Fragments/rgbdns-frag-c9c29728a270|unix_now]]: lines 952-957
- [[DNS from First Principles/Fragments/rgbdns-frag-183dcd88cb73|Lookup]]: lines 958-967
- [[DNS from First Principles/Fragments/rgbdns-frag-0b5f9172900d|split_fields]]: lines 968-980
- [[DNS from First Principles/Fragments/rgbdns-frag-f66d8fab58dd|field]]: lines 981-985
- [[DNS from First Principles/Fragments/rgbdns-frag-3eaad9f2e40f|field_opt]]: lines 986-988
- [[DNS from First Principles/Fragments/rgbdns-frag-3c4c118d61de|number_or]]: lines 989-994
- [[DNS from First Principles/Fragments/rgbdns-frag-ca60714f58c2|expanded_target]]: lines 995-1002
- [[DNS from First Principles/Fragments/rgbdns-frag-1110882373cf|record_metadata]]: lines 1003-1020
- [[DNS from First Principles/Fragments/rgbdns-frag-1a679f85ce82|location_code]]: lines 1021-1027
- [[DNS from First Principles/Fragments/rgbdns-frag-062272b70988|unescape]]: lines 1028-1062
- [[DNS from First Principles/Fragments/rgbdns-frag-46263ecb4c90|tests]]: lines 1063-1065
- [[DNS from First Principles/Fragments/rgbdns-frag-4edddab661cf|common_markers]]: lines 1066-1080
- [[DNS from First Principles/Fragments/rgbdns-frag-807960c521fe|wildcard]]: lines 1081-1088
- [[DNS from First Principles/Fragments/rgbdns-frag-7b48fc998842|escaped_colon]]: lines 1089-1092
- [[DNS from First Principles/Fragments/rgbdns-frag-9c946d82538d|patched_srv_marker_and_glue]]: lines 1093-1112
- [[DNS from First Principles/Fragments/rgbdns-frag-7d9b00ff3ed7|original_ns_and_mx_field_positions_and_expansion]]: lines 1113-1145
- [[DNS from First Principles/Fragments/rgbdns-frag-3a4596180b49|explicit_soa_uses_ttl_field_nine]]: lines 1146-1163
- [[DNS from First Principles/Fragments/rgbdns-frag-477ba6f85fdb|text_escapes_use_one_to_three_octal_digits]]: lines 1164-1168
- [[DNS from First Principles/Fragments/rgbdns-frag-7e54e39e318f|tai64_activation_and_expiration_are_evaluated_at_lookup]]: lines 1169-1169
- [[DNS from First Principles/Fragments/rgbdns-frag-3c26d97aaee7|TAI_EPOCH]]: lines 1170-1198
- [[DNS from First Principles/Fragments/rgbdns-frag-79f59c5b2fab|longest_client_prefix_selects_location_records]]: lines 1199-1232
- [[DNS from First Principles/Fragments/rgbdns-frag-1649bad1a34b|acme_overlay_publishes_the_newer_of_source_and_stored_serials]]: lines 1233-1260
- [[DNS from First Principles/Fragments/rgbdns-frag-809b1f24c9a9|implicit_soa_uses_source_serial_and_generic_types_are_restricted]]: lines 1261-1273
- [[DNS from First Principles/Fragments/rgbdns-frag-8ffe7b4e1a09|empty_nonterminals_and_closest_encloser_block_higher_wildcards]]: lines 1274-1296
- [[DNS from First Principles/Fragments/rgbdns-frag-23d05c9e12ba|aname_coexists_with_apex_authority_but_not_address_or_cname_data]]: lines 1297-1324
- [[DNS from First Principles/Fragments/rgbdns-frag-e4475fcf308b|patched_ipv6_flat_format_has_unambiguous_ttl_and_reverse_trees]]: lines 1325-1348

## Full Source

```rust
use crate::{Error, Name, RData, Record, RecordType, Result};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    path::Path,
    str::FromStr,
};

#[derive(Clone, Debug, Default)]
pub struct Zone {
    records: BTreeMap<Name, Vec<Record>>,
    metadata: BTreeMap<Name, Vec<RecordMetadata>>,
    anames: BTreeMap<Name, Aname>,
    authoritative: BTreeSet<Name>,
    delegations: BTreeSet<Name>,
    locations: Vec<(Vec<u8>, [u8; 2])>,
    current_metadata: RecordMetadata,
    default_serial: u32,
    nodes: BTreeSet<Name>,
    unqualified_nodes: BTreeSet<Name>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Aname {
    pub target: Name,
    pub ttl: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct RecordMetadata {
    pub cutoff: u64,
    pub location: Option<[u8; 2]>,
}
pub(crate) type MaterializationInput = (Vec<Record>, Vec<(Name, Aname)>);
/// The later of two SOA serials in RFC 1982 serial-number order.
pub(crate) fn newer_serial(a: u32, b: u32) -> u32 {
    if b != a && b.wrapping_sub(a) < (1 << 31) {
        b
    } else {
        a
    }
}

impl Zone {
    /// Returns a validated snapshot with the supplied ACME TXT overlay and SOA
    /// serial overrides. The source zone is not modified.
    pub(crate) fn with_acme_overlay(
        &self,
        records: &[(Name, Vec<u8>, u32)],
        serials: &BTreeMap<Name, u32>,
    ) -> Result<Self> {
        let mut zone = self.clone();
        for (owner, value, ttl) in records {
            if zone.records.get(owner).is_some_and(|existing| {
                existing
                    .iter()
                    .any(|record| record.rr_type() != RecordType::Txt)
            }) || zone.anames.contains_key(owner)
            {
                return Err(Error::InvalidRecord(format!(
                    "ACME TXT at {owner} conflicts with alias data"
                )));
            }
            zone.current_metadata = RecordMetadata::default();
            zone.add(Record {
                name: owner.clone(),
                ttl: *ttl,
                data: RData::Txt(value.chunks(255).map(<[u8]>::to_vec).collect()),
            });
        }
        for (name, serial) in serials {
            let Some(zone_records) = zone.records.get_mut(name) else {
                return Err(Error::InvalidRecord(format!(
                    "ACME policy zone {name} is not authoritative"
                )));
            };
            let Some(soa) = zone_records
                .iter_mut()
                .find(|record| record.rr_type() == RecordType::Soa)
            else {
                return Err(Error::InvalidRecord(format!(
                    "ACME policy zone {name} has no SOA"
                )));
            };
            if let RData::Soa {
                serial: current, ..
            } = &mut soa.data
            {
                // The stored ACME serial only moves forward from updates; a newer
                // source serial (an operator's edit) must still publish, or
                // secondaries never see the change.
                *current = newer_serial(*current, *serial);
            }
        }
        zone.validate_aliases()?;
        Ok(zone)
    }

    pub(crate) fn soa_serial(&self, zone: &Name) -> Option<u32> {
        self.records.get(zone)?.iter().find_map(|record| {
            if let RData::Soa { serial, .. } = record.data {
                Some(serial)
            } else {
                None
            }
        })
    }

    pub(crate) fn is_authoritative(&self, zone: &Name) -> bool {
        self.authoritative.contains(zone)
    }
    pub(crate) fn is_dnssec_signed(&self, zone: &Name) -> bool {
        self.records.get(zone).is_some_and(|records| {
            [RecordType::Dnskey, RecordType::Nsec, RecordType::Rrsig]
                .iter()
                .all(|typ| records.iter().any(|record| record.rr_type() == *typ))
        })
    }
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if path.extension().is_some_and(|extension| extension == "cdb") {
            crate::cdb::load(path)
        } else {
            let serial = fs::metadata(path)?
                .modified()?
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .map_or(1, |duration| duration.as_secs() as u32)
                .max(1);
            Self::parse_with_serial(&fs::read_to_string(path)?, serial)
        }
    }
    pub fn parse(text: &str) -> Result<Self> {
        Self::parse_with_serial(text, 1)
    }
    fn parse_with_serial(text: &str, default_serial: u32) -> Result<Self> {
        let mut z = Self {
            default_serial,
            ..Self::default()
        };
        for (number, raw) in text.lines().enumerate() {
            let line = raw.trim_end_matches('\r');
            if line.is_empty() || line.starts_with('#') || line.starts_with('-') {
                continue;
            }
            z.add_line(line)
                .map_err(|e| Error::InvalidRecord(format!("line {}: {e}", number + 1)))?;
        }
        z.validate_aliases()?;
        Ok(z)
    }
    fn validate_aliases(&self) -> Result<()> {
        for (owner, records) in &self.records {
            let cnames = records
                .iter()
                .filter(|record| record.rr_type() == RecordType::Cname)
                .collect::<Vec<_>>();
            if cnames.is_empty() {
                continue;
            }
            if records
                .iter()
                .any(|record| record.rr_type() != RecordType::Cname)
                || cnames
                    .iter()
                    .skip(1)
                    .any(|record| record.data != cnames[0].data)
            {
                return Err(Error::InvalidRecord(format!(
                    "CNAME at {owner} conflicts with other data"
                )));
            }
        }
        for (owner, aname) in &self.anames {
            if owner.without_wildcard().is_some() {
                return Err(Error::InvalidRecord(format!(
                    "ANAME owner cannot be a wildcard: {owner}"
                )));
            }
            if owner == &aname.target {
                return Err(Error::InvalidRecord(format!(
                    "ANAME at {owner} points to itself"
                )));
            }
            if self.records.get(owner).is_some_and(|records| {
                records.iter().any(|record| {
                    matches!(
                        record.rr_type(),
                        RecordType::A | RecordType::Aaaa | RecordType::Cname
                    )
                })
            }) {
                return Err(Error::InvalidRecord(format!(
                    "ANAME at {owner} conflicts with A, AAAA, or CNAME data"
                )));
            }
        }
        Ok(())
    }
    fn add(&mut self, r: Record) {
        let mut node = Some(r.name.clone());
        while let Some(name) = node {
            self.nodes.insert(name.clone());
            if self.current_metadata.cutoff == 0 && self.current_metadata.location.is_none() {
                self.unqualified_nodes.insert(name.clone());
            }
            node = name.parent();
        }
        self.metadata
            .entry(r.name.clone())
            .or_default()
            .push(self.current_metadata);
        self.records.entry(r.name.clone()).or_default().push(r)
    }
    pub(crate) fn record_entries(&self) -> impl Iterator<Item = (&Record, RecordMetadata)> {
        self.records.iter().flat_map(|(owner, records)| {
            records.iter().zip(
                self.metadata
                    .get(owner)
                    .expect("record metadata invariant")
                    .iter()
                    .copied(),
            )
        })
    }
    pub(crate) fn location_entries(&self) -> impl Iterator<Item = (&[u8], [u8; 2])> {
        self.locations
            .iter()
            .map(|(prefix, location)| (prefix.as_slice(), *location))
    }
    pub(crate) fn aname_entries(&self) -> impl Iterator<Item = (&Name, &Aname)> {
        self.anames.iter()
    }
    pub(crate) fn transfer_anames(&self, name: &Name) -> Option<Vec<(Name, Aname)>> {
        if !self.authoritative.contains(name) {
            return None;
        }
        Some(
            self.anames
                .iter()
                .filter(|(owner, _)| {
                    owner.is_subdomain_of(name)
                        && !self.authoritative.iter().any(|child| {
                            child != name
                                && child.is_subdomain_of(name)
                                && owner.is_subdomain_of(child)
                        })
                })
                .map(|(owner, aname)| (owner.clone(), aname.clone()))
                .collect(),
        )
    }
    pub(crate) fn aname(&self, owner: &Name) -> Option<&Aname> {
        self.anames.get(owner)
    }
    pub(crate) fn has_anames(&self) -> bool {
        !self.anames.is_empty()
    }
    pub(crate) fn has_aname_in_zone(&self, name: &Name) -> bool {
        self.transfer_anames(name)
            .is_some_and(|anames| !anames.is_empty())
    }
    pub(crate) fn has_qualified_data_in_zone(&self, name: &Name) -> bool {
        self.records.iter().any(|(owner, records)| {
            owner.is_subdomain_of(name)
                && !self.authoritative.iter().any(|child| {
                    child != name && child.is_subdomain_of(name) && owner.is_subdomain_of(child)
                })
                && records.iter().enumerate().any(|(index, _)| {
                    let metadata = self.metadata[owner][index];
                    metadata.location.is_some() || metadata.cutoff != 0
                })
        })
    }
    pub(crate) fn authoritative_names(&self) -> BTreeSet<Name> {
        self.authoritative.clone()
    }
    pub(crate) fn records_outside(&self, zones: &BTreeSet<Name>) -> Vec<Record> {
        self.record_entries()
            .filter(|(record, metadata)| {
                metadata.location.is_none()
                    && metadata.cutoff == 0
                    && !zones.iter().any(|zone| record.name.is_subdomain_of(zone))
            })
            .map(|(record, _)| record.clone())
            .collect()
    }
    pub(crate) fn materialization_input(&self) -> Result<MaterializationInput> {
        if self
            .record_entries()
            .any(|(_, metadata)| metadata.location.is_some() || metadata.cutoff != 0)
        {
            return Err(Error::InvalidRecord(
                "ANAME materialization does not accept location-dependent or expiring data".into(),
            ));
        }
        Ok((
            self.record_entries()
                .map(|(record, _)| record.clone())
                .collect(),
            self.anames
                .iter()
                .map(|(owner, aname)| (owner.clone(), aname.clone()))
                .collect(),
        ))
    }
    pub fn transfer(&self, name: &Name) -> Option<Vec<Record>> {
        if !self.authoritative.contains(name) {
            return None;
        }
        let now = 4_611_686_018_427_387_914u64.saturating_add(unix_now());
        let soa = self.soa(name, [0, 0], now)?;
        let mut records = vec![soa.clone()];
        records.extend(
            self.records
                .iter()
                .filter(|(owner, _)| {
                    owner.is_subdomain_of(name)
                        && !self.authoritative.iter().any(|child| {
                            child != name
                                && child.is_subdomain_of(name)
                                && owner.is_subdomain_of(child)
                        })
                })
                .flat_map(|(owner, _)| self.visible_records(owner, [0, 0], now))
                .filter(|record| !(record.name == *name && record.rr_type() == RecordType::Soa)),
        );
        records.push(soa);
        Some(records)
    }
    pub(crate) fn from_compiled_records(
        records: Vec<(Record, RecordMetadata)>,
        locations: Vec<(Vec<u8>, [u8; 2])>,
        anames: Vec<(Name, Aname)>,
    ) -> Self {
        let mut zone = Self {
            locations,
            anames: anames.into_iter().collect(),
            ..Self::default()
        };
        for (record, metadata) in records {
            zone.current_metadata = metadata;
            if record.rr_type() == RecordType::Soa {
                zone.authoritative.insert(record.name.clone());
            }
            zone.add(record);
        }
        zone.current_metadata = RecordMetadata::default();
        let ns_owners = zone
            .records
            .iter()
            .filter(|(_, records)| records.iter().any(|r| r.rr_type() == RecordType::Ns))
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();
        for owner in ns_owners {
            if !zone.authoritative.contains(&owner) {
                zone.delegations.insert(owner);
            }
        }
        for owner in zone.anames.keys() {
            let mut node = Some(owner.clone());
            while let Some(name) = node {
                zone.nodes.insert(name.clone());
                zone.unqualified_nodes.insert(name.clone());
                node = name.parent();
            }
        }
        zone
    }
    fn add_line(&mut self, line: &str) -> Result<()> {
        let kind = line.as_bytes()[0];
        let f = split_fields(&line[1..]);
        if kind == b'%' {
            let location = location_code(field_opt(&f, 0).unwrap_or_default());
            let prefix = field_opt(&f, 1)
                .unwrap_or_default()
                .split('.')
                .filter(|field| !field.is_empty())
                .map(|field| {
                    field
                        .parse::<u8>()
                        .map_err(|_| Error::InvalidRecord("bad location IP prefix".into()))
                })
                .collect::<Result<Vec<_>>>()?;
            if prefix.len() > 4 {
                return Err(Error::InvalidRecord(
                    "location IP prefix has more than four octets".into(),
                ));
            }
            self.locations.push((prefix, location));
            return Ok(());
        }
        let name = field(&f, 0)?.parse::<Name>()?;
        self.current_metadata = match kind {
            b'Z' => record_metadata(&f, 9, 10),
            b'.' | b'&' => record_metadata(&f, 4, 5),
            b'+' | b'=' | b'C' | b'^' | b'\'' => record_metadata(&f, 3, 4),
            b'@' => record_metadata(&f, 5, 6),
            b'S' => record_metadata(&f, 7, 8),
            b':' => record_metadata(&f, 4, 5),
            _ => RecordMetadata::default(),
        };
        match kind {
            b'A' => {
                let target = field(&f, 1)?.parse::<Name>()?;
                let ttl = number_or(&f, 2, 300);
                if ttl == 0 {
                    return Err(Error::InvalidRecord("ANAME TTL must be positive".into()));
                }
                if let Some(existing) = self.anames.insert(name.clone(), Aname { target, ttl })
                    && self.anames[&name] != existing
                {
                    return Err(Error::InvalidRecord(format!(
                        "multiple ANAME targets at {name}"
                    )));
                }
                let mut node = Some(name);
                while let Some(value) = node {
                    self.nodes.insert(value.clone());
                    self.unqualified_nodes.insert(value.clone());
                    node = value.parent();
                }
            }
            b'=' | b'+' => {
                let ttl = number_or(&f, 2, 86400);
                let ip = field(&f, 1)?
                    .parse::<Ipv4Addr>()
                    .map_err(|_| Error::InvalidRecord("bad IPv4".into()))?;
                self.add(Record {
                    name: name.clone(),
                    ttl,
                    data: RData::A(ip),
                });
                if kind == b'=' {
                    let rev = Name::from_str(&format!(
                        "{}.{}.{}.{}.in-addr.arpa",
                        ip.octets()[3],
                        ip.octets()[2],
                        ip.octets()[1],
                        ip.octets()[0]
                    ))?;
                    self.add(Record {
                        name: rev,
                        ttl,
                        data: RData::Name(RecordType::Ptr, name),
                    })
                }
            }
            b'6' | b'3' => {
                // fefe's djbdns IPv6 patch deliberately uses a flat 32-digit
                // address so ':' remains an unambiguous field separator.
                let address = field(&f, 1)?;
                if address.len() != 32 || !address.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                    return Err(Error::InvalidRecord(
                        "IPv6 address must contain 32 flat hexadecimal digits".into(),
                    ));
                }
                let ip = Ipv6Addr::from(
                    u128::from_str_radix(address, 16)
                        .map_err(|_| Error::InvalidRecord("bad IPv6".into()))?,
                );
                let ttl = number_or(&f, 2, 86400);
                self.current_metadata = record_metadata(&f, 3, 4);
                self.add(Record {
                    name: name.clone(),
                    ttl,
                    data: RData::Aaaa(ip),
                });
                if kind == b'6' {
                    let hex = format!("{:032x}", u128::from(ip));
                    let nibbles = hex
                        .chars()
                        .rev()
                        .map(|character| character.to_string())
                        .collect::<Vec<_>>()
                        .join(".");
                    for suffix in ["ip6.arpa", "ip6.int"] {
                        self.add(Record {
                            name: Name::from_str(&format!("{nibbles}.{suffix}"))?,
                            ttl,
                            data: RData::Name(RecordType::Ptr, name.clone()),
                        });
                    }
                }
            }
            b'C' | b'^' => {
                let ttl = number_or(&f, 2, 86400);
                self.add(Record {
                    name,
                    ttl,
                    data: RData::Name(
                        if kind == b'C' {
                            RecordType::Cname
                        } else {
                            RecordType::Ptr
                        },
                        field(&f, 1)?.parse()?,
                    ),
                })
            }
            b'@' => {
                let ip = field_opt(&f, 1).filter(|value| !value.is_empty());
                let mx = expanded_target(field_opt(&f, 2).unwrap_or_default(), "mx", &name)?;
                let pref = number_or(&f, 3, 0);
                let ttl = number_or(&f, 4, 86400);
                self.add(Record {
                    name,
                    ttl,
                    data: RData::Mx(pref, mx.clone()),
                });
                if let Some(ip) = ip {
                    self.add(Record {
                        name: mx,
                        ttl,
                        data: RData::A(
                            ip.parse()
                                .map_err(|_| Error::InvalidRecord("bad MX glue".into()))?,
                        ),
                    });
                }
            }
            b'S' => {
                let ip = field_opt(&f, 1).filter(|x| !x.is_empty());
                let mut target = field(&f, 2)?.to_owned();
                if !target.contains('.') {
                    target = format!("{target}.srv.{name}");
                }
                let target: Name = target.parse()?;
                let number = |index: usize, default: u16| -> Result<u16> {
                    match field_opt(&f, index).filter(|x| !x.is_empty()) {
                        Some(value) => value
                            .parse()
                            .map_err(|_| Error::InvalidRecord(format!("bad SRV field {index}"))),
                        None => Ok(default),
                    }
                };
                let port = number(3, 0)?;
                let priority = number(4, 0)?;
                let weight = number(5, 0)?;
                let ttl = field_opt(&f, 6)
                    .and_then(|x| x.parse().ok())
                    .unwrap_or(86400);
                self.add(Record {
                    name,
                    ttl,
                    data: RData::Srv {
                        priority,
                        weight,
                        port,
                        target: target.clone(),
                    },
                });
                if let Some(ip) = ip {
                    self.add(Record {
                        name: target,
                        ttl,
                        data: RData::A(
                            ip.parse()
                                .map_err(|_| Error::InvalidRecord("bad SRV glue".into()))?,
                        ),
                    });
                }
            }
            b'\'' => {
                let ttl = number_or(&f, 2, 86400);
                let bytes = unescape(field(&f, 1)?)?;
                let chunks = bytes.chunks(127).map(<[u8]>::to_vec).collect();
                self.add(Record {
                    name,
                    ttl,
                    data: RData::Txt(chunks),
                })
            }
            b':' => {
                let typ = RecordType::from_code(
                    field(&f, 1)?
                        .parse()
                        .map_err(|_| Error::InvalidRecord("bad type".into()))?,
                );
                if matches!(
                    typ,
                    RecordType::Unknown(0)
                        | RecordType::Axfr
                        | RecordType::Soa
                        | RecordType::Ns
                        | RecordType::Cname
                        | RecordType::Ptr
                        | RecordType::Mx
                ) {
                    return Err(Error::InvalidRecord(
                        "record type is prohibited for the generic marker".into(),
                    ));
                }
                let data = RData::Opaque(typ, unescape(field(&f, 2)?)?);
                data.dnssec()?;
                self.add(Record {
                    name,
                    ttl: field_opt(&f, 3)
                        .and_then(|x| x.parse().ok())
                        .unwrap_or(86400),
                    data,
                })
            }
            b'&' | b'.' => {
                let ttl = number_or(&f, 3, 259200);
                let ip = field_opt(&f, 1).filter(|x| !x.is_empty());
                let host = expanded_target(field_opt(&f, 2).unwrap_or_default(), "ns", &name)?;
                self.add(Record {
                    name: name.clone(),
                    ttl,
                    data: RData::Name(RecordType::Ns, host.clone()),
                });
                if let Some(ip) = ip {
                    self.add(Record {
                        name: host.clone(),
                        ttl,
                        data: RData::A(
                            ip.parse()
                                .map_err(|_| Error::InvalidRecord("bad glue".into()))?,
                        ),
                    })
                }
                if kind == b'.' {
                    self.authoritative.insert(name.clone());
                    self.delegations.remove(&name);
                    let admin = format!("hostmaster.{name}").parse()?;
                    self.add(Record {
                        name,
                        ttl: if ttl == 0 { 0 } else { 2560 },
                        data: RData::Soa {
                            mname: host,
                            admin,
                            serial: self.default_serial,
                            refresh: 16384,
                            retry: 2048,
                            expire: 1048576,
                            minimum: 2560,
                        },
                    })
                } else {
                    if !self.authoritative.contains(&name) {
                        self.delegations.insert(name);
                    }
                }
            }
            b'Z' => {
                let ns = field(&f, 1)?.parse()?;
                let admin = field(&f, 2)?.parse()?;
                let nums = [
                    number_or(&f, 3, 1),
                    number_or(&f, 4, 16384),
                    number_or(&f, 5, 2048),
                    number_or(&f, 6, 1048576),
                    number_or(&f, 7, 2560),
                ];
                let ttl = number_or(&f, 8, 2560);
                self.authoritative.insert(name.clone());
                self.delegations.remove(&name);
                self.add(Record {
                    name,
                    ttl,
                    data: RData::Soa {
                        mname: ns,
                        admin,
                        serial: nums[0],
                        refresh: nums[1],
                        retry: nums[2],
                        expire: nums[3],
                        minimum: nums[4],
                    },
                })
            }
            _ => {
                return Err(Error::InvalidRecord(format!(
                    "unsupported marker {}",
                    kind as char
                )));
            }
        }
        Ok(())
    }
    pub fn lookup(&self, name: &Name, typ: RecordType) -> Lookup {
        self.lookup_for(name, typ, None, unix_now())
    }
    pub fn lookup_from(&self, name: &Name, typ: RecordType, client: IpAddr) -> Lookup {
        self.lookup_for(name, typ, Some(client), unix_now())
    }
    fn lookup_for(
        &self,
        name: &Name,
        typ: RecordType,
        client: Option<IpAddr>,
        unix_seconds: u64,
    ) -> Lookup {
        let location = self.client_location(client);
        let now = 4_611_686_018_427_387_914u64.saturating_add(unix_seconds);
        if let Some(delegation) = self
            .delegations
            .iter()
            .filter(|owner| name.is_subdomain_of(owner))
            .max_by_key(|owner| owner.labels().count())
        {
            if typ == RecordType::Ds && name == delegation {
                let parent = self
                    .authoritative
                    .iter()
                    .filter(|zone| delegation.is_subdomain_of(zone))
                    .max_by_key(|zone| zone.labels().count());
                if parent.is_some_and(|zone| self.is_dnssec_signed(zone)) {
                    let answer = self
                        .visible_records(delegation, location, now)
                        .into_iter()
                        .filter(|record| record.rr_type() == RecordType::Ds)
                        .collect::<Vec<_>>();
                    if !answer.is_empty() {
                        return Lookup::Answer(answer);
                    }
                    return Lookup::NoData(parent.and_then(|zone| self.soa(zone, location, now)));
                }
            }
            let authorities = self
                .visible_records(delegation, location, now)
                .into_iter()
                .filter(|record| record.rr_type() == RecordType::Ns)
                .collect::<Vec<_>>();
            let mut additionals = Vec::new();
            for authority in &authorities {
                let RData::Name(RecordType::Ns, target) = &authority.data else {
                    continue;
                };
                if !target.is_subdomain_of(delegation) {
                    continue;
                }
                additionals.extend(
                    self.visible_records(target, location, now)
                        .into_iter()
                        .filter(|record| {
                            matches!(record.rr_type(), RecordType::A | RecordType::Aaaa)
                        }),
                );
            }
            return Lookup::Referral {
                authorities,
                additionals,
            };
        }
        let mut rows = self.visible_records(name, location, now);
        if rows.is_empty() {
            if self.name_exists(name, location, now) {
                let zone = self
                    .authoritative
                    .iter()
                    .filter(|z| name.is_subdomain_of(z))
                    .max_by_key(|z| z.labels().count());
                return Lookup::NoData(zone.and_then(|z| self.soa(z, location, now)));
            }
            let mut p = name.parent();
            while let Some(n) = p {
                if self.name_exists(&n, location, now) {
                    rows = self.visible_records(&n.wildcard(), location, now);
                    break;
                }
                p = n.parent()
            }
        }
        let zone = self
            .authoritative
            .iter()
            .filter(|z| name.is_subdomain_of(z))
            .max_by_key(|z| z.labels().count());
        if rows.is_empty() {
            return if let Some(zone) = zone {
                Lookup::NxDomain(self.soa(zone, location, now))
            } else {
                Lookup::Refused
            };
        }
        let mut answer: Vec<Record> = rows
            .into_iter()
            .filter(|r| {
                typ == RecordType::Any || r.rr_type() == typ || r.rr_type() == RecordType::Cname
            })
            .collect();
        for r in &mut answer {
            r.name = name.clone()
        }
        if answer.is_empty() {
            Lookup::NoData(zone.and_then(|z| self.soa(z, location, now)))
        } else {
            Lookup::Answer(answer)
        }
    }
    fn soa(&self, z: &Name, location: [u8; 2], now: u64) -> Option<Record> {
        self.visible_records(z, location, now)
            .into_iter()
            .find(|r| r.rr_type() == RecordType::Soa)
    }
    fn visible_records(&self, owner: &Name, location: [u8; 2], now: u64) -> Vec<Record> {
        let Some(records) = self.records.get(owner) else {
            return Vec::new();
        };
        records
            .iter()
            .zip(self.metadata.get(owner).expect("record metadata invariant"))
            .filter_map(|(record, metadata)| {
                if metadata.location.is_some_and(|value| value != location) {
                    return None;
                }
                let mut record = record.clone();
                if metadata.cutoff != 0 {
                    if record.ttl == 0 {
                        if metadata.cutoff < now {
                            return None;
                        }
                        record.ttl = metadata.cutoff.saturating_sub(now).clamp(2, 3600) as u32;
                    } else if metadata.cutoff >= now {
                        return None;
                    }
                }
                Some(record)
            })
            .collect()
    }
    fn client_location(&self, client: Option<IpAddr>) -> [u8; 2] {
        let Some(IpAddr::V4(address)) = client else {
            return [0, 0];
        };
        let octets = address.octets();
        let mut selected = [0, 0];
        let mut selected_length = None;
        for (prefix, location) in &self.locations {
            if prefix.len() <= octets.len()
                && octets[..prefix.len()] == prefix[..]
                && selected_length.is_none_or(|length| prefix.len() > length)
            {
                selected = *location;
                selected_length = Some(prefix.len());
            }
        }
        selected
    }
    fn name_exists(&self, name: &Name, location: [u8; 2], now: u64) -> bool {
        if self.unqualified_nodes.contains(name) {
            return true;
        }
        if !self.nodes.contains(name) {
            return false;
        }
        self.records.keys().any(|owner| {
            owner.is_subdomain_of(name) && !self.visible_records(owner, location, now).is_empty()
        })
    }

    pub(crate) fn dnssec_signatures(
        &self,
        name: &Name,
        covered: &BTreeSet<RecordType>,
        client: Option<IpAddr>,
    ) -> Vec<Record> {
        let location = self.client_location(client);
        let now = 4_611_686_018_427_387_914u64.saturating_add(unix_now());
        let mut owner = name.clone();
        let mut signatures = self.visible_records(&owner, location, now);
        if !signatures
            .iter()
            .any(|record| record.rr_type() == RecordType::Rrsig)
        {
            let mut parent = name.parent();
            while let Some(candidate) = parent {
                if self.name_exists(&candidate, location, now) {
                    owner = candidate.wildcard();
                    signatures = self.visible_records(&owner, location, now);
                    break;
                }
                parent = candidate.parent();
            }
        }
        signatures
            .into_iter()
            .filter(|record| {
                record.rr_type() == RecordType::Rrsig
                    && rrsig_covered(&record.data).is_some_and(|typ| covered.contains(&typ))
            })
            .map(|mut record| {
                record.name = name.clone();
                record
            })
            .collect()
    }

    pub(crate) fn dnssec_nsec_at(&self, name: &Name, client: Option<IpAddr>) -> Vec<Record> {
        let location = self.client_location(client);
        let now = 4_611_686_018_427_387_914u64.saturating_add(unix_now());
        let mut records = self
            .visible_records(name, location, now)
            .into_iter()
            .filter(|record| record.rr_type() == RecordType::Nsec)
            .collect::<Vec<_>>();
        if !records.is_empty() {
            records.extend(self.dnssec_signatures(
                name,
                &BTreeSet::from([RecordType::Nsec]),
                client,
            ));
        }
        records
    }

    pub(crate) fn dnssec_nsec_zone(&self, name: &Name, client: Option<IpAddr>) -> Vec<Record> {
        let location = self.client_location(client);
        let now = 4_611_686_018_427_387_914u64.saturating_add(unix_now());
        let Some(zone) = self
            .authoritative
            .iter()
            .filter(|zone| name.is_subdomain_of(zone))
            .max_by_key(|zone| zone.labels().count())
        else {
            return Vec::new();
        };
        let mut output = Vec::new();
        for owner in self.records.keys().filter(|owner| {
            owner.is_subdomain_of(zone)
                && !self.authoritative.iter().any(|child| {
                    child != zone && child.is_subdomain_of(zone) && owner.is_subdomain_of(child)
                })
        }) {
            let nsec = self
                .visible_records(owner, location, now)
                .into_iter()
                .filter(|record| record.rr_type() == RecordType::Nsec)
                .collect::<Vec<_>>();
            if !nsec.is_empty() {
                output.extend(nsec);
                output.extend(self.dnssec_signatures(
                    owner,
                    &BTreeSet::from([RecordType::Nsec]),
                    client,
                ));
            }
        }
        output
    }
}

fn rrsig_covered(data: &RData) -> Option<RecordType> {
    match data.dnssec().ok().flatten()? {
        crate::DnssecRData::Rrsig { type_covered, .. } => Some(type_covered),
        _ => None,
    }
}

fn unix_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_secs())
}
#[derive(Clone, Debug)]
pub enum Lookup {
    Answer(Vec<Record>),
    Referral {
        authorities: Vec<Record>,
        additionals: Vec<Record>,
    },
    NoData(Option<Record>),
    NxDomain(Option<Record>),
    Refused,
}
fn split_fields(s: &str) -> Vec<String> {
    let mut out = vec![String::new()];
    let mut esc = false;
    for c in s.chars() {
        if c == ':' && !esc {
            out.push(String::new())
        } else {
            out.last_mut().unwrap().push(c)
        }
        esc = c == '\\' && !esc;
    }
    out
}
fn field(f: &[String], i: usize) -> Result<&str> {
    f.get(i)
        .map(String::as_str)
        .ok_or_else(|| Error::InvalidRecord(format!("missing field {}", i + 1)))
}
fn field_opt(f: &[String], i: usize) -> Option<&str> {
    f.get(i).map(String::as_str)
}
fn number_or<T: FromStr + Copy>(fields: &[String], index: usize, default: T) -> T {
    field_opt(fields, index)
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}
fn expanded_target(value: &str, role: &str, owner: &Name) -> Result<Name> {
    let target = if value.contains('.') {
        value.to_owned()
    } else {
        format!("{value}.{role}.{owner}")
    };
    target.parse()
}
fn record_metadata(fields: &[String], timestamp: usize, location: usize) -> RecordMetadata {
    let text = field_opt(fields, timestamp).unwrap_or_default().as_bytes();
    let mut bytes = [0; 8];
    for (index, byte) in text.iter().take(16).enumerate() {
        let nibble = match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            _ => 0,
        };
        bytes[index / 2] |= if index % 2 == 0 { nibble << 4 } else { nibble };
    }
    let location = location_code(field_opt(fields, location).unwrap_or_default());
    RecordMetadata {
        cutoff: u64::from_be_bytes(bytes),
        location: (location != [0, 0]).then_some(location),
    }
}

fn location_code(value: &str) -> [u8; 2] {
    let bytes = value.as_bytes();
    [
        bytes.first().copied().unwrap_or(0),
        bytes.get(1).copied().unwrap_or(0),
    ]
}
fn unescape(s: &str) -> Result<Vec<u8>> {
    let mut o = Vec::new();
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\\' {
            i += 1;
            if i >= b.len() {
                return Err(Error::InvalidRecord("trailing escape".into()));
            }
            if (b'0'..=b'7').contains(&b[i]) {
                let mut n = b[i] - b'0';
                i += 1;
                for _ in 0..2 {
                    if i < b.len() && (b'0'..=b'7').contains(&b[i]) {
                        n = n.wrapping_mul(8).wrapping_add(b[i] - b'0');
                        i += 1;
                    } else {
                        break;
                    }
                }
                o.push(n)
            } else {
                o.push(b[i]);
                i += 1
            }
        } else {
            o.push(b[i]);
            i += 1
        }
    }
    Ok(o)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn common_markers() {
        let z=Zone::parse(".example:192.0.2.53:ns.example\n=www.example:192.0.2.1:60\n'example:hello\\072world\n6v6.example:20010db8000000000000000000000001\n").unwrap();
        assert!(
            matches!(z.lookup(&"www.example".parse().unwrap(),RecordType::A),Lookup::Answer(x) if x[0].ttl==60)
        );
        assert!(matches!(
            z.lookup(&"missing.example".parse().unwrap(), RecordType::A),
            Lookup::NxDomain(Some(_))
        ));
        assert!(matches!(
            z.lookup(&"v6.example".parse().unwrap(), RecordType::Aaaa),
            Lookup::Answer(_)
        ))
    }
    #[test]
    fn wildcard() {
        let z = Zone::parse(".example::ns.example\n+*.example:192.0.2.4\n").unwrap();
        assert!(matches!(
            z.lookup(&"x.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(_)
        ))
    }
    #[test]
    fn escaped_colon() {
        assert_eq!(unescape(r"a\072b").unwrap(), b"a:b")
    }
    #[test]
    fn patched_srv_marker_and_glue() {
        let z = Zone::parse("S_sip._tcp.example:192.0.2.7:sip:5060:10:20:300\n").unwrap();
        assert!(matches!(
            z.lookup(
                &"_sip._tcp.example".parse().unwrap(),
                RecordType::Srv
            ),
            Lookup::Answer(records)
                if matches!(
                    &records[0].data,
                    RData::Srv { priority: 10, weight: 20, port: 5060, target }
                        if target.to_string() == "sip.srv._sip._tcp.example."
                )
        ));
        assert!(matches!(
            z.lookup(&"sip.srv._sip._tcp.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(_)
        ));
    }
    #[test]
    fn original_ns_and_mx_field_positions_and_expansion() {
        let z = Zone::parse(
            ".example:192.0.2.53:a:300\n\
             @example:192.0.2.25:mail:20:400\n",
        )
        .unwrap();
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Ns),
            Lookup::Answer(records)
                if records[0].ttl == 300
                    && matches!(&records[0].data, RData::Name(_, target)
                        if target.to_string() == "a.ns.example.")
        ));
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Mx),
            Lookup::Answer(records)
                if records[0].ttl == 400
                    && matches!(&records[0].data, RData::Mx(20, target)
                        if target.to_string() == "mail.mx.example.")
        ));
        assert!(matches!(
            z.lookup(&"mail.mx.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(records)
                if records[0].ttl == 400
                    && records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 25))
        ));
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Soa),
            Lookup::Answer(records) if records[0].ttl == 2560
        ));
    }

    #[test]
    fn explicit_soa_uses_ttl_field_nine() {
        let z = Zone::parse("Zexample:ns.example:hostmaster.example:7:8:9:10:11:12\n").unwrap();
        assert!(matches!(
            z.lookup(&"example".parse().unwrap(), RecordType::Soa),
            Lookup::Answer(records)
                if records[0].ttl == 12
                    && matches!(&records[0].data, RData::Soa {
                        serial: 7,
                        refresh: 8,
                        retry: 9,
                        expire: 10,
                        minimum: 11,
                        ..
                    })
        ));
    }

    #[test]
    fn text_escapes_use_one_to_three_octal_digits() {
        assert_eq!(unescape(r"\1\12\123\8").unwrap(), [1, 10, 83, b'8']);
    }

    #[test]
    fn tai64_activation_and_expiration_are_evaluated_at_lookup() {
        const TAI_EPOCH: u64 = 4_611_686_018_427_387_914;
        let cutoff = format!("{:016x}", TAI_EPOCH + 200);
        let z = Zone::parse(&format!(
            ".example::ns.example\n\
             +expires.example:192.0.2.1:0:{cutoff}\n\
             +activates.example:192.0.2.2:60:{cutoff}\n"
        ))
        .unwrap();
        let expires = "expires.example".parse().unwrap();
        assert!(matches!(
            z.lookup_for(&expires, RecordType::A, None, 100),
            Lookup::Answer(records) if records[0].ttl == 100
        ));
        assert!(matches!(
            z.lookup_for(&expires, RecordType::A, None, 201),
            Lookup::NxDomain(_)
        ));
        let activates = "activates.example".parse().unwrap();
        assert!(matches!(
            z.lookup_for(&activates, RecordType::A, None, 200),
            Lookup::NxDomain(_)
        ));
        assert!(matches!(
            z.lookup_for(&activates, RecordType::A, None, 201),
            Lookup::Answer(records) if records[0].ttl == 60
        ));
    }

    #[test]
    fn longest_client_prefix_selects_location_records() {
        let z = Zone::parse(
            ".example::ns.example\n\
             %aa:192\n\
             %bb:192.0.2\n\
             +located.example:192.0.2.1:60::aa\n\
             +located.example:192.0.2.2:60::bb\n",
        )
        .unwrap();
        let name = "located.example".parse().unwrap();
        assert!(matches!(
            z.lookup_for(
                &name,
                RecordType::A,
                Some(IpAddr::V4(Ipv4Addr::new(192, 0, 2, 55))),
                0,
            ),
            Lookup::Answer(records)
                if records.len() == 1
                    && records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 2))
        ));
        assert!(matches!(
            z.lookup_for(
                &name,
                RecordType::A,
                Some(IpAddr::V4(Ipv4Addr::new(192, 9, 9, 9))),
                0,
            ),
            Lookup::Answer(records)
                if records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 1))
        ));
    }

    #[test]
    fn acme_overlay_publishes_the_newer_of_source_and_stored_serials() {
        let zone: Name = "example".parse().unwrap();
        let soa = |base: u32| {
            Zone::parse(&format!(
                "Zexample:ns.example:hostmaster.example:{base}:16384:2048:1048576:2560:3600\n"
            ))
            .unwrap()
        };
        let published = |base: u32, stored: u32| {
            let serials = BTreeMap::from([(zone.clone(), stored)]);
            soa(base)
                .with_acme_overlay(&[], &serials)
                .unwrap()
                .soa_serial(&zone)
                .unwrap()
        };
        // an operator's edit after the last ACME update must still publish
        assert_eq!(published(2_026_091_201, 2_026_081_503), 2_026_091_201);
        // ACME updates since the last edit keep their higher serial
        assert_eq!(published(2_026_081_501, 2_026_081_503), 2_026_081_503);
        assert_eq!(published(7, 7), 7);
        // RFC 1982 order across the wrap
        assert_eq!(published(2, u32::MAX), 2);
        assert_eq!(newer_serial(u32::MAX, 2), 2);
        assert_eq!(newer_serial(2, u32::MAX), 2);
    }

    #[test]
    fn implicit_soa_uses_source_serial_and_generic_types_are_restricted() {
        let zone = Zone::parse_with_serial(".example::ns.example\n", 1_234_567).unwrap();
        assert!(matches!(
            zone.lookup(&"example".parse().unwrap(), RecordType::Soa),
            Lookup::Answer(records)
                if matches!(records[0].data, RData::Soa { serial: 1_234_567, .. })
        ));
        for record_type in [0, 2, 5, 6, 12, 15, 252] {
            assert!(Zone::parse(&format!(":example:{record_type}:x\n")).is_err());
        }
    }

    #[test]
    fn empty_nonterminals_and_closest_encloser_block_higher_wildcards() {
        let zone = Zone::parse(
            ".example::ns.example\n\
             +*.example:192.0.2.1\n\
             +leaf.branch.example:192.0.2.2\n",
        )
        .unwrap();
        assert!(matches!(
            zone.lookup(&"branch.example".parse().unwrap(), RecordType::A),
            Lookup::NoData(Some(_))
        ));
        assert!(matches!(
            zone.lookup(&"missing.branch.example".parse().unwrap(), RecordType::A),
            Lookup::NxDomain(Some(_))
        ));
        assert!(matches!(
            zone.lookup(&"other.example".parse().unwrap(), RecordType::A),
            Lookup::Answer(records)
                if records[0].data == RData::A(Ipv4Addr::new(192, 0, 2, 1))
        ));
    }

    #[test]
    fn aname_coexists_with_apex_authority_but_not_address_or_cname_data() {
        let zone = Zone::parse(
            ".example:192.0.2.53:ns.example\n\
             Aexample:hosting.example.net:120\n",
        )
        .unwrap();
        let aname = zone.aname(&"example".parse().unwrap()).unwrap();
        assert_eq!(aname.target, "hosting.example.net".parse().unwrap());
        assert_eq!(aname.ttl, 120);
        assert!(
            Zone::parse(
                ".example:192.0.2.53:ns.example\n\
                 Aexample:hosting.example.net\n\
                 +example:192.0.2.1\n",
            )
            .is_err()
        );
        assert!(
            Zone::parse(
                ".example:192.0.2.53:ns.example\n\
                 Aexample:hosting.example.net\n\
                 Cexample:other.example.net\n",
            )
            .is_err()
        );
    }

    #[test]
    fn patched_ipv6_flat_format_has_unambiguous_ttl_and_reverse_trees() {
        let zone = Zone::parse(
            ".example::ns.example\n\
             6v6.example:20010db8000000000000000000000001:123\n",
        )
        .unwrap();
        assert!(matches!(
            zone.lookup(&"v6.example".parse().unwrap(), RecordType::Aaaa),
            Lookup::Answer(records)
                if records[0].ttl == 123
                    && records[0].data == RData::Aaaa("2001:db8::1".parse().unwrap())
        ));
        let nibbles = "1.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.8.b.d.0.1.0.0.2";
        for suffix in ["ip6.arpa", "ip6.int"] {
            assert!(matches!(
                zone.lookup(
                    &format!("{nibbles}.{suffix}").parse().unwrap(),
                    RecordType::Ptr,
                ),
                Lookup::Answer(_)
            ));
        }
    }
}
```
