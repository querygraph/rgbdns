//! The deterministic `walldns` IPv4 mapping service.

use crate::{Error, Message, Name, RData, Record, RecordType, Result, rbl::numeric_prefix};
use std::net::Ipv4Addr;

pub fn respond(wire: &[u8], _limit: usize) -> Result<Vec<u8>> {
    let query = Message::decode(wire)?;
    if query.flags & 0x8000 != 0 || query.questions.len() != 1 {
        return Err(Error::Format("expected one query"));
    }
    let question = query.questions[0].clone();
    let mut response = Message {
        id: query.id,
        flags: 0x8000 | 0x0400,
        questions: vec![question.clone()],
        ..Default::default()
    };
    let wants_a = matches!(question.qtype, RecordType::A | RecordType::Any);
    let wants_ptr = matches!(question.qtype, RecordType::Ptr | RecordType::Any);
    if question.qclass != 1 || (!wants_a && !wants_ptr) {
        response.flags = 0x8000 | 5;
        return response.encode();
    }

    let root = Name::root();
    if let Some(labels) = numeric_prefix(&question.name, &root, 4)
        && labels.len() == 4
    {
        if wants_a {
            response.answers.push(Record {
                name: question.name,
                ttl: 655_360,
                data: RData::A(Ipv4Addr::new(labels[0], labels[1], labels[2], labels[3])),
            });
        }
        return response.encode();
    }

    let reverse: Name = "in-addr.arpa".parse()?;
    if let Some(labels) = numeric_prefix(&question.name, &reverse, 4) {
        if wants_a && labels.len() == 4 {
            response.answers.push(Record {
                name: question.name.clone(),
                ttl: 655_360,
                data: RData::A(Ipv4Addr::new(labels[3], labels[2], labels[1], labels[0])),
            });
        }
        if wants_ptr {
            response.answers.push(Record {
                name: question.name.clone(),
                ttl: 655_360,
                data: RData::Name(RecordType::Ptr, question.name),
            });
        }
        return response.encode();
    }

    response.flags = 0x8000 | 5;
    response.encode()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Question;

    fn query(name: &str, record_type: RecordType) -> Message {
        let wire = Message {
            id: 9,
            questions: vec![Question {
                name: name.parse().unwrap(),
                qtype: record_type,
                qclass: 1,
            }],
            ..Default::default()
        }
        .encode()
        .unwrap();
        Message::decode(&respond(&wire, 512).unwrap()).unwrap()
    }

    #[test]
    fn direct_and_reverse_wall_mappings() {
        let direct = query("1.2.3.4", RecordType::A);
        assert_eq!(direct.answers[0].data, RData::A(Ipv4Addr::new(1, 2, 3, 4)));
        let reverse = query("4.3.2.1.in-addr.arpa", RecordType::Any);
        assert_eq!(reverse.answers[0].data, RData::A(Ipv4Addr::new(1, 2, 3, 4)));
        assert!(matches!(
            &reverse.answers[1].data,
            RData::Name(RecordType::Ptr, name)
                if name.to_string() == "4.3.2.1.in-addr.arpa."
        ));
    }

    #[test]
    fn partial_reverse_names_have_self_ptr() {
        let response = query("3.2.1.in-addr.arpa", RecordType::Ptr);
        assert_eq!(response.answers.len(), 1);
        assert_eq!(
            response.answers[0].name,
            "3.2.1.in-addr.arpa".parse().unwrap()
        );
    }
}
