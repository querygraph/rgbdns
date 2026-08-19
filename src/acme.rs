//! Narrow RFC 2136/TSIG update support for ACME DNS-01 TXT records.

use crate::{Error, Name, Result, zone::Zone};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use fs2::FileExt;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs::{self, OpenOptions},
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
    sync::{Arc, Mutex, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

const OPCODE_UPDATE: u16 = 5 << 11;
const CLASS_IN: u16 = 1;
const CLASS_NONE: u16 = 254;
const CLASS_ANY: u16 = 255;
const TYPE_SOA: u16 = 6;
const TYPE_TXT: u16 = 16;
const TYPE_TSIG: u16 = 250;
const MAX_KEYS: usize = 256;
const MAX_VALUES: usize = 4096;
const MAX_OWNER_VALUES: usize = 128;
const MAX_STATE_BYTES: usize = 65_536;

type HmacSha256 = Hmac<Sha256>;
type Overlay = BTreeMap<Name, BTreeSet<Vec<u8>>>;

#[derive(Clone)]
struct Policy {
    key_name: Name,
    secret: Vec<u8>,
    zone: Name,
    ttl: u32,
}

#[derive(Clone)]
pub struct LiveZone(Arc<RwLock<Arc<Zone>>>);

impl LiveZone {
    pub fn new(zone: Zone) -> Self {
        Self(Arc::new(RwLock::new(Arc::new(zone))))
    }

    pub fn snapshot(&self) -> Arc<Zone> {
        self.0.read().expect("live zone lock poisoned").clone()
    }

    fn publish(&self, zone: Zone) {
        *self.0.write().expect("live zone lock poisoned") = Arc::new(zone);
    }
}

pub struct AcmeUpdates {
    policies: BTreeMap<Name, Policy>,
    base: Zone,
    live: LiveZone,
    state_dir: PathBuf,
    publication: Option<Publication>,
    state: Mutex<State>,
}

#[derive(Clone)]
pub struct Publication {
    command: PathBuf,
    data: PathBuf,
}

impl Publication {
    pub fn new(command: PathBuf, data: PathBuf) -> Result<Self> {
        if !command.is_absolute() || !data.is_absolute() {
            return Err(Error::InvalidRecord(
                "ACME publication command and data path must be absolute".into(),
            ));
        }
        Ok(Self { command, data })
    }

    fn run(
        &self,
        state_dir: &Path,
        overlay: &Overlay,
        serials: &BTreeMap<Name, u32>,
        policies: &BTreeMap<Name, Policy>,
    ) -> Result<Zone> {
        let status = Command::new(&self.command).arg(state_dir).status()?;
        if !status.success() {
            return Err(Error::Format("ACME publication command failed"));
        }
        let zone = Zone::from_file(&self.data)?;
        validate_published(&zone, overlay, serials, policies)?;
        Ok(zone)
    }
}

pub enum AdminAction<'a> {
    Present(&'a [u8]),
    Cleanup(&'a [u8]),
    Clear,
}

/// Sends one authenticated administrative mutation through the daemon's RFC
/// 2136 interface, so the durable and live states change in one transaction.
pub fn admin_update(
    config: impl AsRef<Path>,
    address: SocketAddr,
    zone: &Name,
    owner: &Name,
    action: AdminAction<'_>,
) -> Result<()> {
    let policies = parse_config_without_zone(&fs::read_to_string(config)?)?;
    let policy = policies
        .values()
        .find(|policy| policy.zone == *zone && authorized(policy, owner))
        .ok_or(Error::Format("no ACME key authorizes this owner"))?;
    let wire = admin_wire(policy, zone, owner, action)?;
    let mut stream = TcpStream::connect_timeout(&address, std::time::Duration::from_secs(10))?;
    stream.set_read_timeout(Some(std::time::Duration::from_secs(10)))?;
    stream.write_all(&(wire.len() as u16).to_be_bytes())?;
    stream.write_all(&wire)?;
    let mut length = [0; 2];
    stream.read_exact(&mut length)?;
    let mut response = vec![0; usize::from(u16::from_be_bytes(length))];
    stream.read_exact(&mut response)?;
    if response.len() < 12 || response[3] & 0x0f != 0 {
        return Err(Error::InvalidRecord(format!(
            "ACME UPDATE failed with DNS rcode {}",
            response.get(3).copied().unwrap_or(0) & 0x0f
        )));
    }
    Ok(())
}

fn admin_wire(
    policy: &Policy,
    zone: &Name,
    owner: &Name,
    action: AdminAction<'_>,
) -> Result<Vec<u8>> {
    let id = random_id()?;
    let now = unix_now();
    let mut wire = Vec::new();
    wire.extend_from_slice(&id.to_be_bytes());
    wire.extend_from_slice(&OPCODE_UPDATE.to_be_bytes());
    wire.extend_from_slice(&[0, 1, 0, 0, 0, 1, 0, 0]);
    encode_name(&mut wire, zone)?;
    wire.extend_from_slice(&TYPE_SOA.to_be_bytes());
    wire.extend_from_slice(&CLASS_IN.to_be_bytes());
    encode_name(&mut wire, owner)?;
    wire.extend_from_slice(&TYPE_TXT.to_be_bytes());
    let (class, ttl, rdata) = match action {
        AdminAction::Present(value) => (CLASS_IN, policy.ttl, encode_txt(value)?),
        AdminAction::Cleanup(value) => (CLASS_NONE, 0, encode_txt(value)?),
        AdminAction::Clear => (CLASS_ANY, 0, Vec::new()),
    };
    wire.extend_from_slice(&class.to_be_bytes());
    wire.extend_from_slice(&ttl.to_be_bytes());
    wire.extend_from_slice(&(rdata.len() as u16).to_be_bytes());
    wire.extend_from_slice(&rdata);
    let algorithm = Name::from_str("hmac-sha256.")?;
    let mut mac_input = wire.clone();
    append_tsig_variables(
        &mut mac_input,
        &policy.key_name,
        &algorithm,
        now,
        300,
        0,
        &[],
    )?;
    let mac = hmac(&policy.secret, &mac_input)?;
    wire[10..12].copy_from_slice(&1u16.to_be_bytes());
    append_tsig_record(&mut wire, policy, &algorithm, now, 300, &mac, id, 0, &[])?;
    Ok(wire)
}

pub fn list_overlay(state_dir: impl AsRef<Path>) -> Result<Vec<(Name, Vec<u8>)>> {
    Ok(read_overlay(&state_dir.as_ref().join("acme-overlay.data"))?
        .into_iter()
        .flat_map(|(owner, values)| values.into_iter().map(move |value| (owner.clone(), value)))
        .collect())
}

pub fn materialize_state(
    input: impl AsRef<Path>,
    config: impl AsRef<Path>,
    state_dir: impl AsRef<Path>,
    output: impl AsRef<Path>,
) -> Result<()> {
    let base = Zone::from_file(input)?;
    let policies = parse_config(&fs::read_to_string(config)?, &base)?;
    let state_dir = state_dir.as_ref();
    let overlay = read_overlay(&state_dir.join("acme-overlay.data"))?;
    let serials = read_serials(&state_dir.join("acme-serials"))?;
    validate_overlay(&overlay, &policies)?;
    let candidate = base.with_acme_overlay(&flatten(&overlay, &policies)?, &serials)?;
    let output = output.as_ref();
    let temporary = output.with_extension(format!("tmp.{}", std::process::id()));
    crate::axfr::write_zone(&candidate, output, &temporary)?;
    FileSync::sync_directory(
        output
            .parent()
            .ok_or(Error::Format("ACME materialized output has no parent"))?,
    )
}

struct State {
    overlay: Overlay,
    serials: BTreeMap<Name, u32>,
    recent_macs: VecDeque<(u64, Vec<u8>)>,
}

#[derive(Clone, Debug)]
struct WireRecord {
    name: Name,
    typ: u16,
    class: u16,
    ttl: u32,
    rdata: Vec<u8>,
}

struct ParsedUpdate {
    id: u16,
    flags: u16,
    zone: Name,
    prerequisites: Vec<WireRecord>,
    updates: Vec<WireRecord>,
    tsig: ParsedTsig,
    unsigned: Vec<u8>,
}

struct ParsedTsig {
    key_name: Name,
    algorithm: Name,
    time_signed: u64,
    fudge: u16,
    mac: Vec<u8>,
    original_id: u16,
    error: u16,
    other: Vec<u8>,
}

impl AcmeUpdates {
    pub fn from_file(
        config: impl AsRef<Path>,
        state_dir: impl AsRef<Path>,
        base: Zone,
        live: LiveZone,
    ) -> Result<Self> {
        Self::from_file_with_publication(config, state_dir, base, live, None)
    }

    pub fn from_file_with_publication(
        config: impl AsRef<Path>,
        state_dir: impl AsRef<Path>,
        base: Zone,
        live: LiveZone,
        publication: Option<Publication>,
    ) -> Result<Self> {
        let policies = parse_config(&fs::read_to_string(config)?, &base)?;
        let state_dir = state_dir.as_ref().to_path_buf();
        let overlay = read_overlay(&state_dir.join("acme-overlay.data"))?;
        let serials = read_serials(&state_dir.join("acme-serials"))?;
        validate_overlay(&overlay, &policies)?;
        let signed = policies
            .values()
            .any(|policy| base.is_dnssec_signed(&policy.zone));
        if signed && publication.is_none() {
            return Err(Error::InvalidRecord(
                "ACME updates to a signed zone require an external publication command".into(),
            ));
        }
        let published = if let Some(publication) = &publication {
            publication.run(&state_dir, &overlay, &serials, &policies)?
        } else {
            base.with_acme_overlay(&flatten(&overlay, &policies)?, &serials)?
        };
        live.publish(published);
        Ok(Self {
            policies,
            base,
            live,
            state_dir,
            publication,
            state: Mutex::new(State {
                overlay,
                serials,
                recent_macs: VecDeque::new(),
            }),
        })
    }

    pub fn is_update(wire: &[u8]) -> bool {
        wire.len() >= 4
            && u16::from_be_bytes([wire[2], wire[3]]) & 0x7800 == OPCODE_UPDATE
            && wire[2] & 0x80 == 0
    }

    pub fn handle(&self, wire: &[u8]) -> Result<Vec<u8>> {
        let parsed = match parse_update(wire) {
            Ok(value) => value,
            Err(_) => return bare_response(wire, 1),
        };
        let Some(policy) = self.policies.get(&parsed.tsig.key_name) else {
            return bare_response(wire, 9);
        };
        if parsed.zone != policy.zone
            || parsed.flags & 0x7800 != OPCODE_UPDATE
            || parsed.tsig.original_id != parsed.id
            || parsed.tsig.error != 0
            || !parsed.tsig.other.is_empty()
            || !is_hmac_sha256(&parsed.tsig.algorithm)
        {
            return signed_response(&parsed, policy, 9, 0, &[]);
        }
        let now = unix_now();
        if parsed.tsig.fudge > 300
            || now.abs_diff(parsed.tsig.time_signed) > u64::from(parsed.tsig.fudge)
        {
            return signed_response(&parsed, policy, 9, 18, &now.to_be_bytes()[2..]);
        }
        let expected = request_mac(&parsed, policy)?;
        if parsed.tsig.mac.len() != 32 || !constant_time_eq(&expected, &parsed.tsig.mac) {
            return signed_response(&parsed, policy, 9, 16, &[]);
        }
        let update_lock = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(self.state_dir.join("acme-update.lock"))?;
        update_lock.lock_exclusive()?;
        let mut state = self.state.lock().expect("ACME state lock poisoned");
        prune_replays(&mut state.recent_macs, now);
        if state
            .recent_macs
            .iter()
            .any(|(_, mac)| constant_time_eq(mac, &parsed.tsig.mac))
        {
            return signed_response(&parsed, policy, 9, 16, &[]);
        }
        state.recent_macs.push_back((now, parsed.tsig.mac.clone()));

        let rcode = match apply_update(&parsed, policy, &mut state.overlay) {
            Ok(()) => {
                let canonical = self.base.soa_serial(&policy.zone).unwrap_or(1);
                let previous = state
                    .serials
                    .get(&policy.zone)
                    .copied()
                    .unwrap_or(canonical);
                let serial = next_serial(previous, canonical, now as u32);
                state.serials.insert(policy.zone.clone(), serial);
                let records = flatten(&state.overlay, &self.policies)?;
                let candidate = self.base.with_acme_overlay(&records, &state.serials)?;
                persist(
                    &self.state_dir,
                    &state.overlay,
                    &state.serials,
                    &self.policies,
                )?;
                let published = if let Some(publication) = &self.publication {
                    publication.run(
                        &self.state_dir,
                        &state.overlay,
                        &state.serials,
                        &self.policies,
                    )
                } else {
                    Ok(candidate)
                };
                match published {
                    Ok(published) => {
                        self.live.publish(published);
                        0
                    }
                    Err(error) => {
                        eprintln!("rgbdns ACME publication: {error}");
                        2
                    }
                }
            }
            Err(rcode) => rcode,
        };
        signed_response(&parsed, policy, rcode, 0, &[])
    }
}

fn validate_published(
    zone: &Zone,
    overlay: &Overlay,
    serials: &BTreeMap<Name, u32>,
    policies: &BTreeMap<Name, Policy>,
) -> Result<()> {
    for policy in policies.values() {
        if !zone.is_dnssec_signed(&policy.zone) {
            return Err(Error::Format("ACME publication produced an unsigned zone"));
        }
        if serials
            .get(&policy.zone)
            .is_some_and(|serial| zone.soa_serial(&policy.zone) != Some(*serial))
        {
            return Err(Error::Format("ACME publication has the wrong SOA serial"));
        }
    }
    for (owner, values) in overlay {
        let found = match zone.lookup(owner, crate::RecordType::Txt) {
            crate::zone::Lookup::Answer(records) => records
                .into_iter()
                .filter_map(|record| match record.data {
                    crate::RData::Txt(chunks) => Some(chunks.into_iter().flatten().collect()),
                    _ => None,
                })
                .collect::<BTreeSet<Vec<u8>>>(),
            _ => BTreeSet::new(),
        };
        if !values.is_subset(&found) {
            return Err(Error::Format("ACME publication omitted challenge data"));
        }
    }
    Ok(())
}

fn parse_config(text: &str, base: &Zone) -> Result<BTreeMap<Name, Policy>> {
    let policies = parse_config_without_zone(text)?;
    for policy in policies.values() {
        if !base.is_authoritative(&policy.zone) {
            return Err(Error::InvalidRecord(format!(
                "ACME zone {} is not authoritative",
                policy.zone
            )));
        }
    }
    Ok(policies)
}

fn parse_config_without_zone(text: &str) -> Result<BTreeMap<Name, Policy>> {
    let mut policies = BTreeMap::new();
    for (index, raw) in text.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 6 {
            return Err(Error::InvalidRecord(format!(
                "ACME config line {} must have six fields",
                index + 1
            )));
        }
        let key_name = Name::from_str(fields[0])?;
        if !is_hmac_sha256(&Name::from_str(fields[1])?) || fields[4] != "_acme-challenge." {
            return Err(Error::InvalidRecord(format!(
                "unsupported ACME policy on line {}",
                index + 1
            )));
        }
        let secret = STANDARD
            .decode(fields[2])
            .map_err(|_| Error::InvalidRecord("invalid ACME TSIG secret".into()))?;
        if !(16..=64).contains(&secret.len()) {
            return Err(Error::InvalidRecord(
                "ACME TSIG secret must contain 16 through 64 bytes".into(),
            ));
        }
        let zone = Name::from_str(fields[3])?;
        let ttl = fields[5]
            .parse::<u32>()
            .map_err(|_| Error::InvalidRecord("invalid ACME TTL".into()))?;
        if !(30..=300).contains(&ttl) {
            return Err(Error::InvalidRecord(
                "ACME TTL must be 30 through 300".into(),
            ));
        }
        let policy = Policy {
            key_name: key_name.clone(),
            secret,
            zone,
            ttl,
        };
        if policies.insert(key_name, policy).is_some() || policies.len() > MAX_KEYS {
            return Err(Error::InvalidRecord(
                "duplicate or excessive ACME update keys".into(),
            ));
        }
    }
    if policies.is_empty() {
        return Err(Error::InvalidRecord(
            "ACME update config has no keys".into(),
        ));
    }
    Ok(policies)
}

fn authorized(policy: &Policy, owner: &Name) -> bool {
    if !owner.is_subdomain_of(&policy.zone) {
        return false;
    }
    let text = owner.to_string().to_ascii_lowercase();
    let suffix = policy.zone.to_string().to_ascii_lowercase();
    let relative = text.strip_suffix(&suffix).unwrap_or("");
    relative == "_acme-challenge." || relative.starts_with("_acme-challenge.")
}

fn apply_update(
    message: &ParsedUpdate,
    policy: &Policy,
    overlay: &mut Overlay,
) -> std::result::Result<(), u16> {
    for prerequisite in &message.prerequisites {
        if !authorized(policy, &prerequisite.name) || prerequisite.typ != TYPE_TXT {
            return Err(5);
        }
        let exists = overlay
            .get(&prerequisite.name)
            .is_some_and(|values| !values.is_empty());
        match (
            prerequisite.class,
            prerequisite.ttl,
            prerequisite.rdata.is_empty(),
        ) {
            (CLASS_ANY, 0, true) if !exists => return Err(8),
            (CLASS_NONE, 0, true) if exists => return Err(7),
            (CLASS_ANY | CLASS_NONE, 0, true) => {}
            _ => return Err(4),
        }
    }
    let mut candidate = overlay.clone();
    for update in &message.updates {
        if !authorized(policy, &update.name) || update.typ != TYPE_TXT {
            return Err(5);
        }
        match update.class {
            CLASS_IN if update.ttl != 0 => {
                let value = decode_txt(&update.rdata).map_err(|_| 1u16)?;
                candidate
                    .entry(update.name.clone())
                    .or_default()
                    .insert(value);
            }
            CLASS_NONE if update.ttl == 0 => {
                let value = decode_txt(&update.rdata).map_err(|_| 1u16)?;
                if let Some(values) = candidate.get_mut(&update.name) {
                    values.remove(&value);
                    if values.is_empty() {
                        candidate.remove(&update.name);
                    }
                }
            }
            CLASS_ANY if update.ttl == 0 && update.rdata.is_empty() => {
                candidate.remove(&update.name);
            }
            _ => return Err(1),
        }
    }
    validate_overlay_counts(&candidate).map_err(|_| 5u16)?;
    *overlay = candidate;
    Ok(())
}

fn decode_txt(rdata: &[u8]) -> Result<Vec<u8>> {
    let mut input = rdata;
    let mut value = Vec::new();
    while !input.is_empty() {
        let length = usize::from(input[0]);
        input = input
            .get(1..)
            .ok_or(Error::Format("truncated TXT update"))?;
        let chunk = input
            .get(..length)
            .ok_or(Error::Format("truncated TXT update"))?;
        value.extend_from_slice(chunk);
        input = &input[length..];
    }
    if value.is_empty() || value.len() > 255 {
        return Err(Error::Format("ACME TXT value length is invalid"));
    }
    Ok(value)
}

fn encode_txt(value: &[u8]) -> Result<Vec<u8>> {
    if value.is_empty() || value.len() > 255 {
        return Err(Error::Format("ACME TXT value length is invalid"));
    }
    let mut output = Vec::with_capacity(value.len() + 1);
    output.push(value.len() as u8);
    output.extend_from_slice(value);
    Ok(output)
}

fn validate_overlay_counts(overlay: &Overlay) -> Result<()> {
    if overlay.values().map(BTreeSet::len).sum::<usize>() > MAX_VALUES
        || overlay
            .values()
            .any(|values| values.len() > MAX_OWNER_VALUES)
    {
        return Err(Error::Format("ACME state limit exceeded"));
    }
    Ok(())
}

fn validate_overlay(overlay: &Overlay, policies: &BTreeMap<Name, Policy>) -> Result<()> {
    validate_overlay_counts(overlay)?;
    if overlay
        .keys()
        .any(|owner| !policies.values().any(|policy| authorized(policy, owner)))
    {
        return Err(Error::Format("ACME overlay contains unauthorized owner"));
    }
    Ok(())
}

fn flatten(
    overlay: &Overlay,
    policies: &BTreeMap<Name, Policy>,
) -> Result<Vec<(Name, Vec<u8>, u32)>> {
    let mut records = Vec::new();
    for (owner, values) in overlay {
        let ttl = policies
            .values()
            .filter(|policy| authorized(policy, owner))
            .map(|policy| policy.ttl)
            .min()
            .ok_or(Error::Format("ACME overlay owner has no policy"))?;
        for value in values {
            records.push((owner.clone(), value.clone(), ttl));
        }
    }
    Ok(records)
}

fn persist(
    directory: &Path,
    overlay: &Overlay,
    serials: &BTreeMap<Name, u32>,
    policies: &BTreeMap<Name, Policy>,
) -> Result<()> {
    fs::create_dir_all(directory)?;
    let mut overlay_text = String::from("# generated by rgbdns; do not edit\n");
    for (owner, value, ttl) in flatten(overlay, policies)? {
        overlay_text.push('\'');
        overlay_text.push_str(owner.to_string().trim_end_matches('.'));
        overlay_text.push(':');
        overlay_text.push_str(&escape_tinydns(&value));
        overlay_text.push(':');
        overlay_text.push_str(&ttl.to_string());
        overlay_text.push('\n');
    }
    if overlay_text.len() > MAX_STATE_BYTES {
        return Err(Error::Format("ACME overlay is too large"));
    }
    let serial_text = serials
        .iter()
        .map(|(zone, serial)| format!("{}\t{serial}\n", zone))
        .collect::<String>();
    atomic_write(
        &directory.join("acme-overlay.data"),
        overlay_text.as_bytes(),
    )?;
    atomic_write(&directory.join("acme-serials"), serial_text.as_bytes())?;
    Ok(())
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<()> {
    let temporary = path.with_extension(format!("new.{}", std::process::id()));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(&temporary, path)?;
    FileSync::sync_directory(
        path.parent()
            .ok_or(Error::Format("state path has no parent"))?,
    )?;
    Ok(())
}

struct FileSync;
impl FileSync {
    fn sync_directory(path: &Path) -> Result<()> {
        fs::File::open(path)?.sync_all()?;
        Ok(())
    }
}

fn read_overlay(path: &Path) -> Result<Overlay> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeMap::new()),
        Err(error) => return Err(error.into()),
    };
    if text.len() > MAX_STATE_BYTES {
        return Err(Error::Format("ACME overlay is too large"));
    }
    let mut overlay = Overlay::new();
    for line in text
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let body = line
            .strip_prefix('\'')
            .ok_or(Error::Format("invalid ACME overlay line"))?;
        let fields = split_tinydns(body)?;
        if fields.len() != 3 {
            return Err(Error::Format("invalid ACME overlay line"));
        }
        let owner = Name::from_str(&fields[0])?;
        let value = unescape_tinydns(&fields[1])?;
        if value.is_empty() || value.len() > 255 {
            return Err(Error::Format("invalid ACME overlay TXT"));
        }
        overlay.entry(owner).or_default().insert(value);
    }
    validate_overlay_counts(&overlay)?;
    Ok(overlay)
}

