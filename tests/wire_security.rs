use rgbdns::{Message, Name};

fn standard_header(qd: u16, an: u16, ns: u16, ar: u16) -> Vec<u8> {
    let mut wire = Vec::with_capacity(12);
    for value in [0x1234, 0, qd, an, ns, ar] {
        wire.extend(value.to_be_bytes());
    }
    wire
}

#[test]
fn rfc1035_maximum_name_is_accepted_and_one_octet_more_is_rejected() {
    let maximum = [
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61),
    ]
    .join(".");
    let name: Name = maximum.parse().unwrap();
    assert_eq!(name.to_string().len(), maximum.len() + 1);

    let too_long = [
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(62),
    ]
    .join(".");
    assert!(too_long.parse::<Name>().is_err());
}

#[test]
fn malformed_wire_corpus_is_rejected_without_partial_acceptance() {
    let mut count_bomb = standard_header(65, 0, 0, 0);
    let mut reserved_label = standard_header(1, 0, 0, 0);
    reserved_label.extend([0x40, 0, 1, 0, 1]);
    let mut truncated_pointer = standard_header(1, 0, 0, 0);
    truncated_pointer.push(0xc0);
    let mut forward_pointer = standard_header(1, 0, 0, 0);
    forward_pointer.extend([0xc0, 14, 0, 1, 0, 1]);
    let mut invalid_a_length = standard_header(0, 1, 0, 0);
    invalid_a_length.extend([0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 3, 1, 2, 3]);
    let mut truncated_txt = standard_header(0, 1, 0, 0);
    truncated_txt.extend([0, 0, 16, 0, 1, 0, 0, 0, 1, 0, 2, 4, b'x']);
    let mut bad_opt_owner = standard_header(0, 0, 0, 1);
    bad_opt_owner.extend([1, b'x', 0, 0, 41, 4, 208, 0, 0, 0, 0, 0, 0]);
    let mut truncated_option = standard_header(0, 0, 0, 1);
    truncated_option.extend([0, 0, 41, 4, 208, 0, 0, 0, 0, 0, 5, 0, 1, 0, 2, 0]);
    let mut trailing = standard_header(0, 0, 0, 0);
    trailing.push(0);
    count_bomb.extend([0; 5]);

    for (name, wire) in [
        ("short header", vec![0; 11]),
        ("section count bomb", count_bomb),
        ("reserved label", reserved_label),
        ("truncated pointer", truncated_pointer),
        ("forward pointer", forward_pointer),
        ("invalid A length", invalid_a_length),
        ("truncated TXT chunk", truncated_txt),
        ("non-root OPT owner", bad_opt_owner),
        ("truncated EDNS option", truncated_option),
        ("trailing bytes", trailing),
    ] {
        assert!(Message::decode(&wire).is_err(), "{name}");
    }
}

#[test]
fn compression_pointer_must_target_a_previous_name_boundary() {
    // The question starts with a one-byte label containing NUL. Offset 13
    // (inside that label) happens to decode as a syntactically valid name,
    // but RFC 1035 pointers may only refer to a prior name occurrence.
    let mut wire = standard_header(1, 1, 0, 0);
    wire.extend([1, 0, 0, 0, 1, 0, 1]);
    wire.extend([
        0xc0, 0x0d, // illegal pointer into the question label
        0, 1, // A
        0, 1, // IN
        0, 0, 0, 1, // TTL
        0, 4, 192, 0, 2, 1,
    ]);
    assert!(Message::decode(&wire).is_err());
}

#[test]
fn decoder_rejects_every_truncation_of_a_valid_structured_packet() {
    let message = Message {
        id: 0x1234,
        questions: vec![rgbdns::Question {
            name: "www.example".parse().unwrap(),
            qtype: rgbdns::RecordType::A,
            qclass: 1,
        }],
        answers: vec![rgbdns::Record {
            name: "www.example".parse().unwrap(),
            ttl: 300,
            data: rgbdns::RData::A("192.0.2.1".parse().unwrap()),
        }],
        ..Message::default()
    };
    let wire = message.encode().unwrap();
    for length in 0..wire.len() {
        assert!(Message::decode(&wire[..length]).is_err(), "length {length}");
    }
    assert_eq!(Message::decode(&wire).unwrap(), message);
}
