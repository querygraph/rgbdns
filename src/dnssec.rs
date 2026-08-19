//! Offline DNSSEC transforms used by the small `dnssec-*` utilities.

use crate::{Error, Message, Name, RData, Record, RecordType, Result, axfr, zone::Zone};
use hickory_server::{
    dnssec::NxProofKind,
    net::runtime::TokioRuntimeProvider,
    proto::{
        dnssec::{
            Algorithm, DigestType, DnssecSigner, SigningKey, crypto::EcdsaSigningKey, rdata::DNSKEY,
        },
        op::{Message as HickoryMessage, MessageType, OpCode},
        rr::Name as HickoryName,
    },
    store::in_memory::InMemoryZoneHandler,
    zone_handler::{AxfrPolicy, ZoneType},
};
use rustls_pki_types::PrivatePkcs8KeyDer;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

pub const ALGORITHM: u8 = 13;
pub const DEFAULT_VALIDITY: u64 = 14 * 24 * 60 * 60;
pub const DEFAULT_REFRESH: u64 = 24 * 60 * 60;
pub const DEFAULT_INCEPTION_SKEW: u64 = 60 * 60;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Policy {
    pub zone: Name,
    pub key_file: PathBuf,
    pub validity: u64,
    pub refresh: u64,
    pub inception_skew: u64,
}

impl Policy {
    pub fn parse(line: &str) -> Result<Self> {
        let fields = line
            .strip_prefix('K')
            .ok_or_else(|| Error::InvalidRecord("DNSSEC policy must start with K".into()))?
            .split(':')
            .collect::<Vec<_>>();
        if fields.len() != 6 || fields.iter().any(|field| field.is_empty()) {
            return Err(Error::InvalidRecord(
                "DNSSEC policy is Kzone:keyfile:algorithm:validity:refresh:inception-skew".into(),
            ));
        }
        if fields[2] != ALGORITHM.to_string() {
            return Err(Error::InvalidRecord(
                "only DNSSEC algorithm 13 is supported".into(),
            ));
        }
        let duration = |field: &str, label: &str| -> Result<u64> {
            let value = field
                .parse::<u64>()
                .map_err(|_| Error::InvalidRecord(format!("invalid DNSSEC {label}")))?;
            if !(300..=31_536_000).contains(&value) {
                return Err(Error::InvalidRecord(format!(
                    "DNSSEC {label} must be between 300 and 31536000 seconds"
                )));
            }
            Ok(value)
        };
        let validity = duration(fields[3], "validity")?;
        let refresh = duration(fields[4], "refresh")?;
        let inception_skew = duration(fields[5], "inception skew")?;
        if refresh >= validity || inception_skew >= validity {
            return Err(Error::InvalidRecord(
                "DNSSEC refresh and inception skew must be shorter than validity".into(),
            ));
        }
        let key_file = PathBuf::from(fields[1]);
        if !key_file.is_absolute() {
            return Err(Error::InvalidRecord(
                "DNSSEC key path must be absolute".into(),
            ));
        }
        Ok(Self {
            zone: fields[0].parse()?,
            key_file,
            validity,
            refresh,
            inception_skew,
        })
    }

    pub fn line(&self) -> String {
        format!(
            "K{}:{}:{}:{}:{}:{}",
            self.zone,
            self.key_file.display(),
            ALGORITHM,
            self.validity,
            self.refresh,
            self.inception_skew
        )
    }
}

pub fn read_policies(path: impl AsRef<Path>) -> Result<Vec<Policy>> {
    let text = fs::read_to_string(path)?;
    let mut policies = Vec::new();
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let policy = Policy::parse(line).map_err(|error| {
            Error::InvalidRecord(format!("DNSSEC policy line {}: {error}", index + 1))
        })?;
        if policies
            .iter()
            .any(|existing: &Policy| existing.zone == policy.zone)
        {
            return Err(Error::InvalidRecord(format!(
                "multiple active DNSSEC keys for {} are not supported",
                policy.zone
            )));
        }
        policies.push(policy);
    }
    if policies.is_empty() {
        return Err(Error::InvalidRecord("DNSSEC policy is empty".into()));
    }
    Ok(policies)
}

#[cfg(unix)]
pub fn generate_key(zone: &Name, path: &Path) -> Result<Policy> {
    use std::os::unix::fs::OpenOptionsExt;

    let key = EcdsaSigningKey::generate_pkcs8(Algorithm::ECDSAP256SHA256)
        .map_err(|_| Error::Format("DNSSEC key generation failed"))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    if let Err(error) = file
        .write_all(key.secret_pkcs8_der())
        .and_then(|()| file.sync_all())
    {
        drop(file);
        let _ = fs::remove_file(path);
        return Err(error.into());
    }
    Ok(Policy {
        zone: zone.clone(),
        key_file: path.to_path_buf(),
        validity: DEFAULT_VALIDITY,
        refresh: DEFAULT_REFRESH,
        inception_skew: DEFAULT_INCEPTION_SKEW,
    })
}

