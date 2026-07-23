use crate::{
    Error, Result,
    packet::Message,
    zone::{Lookup, Zone},
};
use std::{
    io::{Read, Write},
    net::{TcpListener, UdpSocket},
    sync::Arc,
    thread,
};

pub fn respond(zone: &Zone, wire: &[u8], udp_limit: usize) -> Result<Vec<u8>> {
    let q = Message::decode(wire)?;
    if q.flags & 0x8000 != 0 || q.questions.len() != 1 {
        return Err(Error::Format("expected one query"));
    }
    let question = q.questions[0].clone();
    let mut r = Message {
        id: q.id,
        flags: 0x8000 | 0x0400 | (q.flags & 0x0100),
        questions: vec![question.clone()],
        ..Default::default()
    };
    if question.qclass != 1 {
        r.flags |= 4
    } else {
        match zone.lookup(&question.name, question.qtype) {
            Lookup::Answer(x) => r.answers = x,
            Lookup::NoData(soa) => {
                if let Some(x) = soa {
                    r.authorities.push(x)
                }
            }
            Lookup::NxDomain(soa) => {
                r.flags |= 3;
                if let Some(x) = soa {
                    r.authorities.push(x)
                }
            }
            Lookup::Refused => r.flags |= 5,
        }
    }
    let full = r.encode()?;
    if full.len() <= udp_limit {
        return Ok(full);
    }
    r.flags |= 0x0200;
    r.answers.clear();
    r.authorities.clear();
    r.additionals.clear();
    r.encode()
}
pub fn serve(zone: Zone, addr: &str) -> Result<()> {
    let zone = Arc::new(zone);
    let udp = UdpSocket::bind(addr)?;
    let tcp = TcpListener::bind(addr)?;
    let z = zone.clone();
    thread::spawn(move || {
        let mut b = [0u8; 65535];
        loop {
            if let Ok((n, peer)) = udp.recv_from(&mut b)
                && let Ok(r) = respond(&z, &b[..n], 1232)
            {
                let _ = udp.send_to(&r, peer);
            }
        }
    });
    for stream in tcp.incoming() {
        let z = zone.clone();
        thread::spawn(move || {
            if let Ok(mut s) = stream {
                let mut l = [0; 2];
                if s.read_exact(&mut l).is_ok() {
                    let mut b = vec![0; u16::from_be_bytes(l) as usize];
                    if s.read_exact(&mut b).is_ok()
                        && let Ok(r) = respond(&z, &b, 65535)
                    {
                        let _ = s.write_all(&(r.len() as u16).to_be_bytes());
                        let _ = s.write_all(&r);
                    }
                }
            }
        });
    }
    Ok(())
}