fn read_serials(path: &Path) -> Result<BTreeMap<Name, u32>> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeMap::new()),
        Err(error) => return Err(error.into()),
    };
    text.lines()
        .map(|line| {
            let (name, serial) = line
                .split_once('\t')
                .ok_or(Error::Format("invalid ACME serial state"))?;
            Ok((
                Name::from_str(name)?,
                serial
                    .parse::<u32>()
                    .map_err(|_| Error::Format("invalid ACME serial"))?,
            ))
        })
        .collect()
}

fn escape_tinydns(value: &[u8]) -> String {
    let mut output = String::new();
    for byte in value {
        if (b'!'..=b'~').contains(byte) && !matches!(byte, b':' | b'\\') {
            output.push(char::from(*byte));
        } else {
            output.push_str(&format!("\\{:03o}", byte));
        }
    }
    output
}

fn split_tinydns(value: &str) -> Result<Vec<String>> {
    let mut fields = vec![String::new()];
    let bytes = value.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\\' {
            let escape = bytes
                .get(index..index + 4)
                .ok_or(Error::Format("truncated ACME overlay escape"))?;
            fields
                .last_mut()
                .unwrap()
                .push_str(std::str::from_utf8(escape).map_err(|_| Error::Format("bad overlay"))?);
            index += 4;
        } else if bytes[index] == b':' {
            fields.push(String::new());
            index += 1;
        } else {
            fields.last_mut().unwrap().push(char::from(bytes[index]));
            index += 1;
        }
    }
    Ok(fields)
}