#[cfg(not(unix))]
pub fn generate_key(_zone: &Name, _path: &Path) -> Result<Policy> {
    Err(Error::Format(
        "DNSSEC key generation requires Unix file modes",
    ))
}

pub fn sign_file(input: &Path, policy_file: &Path, output: &Path) -> Result<()> {
    let policies = read_policies(policy_file)?;
    if policies.len() != 1 {
        return Err(Error::InvalidRecord(
            "this DNSSEC signer release accepts exactly one zone per snapshot".into(),
        ));
    }
    let zone = Zone::from_file(input)?;
    let records = sign_zone(&zone, &policies[0])?;
    let temporary = sibling_temporary(output);
    axfr::write_tinydns(&records, output, &temporary)?;
    sync_parent(output)
}

pub fn compile_file(input: &Path, policy_file: &Path, output: &Path) -> Result<()> {
    let policies = read_policies(policy_file)?;
    if policies.len() != 1 {
        return Err(Error::InvalidRecord(
            "this DNSSEC signer release accepts exactly one zone per snapshot".into(),
        ));
    }
    let source = Zone::from_file(input)?;
    let records = sign_zone(&source, &policies[0])?;
    let signed = Zone::from_compiled_records(
        records
            .into_iter()
            .map(|record| (record, crate::zone::RecordMetadata::default()))
            .collect(),
        Vec::new(),
        Vec::new(),
    );
    let temporary = sibling_temporary(output);
    if let Err(error) = crate::cdb::compile(&signed, &temporary) {
        let _ = fs::remove_file(&temporary);
        return Err(error);
    }
    File::open(&temporary)?.sync_all()?;
    crate::cdb::load(&temporary)?;
    fs::rename(&temporary, output)?;
    sync_parent(output)
}

pub fn ds_line(policy: &Policy) -> Result<String> {
    let key = load_key(&policy.key_file)?;
    let public = key
        .to_public_key()
        .map_err(|_| Error::Format("cannot derive DNSSEC public key"))?;
    let dnskey = DNSKEY::from_key(&public);
    let origin = HickoryName::from_str(&policy.zone.to_string())
        .map_err(|_| Error::Format("DNSSEC origin conversion failed"))?;
    let key_tag = dnskey
        .calculate_key_tag()
        .map_err(|_| Error::Format("cannot calculate DNSSEC key tag"))?;
    let digest = dnskey
        .to_digest(&origin, DigestType::SHA256)
        .map_err(|_| Error::Format("cannot calculate DNSSEC DS digest"))?;
    let hexadecimal = digest
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<String>();
    Ok(format!(
        "{} IN DS {key_tag} {} 2 {hexadecimal}",
        policy.zone, ALGORITHM
    ))
}

pub fn sign_zone(zone: &Zone, policy: &Policy) -> Result<Vec<Record>> {
    if zone.has_aname_below(&policy.zone) {
        return Err(Error::InvalidRecord(format!(
            "signed zone {} contains an unmaterialized ANAME",
            policy.zone
        )));
    }
    if zone.has_qualified_data_below(&policy.zone) {
        return Err(Error::InvalidRecord(format!(
            "signed zone {} contains location-dependent or expiring data",
            policy.zone
        )));
    }
    let mut source = zone
        .transfer(&policy.zone)
        .ok_or_else(|| Error::InvalidRecord(format!("{} is not authoritative", policy.zone)))?;
    source.pop();
    if source.iter().any(|record| {
        matches!(
            record.rr_type(),
            RecordType::Dnskey | RecordType::Rrsig | RecordType::Nsec | RecordType::Unknown(50)
        )
    }) {
        return Err(Error::InvalidRecord(format!(
            "signed zone {} already contains DNSSEC records",
            policy.zone
        )));
    }

    // Hickory's signer increments the SOA before signing. Feed it the prior
    // serial so that the published serial remains owned by the input producer.
    if let Some(Record {
        data: RData::Soa { serial, .. },
        ..
    }) = source
        .iter_mut()
        .find(|record| record.name == policy.zone && record.rr_type() == RecordType::Soa)
    {
        *serial = serial.wrapping_sub(1);
    }

    let origin = HickoryName::from_str(&policy.zone.to_string())
        .map_err(|_| Error::Format("DNSSEC origin conversion failed"))?;
    let mut signed: InMemoryZoneHandler<TokioRuntimeProvider> = InMemoryZoneHandler::empty(
        origin.clone(),
        ZoneType::Primary,
        AxfrPolicy::Deny,
        Some(NxProofKind::Nsec),
    );
    for record in source {
        let record = to_hickory(&record)?;
        if !signed.upsert_mut(record, 0) {
            return Err(Error::Format("DNSSEC signer rejected a zone record"));
        }
    }

    let key = load_key(&policy.key_file)?;
    let dnskey = DNSKEY::from_key(
        &key.to_public_key()
            .map_err(|_| Error::Format("cannot derive DNSSEC public key"))?,
    );
    let signer = DnssecSigner::new(
        dnskey,
        Box::new(key),
        origin,
        Duration::from_secs(policy.validity),
    );
    signed
        .add_zone_signing_key_mut(signer)
        .map_err(|_| Error::Format("cannot install DNSSEC signing key"))?;
    signed
        .secure_zone_mut()
        .map_err(|_| Error::Format("DNSSEC signing failed"))?;

    let mut output = Vec::new();
    for set in signed.records_get_mut().values() {
        for record in set.records(true) {
            output.push(from_hickory(record)?);
        }
    }
    Ok(output)
}

