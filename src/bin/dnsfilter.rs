use rgbdns::{RData, RecordType, client};
use std::{
    io::{BufRead, Write},
    net::Ipv4Addr,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("dnsfilter: fatal: {error}");
        std::process::exit(111);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (concurrency, line_limit) = options()?;
    let stdin = std::io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        lines.push(line?);
        if lines.len() == line_limit {
            process(std::mem::take(&mut lines), concurrency)?;
        }
    }
    if !lines.is_empty() {
        process(lines, concurrency)?;
    }
    Ok(())
}

fn options() -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut concurrency = 10;
    let mut line_limit = 1000;
    let mut arguments = std::env::args().skip(1);
    while let Some(argument) = arguments.next() {
        let value = arguments
            .next()
            .ok_or("usage: dnsfilter [ -c concurrency ] [ -l lines ]")?;
        match argument.as_str() {
            "-c" => concurrency = value.parse::<usize>()?.clamp(1, 1000),
            "-l" => line_limit = value.parse::<usize>()?.clamp(1, 1_000_000),
            _ => return Err("usage: dnsfilter [ -c concurrency ] [ -l lines ]".into()),
        }
    }
    Ok((concurrency, line_limit))
}

fn process(lines: Vec<String>, concurrency: usize) -> Result<(), Box<dyn std::error::Error>> {
    let lines = Arc::new(lines);
    let next = Arc::new(AtomicUsize::new(0));
    let (sender, receiver) = mpsc::channel();
    thread::scope(|scope| {
        for _ in 0..concurrency.min(lines.len()) {
            let lines = lines.clone();
            let next = next.clone();
            let sender = sender.clone();
            scope.spawn(move || {
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(line) = lines.get(index) else {
                        break;
                    };
                    let _ = sender.send((index, filter_line(line)));
                }
            });
        }
    });
    drop(sender);
    let mut output = vec![String::new(); lines.len()];
    for (index, line) in receiver {
        output[index] = line;
    }
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for line in output {
        writeln!(stdout, "{line}")?;
    }
    Ok(())
}

fn filter_line(line: &str) -> String {
    let split = line.find([' ', '\t']).unwrap_or(line.len());
    let (left, right) = line.split_at(split);
    let Ok(address) = left.parse::<Ipv4Addr>() else {
        return line.to_owned();
    };
    let octets = address.octets();
    let reverse = format!(
        "{}.{}.{}.{}.in-addr.arpa",
        octets[3], octets[2], octets[1], octets[0]
    );
    match reverse
        .parse()
        .and_then(|name| client::recursive(name, RecordType::Ptr))
    {
        Ok(response) => {
            let names = response
                .answers
                .iter()
                .filter_map(|record| match &record.data {
                    RData::Name(RecordType::Ptr, name) => Some(name.to_string()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(",");
            if names.is_empty() {
                line.to_owned()
            } else {
                format!("{left}={names}{right}")
            }
        }
        Err(error) => format!("{left}:{}{right}", error.to_string().replace(' ', "-")),
    }
}