fn unescape_tinydns(value: &str) -> Result<Vec<u8>> {
    let bytes = value.as_bytes();
    let mut output = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] != b'\\' {
            output.push(bytes[index]);
            index += 1;
            continue;
        }
        let digits = bytes
            .get(index + 1..index + 4)
            .ok_or(Error::Format("truncated ACME overlay escape"))?;
        if !digits.iter().all(|byte| (b'0'..=b'7').contains(byte)) {
            return Err(Error::Format("invalid ACME overlay escape"));
        }
        output.push((digits[0] - b'0') * 64 + (digits[1] - b'0') * 8 + digits[2] - b'0');
        index += 4;
    }
    Ok(output)
}

fn parse_update(wire: &[u8]) -> Result<ParsedUpdate> {
    let mut reader = Reader::new(wire)?;
    let id = reader.u16()?;
    let flags = reader.u16()?;
    let qd = reader.u16()?;
    let an = reader.u16()?;
    let ns = reader.u16()?;
    let ar = reader.u16()?;
    if qd != 1 || ar == 0 || an > 64 || ns > 64 || ar > 16 {
        return Err(Error::Format("invalid UPDATE section count"));
    }
    let zone = reader.name()?;
    if reader.u16()? != TYPE_SOA || reader.u16()? != CLASS_IN {
        return Err(Error::Format("invalid UPDATE zone section"));
    }
    let prerequisites = (0..an)
        .map(|_| reader.record())
        .collect::<Result<Vec<_>>>()?;
    let updates = (0..ns)
        .map(|_| reader.record())
        .collect::<Result<Vec<_>>>()?;
    let mut additionals = Vec::new();
    let mut tsig_start = None;
    for index in 0..ar {
        let start = reader.position;
        let record = reader.record()?;
        if record.typ == TYPE_TSIG {
            if index + 1 != ar || tsig_start.is_some() {
                return Err(Error::Format("TSIG must be last"));
            }
            tsig_start = Some(start);
        }
        additionals.push(record);
    }
    if reader.position != wire.len() {
        return Err(Error::Format("trailing UPDATE data"));
    }
    let tsig_record = additionals
        .last()
        .filter(|record| record.typ == TYPE_TSIG && record.class == CLASS_ANY && record.ttl == 0)
        .ok_or(Error::Format("missing TSIG"))?;
    let tsig = parse_tsig(tsig_record)?;
    let start = tsig_start.ok_or(Error::Format("missing TSIG"))?;
    let mut unsigned = wire[..start].to_vec();
    unsigned[10..12].copy_from_slice(&(ar - 1).to_be_bytes());
    Ok(ParsedUpdate {
        id,
        flags,
        zone,
        prerequisites,
        updates,
        tsig,
        unsigned,
    })
}

