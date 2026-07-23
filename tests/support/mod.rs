#![allow(dead_code)]

use rgbdns::{Message, Name, Question, RData, Record, RecordType, zone::Zone};

pub const ID: u16 = 0x4a6f;

pub fn zone() -> Zone {
    Zone::parse(
        ".example:192.0.2.53:ns:300\n\
         +www.example:192.0.2.1:300\n\
         3www.example:20010db8000000000000000000000001:300\n\
         Calias.example:www.example:300\n\
         'txt.example:first\\072segment:300\n\
         &child.example:192.0.2.54:ns.child.example:300\n",
    )
    .unwrap()
}

pub fn query(name: &str, qtype: RecordType) -> Message {
    Message {
        id: ID,
        flags: 0x0100,
        questions: vec![Question {
            name: name.parse().unwrap(),
            qtype,
            qclass: 1,
        }],
        ..Message::default()
    }
}

pub fn opt(payload: u16, version: u8, flags: u16, options: Vec<u8>) -> Record {
    Record {
        name: Name::root(),
        ttl: 0,
        data: RData::Opt {
            udp_payload: payload,
            extended_rcode: 0,
            version,
            flags,
            options,
        },
    }
}

pub fn response(message: &Message) -> Message {
    let wire = rgbdns::server::respond(&zone(), &message.encode().unwrap(), 4096).unwrap();
    Message::decode(&wire).unwrap()
}

pub fn rcode(message: &Message) -> u16 {
    message.flags & 0x000f
}

pub fn extended_rcode(message: &Message) -> u16 {
    let extension = message
        .additionals
        .iter()
        .find_map(|record| match record.data {
            RData::Opt { extended_rcode, .. } => Some(u16::from(extended_rcode)),
            _ => None,
        })
        .unwrap_or(0);
    extension << 4 | rcode(message)
}
