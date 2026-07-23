use rgbdns::{Name, RData, RecordType, client};
use std::net::{IpAddr, SocketAddr};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnstrace: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments.len() < 3 {
        return Err("usage: dnstrace type name rootip ...".into());
    }
    let record_type = arguments[0].parse::<RecordType>()?;
    let name = arguments[1].parse::<Name>()?;
    let mut servers = arguments[2..]
        .iter()
        .map(|value| {
            value
                .parse::<IpAddr>()
                .map(|address| SocketAddr::new(address, 53))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let prefix = format!("{} {}: ", record_type.code(), name);
    for depth in 0..64 {
        let server = servers[depth % servers.len()];
        println!("{prefix}QUERY:{server}");
        let response = client::query(name.clone(), record_type, false, &[server])?;
        for record in &response.answers {
            println!("{prefix}ANSWER:{record:?}");
        }
        if !response.answers.is_empty() || response.flags & 15 != 0 {
            println!("{prefix}RCODE:{}", response.flags & 15);
            return Ok(());
        }
        let nameservers = response
            .authorities
            .iter()
            .filter_map(|record| match &record.data {
                RData::Name(RecordType::Ns, target) => {
                    println!("{prefix}NS:{}:{target}", record.name);
                    Some(target.clone())
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        let mut next = Vec::new();
        for record in &response.additionals {
            if nameservers.contains(&record.name) {
                match record.data {
                    RData::A(address) => {
                        println!("{prefix}A:{}:{address}", record.name);
                        next.push(SocketAddr::new(IpAddr::V4(address), 53));
                    }
                    RData::Aaaa(address) => {
                        println!("{prefix}AAAA:{}:{address}", record.name);
                        next.push(SocketAddr::new(IpAddr::V6(address), 53));
                    }
                    _ => {}
                }
            }
        }
        if next.is_empty() {
            for nameserver in nameservers {
                if let Ok(addresses) = client::recursive(nameserver.clone(), RecordType::A) {
                    for record in addresses.answers {
                        if let RData::A(address) = record.data {
                            println!("{prefix}A:{nameserver}:{address}");
                            next.push(SocketAddr::new(IpAddr::V4(address), 53));
                        }
                    }
                }
                if let Ok(addresses) = client::recursive(nameserver.clone(), RecordType::Aaaa) {
                    for record in addresses.answers {
                        if let RData::Aaaa(address) = record.data {
                            println!("{prefix}AAAA:{nameserver}:{address}");
                            next.push(SocketAddr::new(IpAddr::V6(address), 53));
                        }
                    }
                }
            }
        }
        if next.is_empty() {
            return Err("referral contains no reachable IPv4 nameserver".into());
        }
        servers = next;
        if depth == 63 {
            return Err("trace depth limit exceeded".into());
        }
    }
    Err("trace depth limit exceeded".into())
}