fn parse_tsig(record: &WireRecord) -> Result<ParsedTsig> {
    let mut reader = Reader::new(&record.rdata)?;
    let algorithm = reader.name()?;
    let high = u64::from(reader.u16()?);
    let low = u64::from(reader.u32()?);
    let time_signed = (high << 32) | low;
    let fudge = reader.u16()?;
    let mac_length = usize::from(reader.u16()?);
    let mac = reader.bytes(mac_length)?.to_vec();
    let original_id = reader.u16()?;
    let error = reader.u16()?;
    let other_length = usize::from(reader.u16()?);
    let other = reader.bytes(other_length)?.to_vec();
    if reader.position != record.rdata.len() {
        return Err(Error::Format("trailing TSIG data"));
    }
    Ok(ParsedTsig {
        key_name: record.name.clone(),
        algorithm,
        time_signed,
        fudge,
        mac,
        original_id,
        error,
        other,
    })
}

struct Reader<'a> {
    wire: &'a [u8],
    position: usize,
}

impl<'a> Reader<'a> {
    fn new(wire: &'a [u8]) -> Result<Self> {
        if wire.len() < 12 && !wire.is_empty() {
            return Err(Error::Format("short UPDATE message"));
        }
        Ok(Self { wire, position: 0 })
    }
    fn u8(&mut self) -> Result<u8> {
        let value = *self
            .wire
            .get(self.position)
            .ok_or(Error::Format("truncated UPDATE"))?;
        self.position += 1;
        Ok(value)
    }
    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes([self.u8()?, self.u8()?]))
    }
    fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes([
            self.u8()?,
            self.u8()?,
            self.u8()?,
            self.u8()?,
        ]))
    }
    fn bytes(&mut self, length: usize) -> Result<&'a [u8]> {
        let value = self
            .wire
            .get(self.position..self.position + length)
            .ok_or(Error::Format("truncated UPDATE field"))?;
        self.position += length;
        Ok(value)
    }
    fn name(&mut self) -> Result<Name> {
        let mut labels = Vec::new();
        let mut cursor = self.position;
        let mut returned = None;
        let mut hops = 0;
        loop {
            hops += 1;
            if hops > 128 {
                return Err(Error::Format("UPDATE name compression loop"));
            }
            let length = *self
                .wire
                .get(cursor)
                .ok_or(Error::Format("truncated UPDATE name"))?;
            if length & 0xc0 == 0xc0 {
                let next = *self
                    .wire
                    .get(cursor + 1)
                    .ok_or(Error::Format("truncated UPDATE pointer"))?;
                let target = (usize::from(length & 0x3f) << 8) | usize::from(next);
                if target >= cursor {
                    return Err(Error::Format("forward UPDATE pointer"));
                }
                returned.get_or_insert(cursor + 2);
                cursor = target;
                continue;
            }
            if length & 0xc0 != 0 {
                return Err(Error::Format("invalid UPDATE label"));
            }
            cursor += 1;
            if length == 0 {
                self.position = returned.unwrap_or(cursor);
                return Name::from_labels(labels);
            }
            let label = self
                .wire
                .get(cursor..cursor + usize::from(length))
                .ok_or(Error::Format("truncated UPDATE label"))?;
            labels.push(label.to_vec());
            cursor += usize::from(length);
        }
    }
    fn record(&mut self) -> Result<WireRecord> {
        let name = self.name()?;
        let typ = self.u16()?;
        let class = self.u16()?;
        let ttl = self.u32()?;
        let length = usize::from(self.u16()?);
        let rdata = self.bytes(length)?.to_vec();
        Ok(WireRecord {
            name,
            typ,
            class,
            ttl,
            rdata,
        })
    }
}

