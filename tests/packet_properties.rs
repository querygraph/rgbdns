use proptest::prelude::*;
use rgbdns::{Message, Question, RData, Record, RecordType};
use std::net::{Ipv4Addr, Ipv6Addr};

fn dns_name() -> impl Strategy<Value = rgbdns::Name> {
    prop::collection::vec("[a-z0-9]{1,20}", 1..=4)
        .prop_map(|labels| labels.join(".").parse().unwrap())
}

fn record() -> impl Strategy<Value = Record> {
    (
        dns_name(),
        any::<u32>(),
        prop_oneof![
            any::<[u8; 4]>().prop_map(|octets| RData::A(Ipv4Addr::from(octets))),
            any::<[u8; 16]>().prop_map(|octets| RData::Aaaa(Ipv6Addr::from(octets))),
            prop::collection::vec(prop::collection::vec(any::<u8>(), 0..=64), 0..=4)
                .prop_map(RData::Txt),
            prop::collection::vec(any::<u8>(), 0..=128)
                .prop_map(|bytes| RData::Opaque(RecordType::Unknown(65_000), bytes)),
        ],
    )
        .prop_map(|(name, ttl, data)| Record { name, ttl, data })
}

fn structured_message() -> impl Strategy<Value = Message> {
    (
        any::<u16>(),
        any::<u16>(),
        prop::collection::vec((dns_name(), 0_u16..=u16::MAX), 0..=4),
        prop::collection::vec(record(), 0..=16),
        prop::collection::vec(record(), 0..=8),
        prop::collection::vec(record(), 0..=8),
    )
        .prop_map(
            |(id, flags, questions, answers, authorities, additionals)| Message {
                id,
                flags,
                questions: questions
                    .into_iter()
                    .map(|(name, code)| Question {
                        name,
                        qtype: RecordType::from_code(code),
                        qclass: 1,
                    })
                    .collect(),
                answers,
                authorities,
                additionals,
            },
        )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

    #[test]
    fn arbitrary_packets_never_panic(bytes in prop::collection::vec(any::<u8>(), 0..4096)) {
        let _ = Message::decode(&bytes);
    }

    #[test]
    fn accepted_packets_are_stably_reparseable(bytes in prop::collection::vec(any::<u8>(), 0..2048)) {
        if let Ok(message) = Message::decode(&bytes) {
            let encoded = message.encode().expect("decoded messages must be encodable");
            let reparsed = Message::decode(&encoded).expect("encoder must produce valid wire data");
            prop_assert_eq!(reparsed, message);
        }
    }

    #[test]
    fn structured_messages_roundtrip_without_semantic_loss(message in structured_message()) {
        let encoded = message.encode().expect("bounded generated message must encode");
        let decoded = Message::decode(&encoded).expect("generated encoding must decode");
        prop_assert_eq!(decoded, message);
    }

    #[test]
    fn ascii_case_changes_do_not_change_name_identity(
        labels in prop::collection::vec("[a-z]{1,20}", 1..=8),
        choices in prop::collection::vec(any::<bool>(), 1..=160),
    ) {
        let lower = labels.join(".");
        let mut index = 0;
        let mixed = lower
            .bytes()
            .map(|byte| {
                if byte.is_ascii_alphabetic() {
                    let upper = choices[index % choices.len()];
                    index += 1;
                    if upper { byte.to_ascii_uppercase() } else { byte }
                } else {
                    byte
                }
            })
            .collect::<Vec<_>>();
        let mixed = String::from_utf8(mixed).unwrap();
        prop_assert_eq!(
            lower.parse::<rgbdns::Name>().unwrap(),
            mixed.parse::<rgbdns::Name>().unwrap()
        );
    }
}