fn load_key(path: &Path) -> Result<EcdsaSigningKey> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if fs::metadata(path)?.permissions().mode() & 0o077 != 0 {
            return Err(Error::InvalidRecord(format!(
                "DNSSEC key {} must not be accessible by group or other",
                path.display()
            )));
        }
    }
    let key_der = PrivatePkcs8KeyDer::from(fs::read(path)?);
    EcdsaSigningKey::from_pkcs8(&key_der, Algorithm::ECDSAP256SHA256)
        .map_err(|_| Error::Format("invalid DNSSEC PKCS#8 key"))
}

fn to_hickory(record: &Record) -> Result<hickory_server::proto::rr::Record> {
    let bytes = Message {
        flags: 0x8400,
        answers: vec![record.clone()],
        ..Message::default()
    }
    .encode()?;
    HickoryMessage::from_vec(&bytes)
        .map_err(|_| Error::Format("DNSSEC record conversion failed"))?
        .answers
        .first()
        .cloned()
        .ok_or(Error::Format("DNSSEC record conversion produced no record"))
}

fn from_hickory(record: &hickory_server::proto::rr::Record) -> Result<Record> {
    let mut message = HickoryMessage::new(0, MessageType::Response, OpCode::Query);
    message.add_answer(record.clone());
    let bytes = message
        .to_vec()
        .map_err(|_| Error::Format("signed record encoding failed"))?;
    Message::decode(&bytes)?
        .answers
        .into_iter()
        .next()
        .ok_or(Error::Format("signed record conversion produced no record"))
}

fn sibling_temporary(output: &Path) -> PathBuf {
    let mut name = output
        .file_name()
        .map_or_else(|| "data.signed".into(), |name| name.to_os_string());
    name.push(".tmp");
    output.with_file_name(name)
}

pub fn sync_parent(path: &Path) -> Result<()> {
    File::open(path.parent().unwrap_or_else(|| Path::new(".")))?.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_is_one_line_and_bounded() {
        let policy = Policy::parse("Kexample:/tmp/example.pk8:13:1209600:86400:3600").unwrap();
        assert_eq!(policy.zone, "example".parse().unwrap());
        assert_eq!(
            policy.line(),
            "Kexample.:/tmp/example.pk8:13:1209600:86400:3600"
        );
        assert!(Policy::parse("Kexample:key:13:1209600:86400:3600").is_err());
        assert!(Policy::parse("Kexample:/key:15:1209600:86400:3600").is_err());
        assert!(Policy::parse("Kexample:/key:13:86400:86400:3600").is_err());
    }

    #[test]
    fn signs_a_zone_with_dnskey_nsec_and_rrsigs() {
        let directory = std::env::temp_dir().join(format!(
            "rgbdns-dnssec-{}-{}",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::create_dir_all(&directory).unwrap();
        let key = directory.join("example.pk8");
        let zone_name = "example".parse().unwrap();
        let policy = generate_key(&zone_name, &key).unwrap();
        let zone = Zone::parse(
            "Zexample:ns.example:hostmaster.example:7:3600:600:86400:300:300\n\
             &example:192.0.2.53:ns.example:300\n\
             +www.example:192.0.2.1:300\n",
        )
        .unwrap();
        let records = sign_zone(&zone, &policy).unwrap();
        assert!(
            records
                .iter()
                .any(|record| record.rr_type() == RecordType::Dnskey)
        );
        assert!(
            records
                .iter()
                .any(|record| record.rr_type() == RecordType::Nsec)
        );
        assert!(
            records
                .iter()
                .any(|record| record.rr_type() == RecordType::Rrsig)
        );
        let ds = ds_line(&policy).unwrap();
        assert!(ds.starts_with("example. IN DS "));
        assert!(ds.contains(" 13 2 "));
        assert_eq!(
            records.iter().find_map(|record| match record.data {
                RData::Soa { serial, .. } => Some(serial),
                _ => None,
            }),
            Some(7)
        );
        fs::remove_dir_all(directory).unwrap();
    }
}