fn request_mac(message: &ParsedUpdate, policy: &Policy) -> Result<Vec<u8>> {
    let mut input = message.unsigned.clone();
    append_tsig_variables(
        &mut input,
        &message.tsig.key_name,
        &message.tsig.algorithm,
        message.tsig.time_signed,
        message.tsig.fudge,
        message.tsig.error,
        &message.tsig.other,
    )?;
    hmac(&policy.secret, &input)
}

fn signed_response(
    request: &ParsedUpdate,
    policy: &Policy,
    rcode: u16,
    tsig_error: u16,
    other: &[u8],
) -> Result<Vec<u8>> {
    let now = unix_now();
    let mut wire = Vec::new();
    wire.extend_from_slice(&request.id.to_be_bytes());
    wire.extend_from_slice(&(0x8000 | OPCODE_UPDATE | (rcode & 0x0f)).to_be_bytes());
    wire.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 1]);
    let mut mac_input = Vec::new();
    mac_input.extend_from_slice(&(request.tsig.mac.len() as u16).to_be_bytes());
    mac_input.extend_from_slice(&request.tsig.mac);
    mac_input.extend_from_slice(&wire[..10]);
    mac_input.extend_from_slice(&[0, 0]);
    append_tsig_variables(
        &mut mac_input,
        &policy.key_name,
        &request.tsig.algorithm,
        now,
        request.tsig.fudge.min(300),
        tsig_error,
        other,
    )?;
    let mac = hmac(&policy.secret, &mac_input)?;
    append_tsig_record(
        &mut wire,
        policy,
        &request.tsig.algorithm,
        now,
        request.tsig.fudge.min(300),
        &mac,
        request.id,
        tsig_error,
        other,
    )?;
    Ok(wire)
}

