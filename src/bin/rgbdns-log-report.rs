use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

#[derive(Debug, Default)]
struct Count {
    total: u64,
    clients: HashSet<String>,
}

#[derive(Debug)]
struct Options {
    zones: PathBuf,
    date: String,
}

fn usage() -> &'static str {
    "usage: rgbdns-log-report --zones FILE --date YYYY-MM-DD"
}

fn options<I>(mut args: I) -> Result<Options, String>
where
    I: Iterator<Item = String>,
{
    let mut zones = None;
    let mut date = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--zones" => zones = args.next().map(PathBuf::from),
            "--date" => date = args.next(),
            "-h" | "--help" => return Err(usage().to_owned()),
            _ => return Err(format!("unknown argument: {arg}\n{}", usage())),
        }
    }
    Ok(Options {
        zones: zones.ok_or_else(|| format!("--zones is required\n{}", usage()))?,
        date: date.ok_or_else(|| format!("--date is required\n{}", usage()))?,
    })
}

fn normalize_name(value: &str) -> Option<String> {
    let value = value.trim().trim_end_matches('.').to_ascii_lowercase();
    (!value.is_empty()
        && value.len() <= 253
        && value
            .split('.')
            .all(|label| !label.is_empty() && label.len() <= 63))
    .then_some(value)
}

fn read_zones(reader: impl BufRead) -> Result<Vec<String>, String> {
    let mut zones = HashSet::new();
    for (number, line) in reader.lines().enumerate() {
        let line = line.map_err(|error| format!("cannot read zones: {error}"))?;
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let zone = normalize_name(line)
            .ok_or_else(|| format!("invalid zone on line {}: {line}", number + 1))?;
        zones.insert(zone);
    }
    let mut zones: Vec<_> = zones.into_iter().collect();
    zones.sort_by_key(|zone| Reverse(zone.len()));
    if zones.is_empty() {
        return Err("zone list is empty".to_owned());
    }
    Ok(zones)
}

fn containing_zone<'a>(name: &str, zones: &'a [String]) -> Option<&'a str> {
    zones
        .iter()
        .find(|zone| {
            name == zone.as_str()
                || name
                    .strip_suffix(zone.as_str())
                    .is_some_and(|prefix| prefix.ends_with('.'))
        })
        .map(String::as_str)
}

fn parse_query(line: &str) -> Option<(&str, String)> {
    let mut fields = line.split_whitespace();
    let request = fields.next()?;
    if fields.next()? != "+" {
        return None;
    }
    fields.next()?;
    let name = normalize_name(fields.next()?)?;
    if fields.next().is_some() {
        return None;
    }
    let mut request_fields = request.split(':');
    let client = request_fields.next()?;
    let port = request_fields.next()?;
    let id = request_fields.next()?;
    if request_fields.next().is_some()
        || !matches!(client.len(), 8 | 32)
        || !client.bytes().all(|byte| byte.is_ascii_hexdigit())
        || port.len() != 4
        || id.len() != 4
    {
        return None;
    }
    Some((client, name))
}

fn aggregate(reader: impl BufRead, zones: &[String]) -> Result<HashMap<String, Count>, String> {
    let mut counts: HashMap<String, Count> = zones
        .iter()
        .map(|zone| (zone.clone(), Count::default()))
        .collect();
    for line in reader.lines() {
        let line = line.map_err(|error| format!("cannot read query log: {error}"))?;
        let Some((client, name)) = parse_query(&line) else {
            continue;
        };
        let Some(zone) = containing_zone(&name, zones) else {
            continue;
        };
        let count = counts.get_mut(zone).expect("configured zone must exist");
        count.total += 1;
        count.clients.insert(client.to_ascii_lowercase());
    }
    Ok(counts)
}

fn render(date: &str, counts: HashMap<String, Count>) -> String {
    let mut counts: Vec<_> = counts.into_iter().collect();
    counts.sort_by(|(left_zone, left), (right_zone, right)| {
        right
            .total
            .cmp(&left.total)
            .then_with(|| right.clients.len().cmp(&left.clients.len()))
            .then_with(|| left_zone.cmp(right_zone))
    });
    let total: u64 = counts.iter().map(|(_, count)| count.total).sum();
    let mut output = format!(
        "rgbdns daily query report for {date}\n\n{:<40} {:>12} {:>12}\n{:-<40} {:-<12} {:-<12}\n",
        "DOMAIN", "TOTAL", "UNIQUE", "", "", ""
    );
    for (zone, count) in counts {
        output.push_str(&format!(
            "{zone:<40} {:>12} {:>12}\n",
            count.total,
            count.clients.len()
        ));
    }
    output.push_str(&format!(
        "\nTotal accepted DNS queries: {total}\n\n\
         Unique counts are distinct client/resolver IP addresses per domain.\n\
         They are not unique people or HTTP pageviews.\n"
    ));
    output
}

fn run() -> Result<(), String> {
    let options = options(env::args().skip(1))?;
    let zones_file = File::open(&options.zones)
        .map_err(|error| format!("cannot open {}: {error}", options.zones.display()))?;
    let zones = read_zones(BufReader::new(zones_file))?;
    let counts = aggregate(io::stdin().lock(), &zones)?;
    print!("{}", render(&options.date, counts));
    Ok(())
}

fn main() {
    if env::args()
        .nth(1)
        .is_some_and(|argument| matches!(argument.as_str(), "-h" | "--help"))
    {
        println!("{}", usage());
        return;
    }
    if let Err(error) = run() {
        eprintln!("rgbdns-log-report: {error}");
        std::process::exit(100);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn aggregates_subdomains_by_longest_zone_and_sorts_by_total() {
        let zones = read_zones(Cursor::new("example\nexample.com\nquiet.test\n")).unwrap();
        let logs = concat!(
            "7f000001:e214:0018 + 0001 example.com\n",
            "7f000001:e215:0019 + 001c www.example.com\n",
            "08080808:0035:0020 + 0001 api.example.com.\n",
            "7f000001:e216:0021 + 0001 example\n",
            "7f000001:e217:0022 - 0001 refused.example.com\n",
            "starting tinydns\n",
        );
        let report = render("2026-08-04", aggregate(Cursor::new(logs), &zones).unwrap());
        let example_com = report.find("example.com").unwrap();
        let example = report.find("example                             ").unwrap();
        assert!(example_com < example);
        assert!(report[example_com..].starts_with("example.com"));
        assert!(report[example_com..].contains("           3            2"));
        assert!(report[example..].contains("           1            1"));
        assert!(report.contains("quiet.test"));
        assert!(report.contains("Total accepted DNS queries: 4"));
    }

    #[test]
    fn accepts_ipv6_clients_and_rejects_bad_records() {
        let zones = vec!["example.com".to_owned()];
        let logs = concat!(
            "20010db8000000000000000000000001:0035:0001 + 0001 example.com\n",
            "nothex:0035:0001 + 0001 example.com\n",
            "7f000001:0035:0001 + 0001 outside.test\n",
        );
        let counts = aggregate(Cursor::new(logs), &zones).unwrap();
        assert_eq!(counts["example.com"].total, 1);
        assert_eq!(counts["example.com"].clients.len(), 1);
    }
}