#[allow(clippy::too_many_arguments)]
fn append_tsig_record(
    wire: &mut Vec<u8>,
    policy: &Policy,
    algorithm: &Name,
    time: u64,
    fudge: u16,
    mac: &[u8],
    original_id: u16,
    error: u16,
    other: &[u8],
) -> Result<()> {
    encode_name(wire, &policy.key_name)?;
    wire.extend_from_slice(&TYPE_TSIG.to_be_bytes());
    wire.extend_from_slice(&CLASS_ANY.to_be_bytes());
    wire.extend_from_slice(&0u32.to_be_bytes());
    let mut rdata = Vec::new();
    encode_name(&mut rdata, algorithm)?;
    rdata.extend_from_slice(&time.to_be_bytes()[2..]);
    rdata.extend_from_slice(&fudge.to_be_bytes());
    rdata.extend_from_slice(&(mac.len() as u16).to_be_bytes());
    rdata.extend_from_slice(mac);
    rdata.extend_from_slice(&original_id.to_be_bytes());
    rdata.extend_from_slice(&error.to_be_bytes());
    rdata.extend_from_slice(&(other.len() as u16).to_be_bytes());
    rdata.extend_from_slice(other);
    wire.extend_from_slice(&(rdata.len() as u16).to_be_bytes());
    wire.extend_from_slice(&rdata);
    Ok(())
}

fn bare_response(request: &[u8], rcode: u16) -> Result<Vec<u8>> {
    if request.len() < 4 {
        return Err(Error::Format("short UPDATE"));
    }
    let mut response = vec![request[0], request[1]];
    response.extend_from_slice(&(0x8000 | OPCODE_UPDATE | (rcode & 0x0f)).to_be_bytes());
    response.extend_from_slice(&[0; 8]);
    Ok(response)
}

fn append_tsig_variables(
    output: &mut Vec<u8>,
    key: &Name,
    algorithm: &Name,
    time: u64,
    fudge: u16,
    error: u16,
    other: &[u8],
) -> Result<()> {
    encode_name_canonical(output, key)?;
    output.extend_from_slice(&CLASS_ANY.to_be_bytes());
    output.extend_from_slice(&0u32.to_be_bytes());
    encode_name_canonical(output, algorithm)?;
    output.extend_from_slice(&time.to_be_bytes()[2..]);
    output.extend_from_slice(&fudge.to_be_bytes());
    output.extend_from_slice(&error.to_be_bytes());
    output.extend_from_slice(&(other.len() as u16).to_be_bytes());
    output.extend_from_slice(other);
    Ok(())
}

fn encode_name(output: &mut Vec<u8>, name: &Name) -> Result<()> {
    for label in name.labels() {
        output.push(u8::try_from(label.len()).map_err(|_| Error::Format("long TSIG label"))?);
        output.extend_from_slice(label);
    }
    output.push(0);
    Ok(())
}

fn encode_name_canonical(output: &mut Vec<u8>, name: &Name) -> Result<()> {
    for label in name.labels() {
        output.push(u8::try_from(label.len()).map_err(|_| Error::Format("long TSIG label"))?);
        output.extend(label.iter().map(u8::to_ascii_lowercase));
    }
    output.push(0);
    Ok(())
}

fn hmac(secret: &[u8], input: &[u8]) -> Result<Vec<u8>> {
    let mut mac =
        HmacSha256::new_from_slice(secret).map_err(|_| Error::Format("invalid TSIG key length"))?;
    mac.update(input);
    Ok(mac.finalize().into_bytes().to_vec())
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    if left.len() != right.len() {
        return false;
    }
    left.iter()
        .zip(right)
        .fold(0u8, |difference, (a, b)| difference | (a ^ b))
        == 0
}

fn is_hmac_sha256(name: &Name) -> bool {
    matches!(
        name.to_string().to_ascii_lowercase().as_str(),
        "hmac-sha256." | "hmac-sha256.sig-alg.reg.int."
    )
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn next_serial(previous: u32, canonical: u32, clock: u32) -> u32 {
    let mut next = previous.wrapping_add(1);
    for candidate in [canonical.wrapping_add(1), clock] {
        if candidate != next && candidate.wrapping_sub(next) < (1 << 31) {
            next = candidate;
        }
    }
    next
}

fn random_id() -> Result<u16> {
    let mut bytes = [0; 2];
    getrandom::fill(&mut bytes).map_err(|_| Error::Format("random source unavailable"))?;
    Ok(u16::from_be_bytes(bytes))
}

fn prune_replays(replays: &mut VecDeque<(u64, Vec<u8>)>, now: u64) {
    while replays
        .front()
        .is_some_and(|(timestamp, _)| now.saturating_sub(*timestamp) > 300)
    {
        replays.pop_front();
    }
    while replays.len() > 4096 {
        replays.pop_front();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tinydns_overlay_escape_round_trips() {
        let value = b"token:with\\bytes\n\0";
        assert_eq!(unescape_tinydns(&escape_tinydns(value)).unwrap(), value);
    }

    #[test]
    fn serials_advance_across_wrap_using_rfc1982_order() {
        assert_eq!(next_serial(10, 5, 9), 11);
        assert_eq!(next_serial(u32::MAX, u32::MAX - 1, 0), 0);
        assert_eq!(next_serial(10, 20, 15), 21);
    }

    #[test]
    fn owner_policy_is_scoped_to_acme_label() {
        let policy = Policy {
            key_name: "key.".parse().unwrap(),
            secret: vec![0; 32],
            zone: "example.org.".parse().unwrap(),
            ttl: 60,
        };
        assert!(authorized(
            &policy,
            &"_acme-challenge.example.org".parse().unwrap()
        ));
        assert!(authorized(
            &policy,
            &"_acme-challenge.www.example.org".parse().unwrap()
        ));
        assert!(!authorized(&policy, &"www.example.org".parse().unwrap()));
        assert!(!authorized(
            &policy,
            &"_acme-challenge.example.net".parse().unwrap()
        ));
    }

    #[test]
    fn signed_updates_preserve_concurrent_values_and_persist() {
        let unique = format!(
            "rgbdns-acme-test-{}-{}",
            std::process::id(),
            random_id().unwrap()
        );
        let directory = std::env::temp_dir().join(unique);
        fs::create_dir(&directory).unwrap();
        let secret = vec![7u8; 32];
        let encoded = STANDARD.encode(&secret);
        let config = directory.join("acme.conf");
        fs::write(
            &config,
            format!("certbot. hmac-sha256. {encoded} example.org. _acme-challenge. 60\n"),
        )
        .unwrap();
        let base = Zone::parse(".example.org:192.0.2.1:ns.example.org\n").unwrap();
        let live = LiveZone::new(base.clone());
        let updates =
            Arc::new(AcmeUpdates::from_file(&config, &directory, base, live.clone()).unwrap());
        let policy = updates.policies.values().next().unwrap().clone();
        let zone: Name = "example.org".parse().unwrap();
        let owner: Name = "_acme-challenge.example.org".parse().unwrap();
        let mut replay = None;
        for value in [b"first".as_slice(), b"second".as_slice()] {
            let wire = admin_wire(&policy, &zone, &owner, AdminAction::Present(value)).unwrap();
            let response = updates.handle(&wire).unwrap();
            assert_eq!(response[3] & 0x0f, 0);
            replay.get_or_insert(wire);
        }
        assert_eq!(updates.handle(&replay.unwrap()).unwrap()[3] & 0x0f, 9);
        let records = live.snapshot().lookup(&owner, crate::RecordType::Txt);
        assert!(matches!(records, crate::zone::Lookup::Answer(values) if values.len() == 2));
        let wire = admin_wire(&policy, &zone, &owner, AdminAction::Cleanup(b"first")).unwrap();
        assert_eq!(updates.handle(&wire).unwrap()[3] & 0x0f, 0);
        let persisted = read_overlay(&directory.join("acme-overlay.data")).unwrap();
        assert_eq!(persisted[&owner].len(), 1);
        assert!(persisted[&owner].contains(b"second".as_slice()));

        let threads = (0..8)
            .map(|index| {
                let updates = updates.clone();
                let policy = policy.clone();
                let zone = zone.clone();
                let owner = owner.clone();
                std::thread::spawn(move || {
                    let value = format!("parallel-{index}");
                    let wire = admin_wire(
                        &policy,
                        &zone,
                        &owner,
                        AdminAction::Present(value.as_bytes()),
                    )
                    .unwrap();
                    assert_eq!(updates.handle(&wire).unwrap()[3] & 0x0f, 0);
                })
            })
            .collect::<Vec<_>>();
        for thread in threads {
            thread.join().unwrap();
        }
        assert_eq!(
            read_overlay(&directory.join("acme-overlay.data")).unwrap()[&owner].len(),
            9
        );
    }

    #[cfg(unix)]
    #[test]
    fn signed_zone_requires_an_explicit_publication_hook() {
        let directory = std::env::temp_dir().join(format!(
            "rgbdns-acme-signed-test-{}-{}",
            std::process::id(),
            random_id().unwrap()
        ));
        fs::create_dir(&directory).unwrap();
        let secret = STANDARD.encode([7u8; 32]);
        let config = directory.join("acme.conf");
        fs::write(
            &config,
            format!("certbot. hmac-sha256. {secret} example.org. _acme-challenge. 60\n"),
        )
        .unwrap();
        let apex: Name = "example.org".parse().unwrap();
        let dnssec_policy = crate::dnssec::generate_key(&apex, &directory.join("key.pk8")).unwrap();
        let unsigned = Zone::parse(
            "Zexample.org:ns.example.org:hostmaster.example.org:7:3600:600:86400:300:300\n\
             &example.org:192.0.2.53:ns.example.org:300\n",
        )
        .unwrap();
        let signed = Zone::from_compiled_records(
            crate::dnssec::sign_zone(&unsigned, &dnssec_policy)
                .unwrap()
                .into_iter()
                .map(|record| (record, crate::zone::RecordMetadata::default()))
                .collect(),
            Vec::new(),
            Vec::new(),
        );
        let live = LiveZone::new(signed.clone());
        let result = AcmeUpdates::from_file(&config, &directory, signed, live);
        assert!(matches!(
            result,
            Err(Error::InvalidRecord(message))
                if message.contains("require an external publication command")
        ));
        fs::remove_dir_all(directory).unwrap();
    }

    #[test]
    fn wrong_owner_is_refused_without_state_change() {
        let policy = Policy {
            key_name: "key.".parse().unwrap(),
            secret: vec![9; 32],
            zone: "example.org.".parse().unwrap(),
            ttl: 60,
        };
        let wire = admin_wire(
            &policy,
            &policy.zone,
            &"www.example.org".parse().unwrap(),
            AdminAction::Present(b"token"),
        )
        .unwrap();
        let parsed = parse_update(&wire).unwrap();
        let mut overlay = Overlay::new();
        assert_eq!(apply_update(&parsed, &policy, &mut overlay), Err(5));
        assert!(overlay.is_empty());
    }
}
