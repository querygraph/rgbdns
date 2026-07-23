mod support;

use rgbdns::{Message, RData, RecordType};
use support::{ID, extended_rcode, opt, query, rcode, response, zone};

#[test]
fn rfc1035_response_identity_flags_and_question_are_coherent() {
    let request = query("WwW.ExAmPlE", RecordType::A);
    let response = response(&request);

    assert_eq!(response.id, ID);
    assert_ne!(response.flags & 0x8000, 0, "QR");
    assert_ne!(response.flags & 0x0400, 0, "AA");
    assert_eq!(response.flags & 0x0200, 0, "TC");
    assert_ne!(response.flags & 0x0100, 0, "RD is copied");
    assert_eq!(response.flags & 0x0080, 0, "authorities do not offer RA");
    assert_eq!(response.flags & 0x0070, 0, "reserved Z bits");
    assert_eq!(response.questions, request.questions);
    assert_eq!(response.questions[0].name.to_string(), "WwW.ExAmPlE.");
}

#[test]
fn rfc9619_standard_queries_require_exactly_one_question() {
    let mut empty = query("example", RecordType::A);
    empty.questions.clear();
    let answer = response(&empty);
    assert_eq!(rcode(&answer), 1);
    assert!(answer.questions.is_empty());

    let mut multiple = query("example", RecordType::A);
    multiple.questions.push(multiple.questions[0].clone());
    let answer = response(&multiple);
    assert_eq!(rcode(&answer), 1);
    assert!(answer.questions.is_empty());
}

#[test]
fn rfc8906_unknown_opcode_is_notimp_even_when_its_body_is_unknown() {
    // An unknown opcode may define a different body layout. Only the header is
    // safe to interpret, so a truncated standard-question body must not turn
    // NOTIMP into FORMERR.
    let wire = [
        0x4a, 0x6f, 0x38, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];
    let response = rgbdns::server::respond(&zone(), &wire, 4096).unwrap();
    let response = Message::decode(&response).unwrap();
    assert_eq!(rcode(&response), 4);
    assert_eq!(response.id, ID);
}

#[test]
fn rfc8906_all_unimplemented_opcodes_get_notimp_without_flag_reflection() {
    for opcode in 1_u16..=15 {
        let mut wire = [0_u8; 12];
        wire[..2].copy_from_slice(&ID.to_be_bytes());
        wire[2..4].copy_from_slice(&(opcode << 11 | 0x0170).to_be_bytes());
        let answer =
            Message::decode(&rgbdns::server::respond(&zone(), &wire, 4096).unwrap()).unwrap();
        assert_eq!(rcode(&answer), 4, "opcode {opcode}");
        assert_eq!(answer.flags & 0x7800, opcode << 11, "opcode {opcode}");
        assert_eq!(answer.flags & 0x0070, 0, "opcode {opcode}");
    }
}

#[test]
fn rfc8906_unknown_header_flags_do_not_suppress_or_taint_answers() {
    for flags in [0x0020, 0x0010, 0x0040, 0x0070, 0x0120, 0x0170] {
        let mut request = query("www.example", RecordType::A);
        request.flags = flags;
        let response = response(&request);
        assert_eq!(rcode(&response), 0, "request flags {flags:#06x}");
        assert_eq!(response.answers.len(), 1);
        assert_eq!(response.flags & 0x0070, 0);
        assert_eq!(response.flags & 0x0100, flags & 0x0100);
    }
}

#[test]
fn rfc8906_unknown_types_use_name_existence_not_notimp() {
    let existing = response(&query("www.example", RecordType::Unknown(65_000)));
    assert_eq!(rcode(&existing), 0);
    assert!(existing.answers.is_empty());
    assert_eq!(existing.authorities[0].rr_type(), RecordType::Soa);

    let absent = response(&query("absent.example", RecordType::Unknown(65_000)));
    assert_eq!(rcode(&absent), 3);
    assert!(absent.answers.is_empty());
    assert_eq!(absent.authorities[0].rr_type(), RecordType::Soa);
}

#[test]
fn rfc1035_unsupported_query_class_gets_notimp() {
    let mut request = query("www.example", RecordType::A);
    request.questions[0].qclass = 3;
    let response = response(&request);
    assert_eq!(rcode(&response), 4);
}

#[test]
fn rfc2308_nodata_and_nxdomain_are_distinct_and_include_soa() {
    let nodata = response(&query("www.example", RecordType::Mx));
    assert_eq!(rcode(&nodata), 0);
    assert!(nodata.answers.is_empty());
    assert_eq!(nodata.authorities[0].rr_type(), RecordType::Soa);

    let nxdomain = response(&query("missing.example", RecordType::A));
    assert_eq!(rcode(&nxdomain), 3);
    assert!(nxdomain.answers.is_empty());
    assert_eq!(nxdomain.authorities[0].rr_type(), RecordType::Soa);
}

#[test]
fn rfc2308_negative_soa_ttl_is_the_minimum_of_soa_ttl_and_minimum() {
    let zone =
        rgbdns::zone::Zone::parse("Zexample:ns.example:hostmaster.example:1:2:3:4:60:3600\n")
            .unwrap();
    let request = query("missing.example", RecordType::A);
    let wire = rgbdns::server::respond(&zone, &request.encode().unwrap(), 4096).unwrap();
    let answer = Message::decode(&wire).unwrap();
    assert_eq!(rcode(&answer), 3);
    assert_eq!(answer.authorities[0].ttl, 60);
}

#[test]
fn rfc6891_edns_is_acknowledged_and_do_is_copied() {
    for flags in [0, 0x8000, 0x7fff, 0xffff] {
        let mut request = query("www.example", RecordType::A);
        request.additionals.push(opt(1232, 0, flags, Vec::new()));
        let response = response(&request);
        let RData::Opt {
            udp_payload,
            version,
            flags: response_flags,
            ..
        } = response.additionals.last().unwrap().data
        else {
            panic!("missing OPT response");
        };
        assert_eq!(udp_payload, 1232);
        assert_eq!(version, 0);
        assert_eq!(response_flags, flags & 0x8000);
    }
}

#[test]
fn rfc6891_unknown_well_formed_options_are_ignored() {
    let mut request = query("www.example", RecordType::A);
    request
        .additionals
        .push(opt(1232, 0, 0, vec![0xfd, 0xe8, 0, 3, 1, 2, 3]));
    let response = response(&request);
    assert_eq!(rcode(&response), 0);
    assert_eq!(response.answers.len(), 1);
    assert!(matches!(
        &response.additionals.last().unwrap().data,
        RData::Opt { options, .. } if options.is_empty()
    ));
}

#[test]
fn rfc6891_badvers_uses_the_extended_response_code() {
    let mut request = query("www.example", RecordType::A);
    request.additionals.push(opt(1232, 7, 0, Vec::new()));
    let response = response(&request);
    assert_eq!(extended_rcode(&response), 16);
    assert!(response.answers.is_empty());
}

#[test]
fn rfc8906_badvers_survives_unknown_edns_flags_and_options() {
    let mut request = query("www.example", RecordType::A);
    request
        .additionals
        .push(opt(1232, 255, 0xffff, vec![0xfd, 0xe8, 0, 3, 1, 2, 3]));
    let response = response(&request);
    assert_eq!(extended_rcode(&response), 16);
    assert!(matches!(
        response.additionals.last().unwrap().data,
        RData::Opt {
            version: 0,
            flags: 0x8000,
            ..
        }
    ));
}

#[test]
fn rfc6891_duplicate_or_misplaced_opt_is_formerr() {
    let mut duplicate = query("www.example", RecordType::A);
    duplicate.additionals.push(opt(1232, 0, 0, Vec::new()));
    duplicate.additionals.push(opt(1232, 0, 0, Vec::new()));
    assert_eq!(rcode(&response(&duplicate)), 1);

    let mut answer_opt = query("www.example", RecordType::A);
    answer_opt.answers.push(opt(1232, 0, 0, Vec::new()));
    assert_eq!(rcode(&response(&answer_opt)), 1);

    let mut authority_opt = query("www.example", RecordType::A);
    authority_opt.authorities.push(opt(1232, 0, 0, Vec::new()));
    assert_eq!(rcode(&response(&authority_opt)), 1);
}

#[test]
fn rfc6891_advertised_udp_limit_is_honored_with_parseable_truncation() {
    let mut data = ".example::ns.example\n".to_owned();
    for index in 1..=200 {
        data.push_str(&format!("+many.example:192.0.2.{}:300\n", index % 250));
    }
    let zone = rgbdns::zone::Zone::parse(&data).unwrap();
    let mut request = query("many.example", RecordType::A);
    request.additionals.push(opt(768, 0, 0x8000, Vec::new()));
    let wire = rgbdns::server::respond(&zone, &request.encode().unwrap(), 4096).unwrap();
    assert!(wire.len() <= 768);
    let response = Message::decode(&wire).unwrap();
    assert_ne!(response.flags & 0x0200, 0);
    assert!(matches!(
        response.additionals.last().map(|record| &record.data),
        Some(RData::Opt { .. })
    ));
}

#[test]
fn rfc3597_unknown_rdata_roundtrips_losslessly() {
    let original = Message {
        id: ID,
        answers: vec![rgbdns::Record {
            name: "opaque.example".parse().unwrap(),
            ttl: 1234,
            data: RData::Opaque(RecordType::Unknown(65_000), vec![0, 1, 2, 0xff]),
        }],
        ..Message::default()
    };
    assert_eq!(
        Message::decode(&original.encode().unwrap()).unwrap(),
        original
    );
}

#[test]
fn rfc4343_name_comparison_is_ascii_case_insensitive() {
    let upper: rgbdns::Name = "MiXeD.Example".parse().unwrap();
    let lower: rgbdns::Name = "mixed.example".parse().unwrap();
    assert_eq!(upper, lower);

    let answer = response(&query("WwW.Example", RecordType::A));
    assert_eq!(answer.answers[0].name.to_string(), "WwW.Example.");
}

#[test]
fn rfc4592_closest_encloser_blocks_a_higher_wildcard() {
    let zone = rgbdns::zone::Zone::parse(
        ".example::ns.example\n\
         +*.example:192.0.2.1\n\
         +node.branch.example:192.0.2.2\n",
    )
    .unwrap();
    let request = query("missing.branch.example", RecordType::A);
    let answer =
        Message::decode(&rgbdns::server::respond(&zone, &request.encode().unwrap(), 4096).unwrap())
            .unwrap();
    assert_eq!(rcode(&answer), 3);
    assert!(answer.answers.is_empty());
}

#[test]
fn rfc2181_referral_contains_only_in_bailiwick_glue() {
    let request = query("host.child.example", RecordType::A);
    let response = response(&request);
    assert_eq!(response.flags & 0x0400, 0);
    assert_eq!(response.authorities.len(), 1);
    assert!(response.additionals.iter().all(|record| {
        record
            .name
            .is_subdomain_of(&"child.example".parse().unwrap())
            && matches!(record.rr_type(), RecordType::A | RecordType::Aaaa)
    }));
}

#[test]
fn rfc2181_rrset_ttls_are_equalized_and_duplicate_records_suppressed() {
    let zone = rgbdns::zone::Zone::parse(
        ".example::ns.example\n\
         +multi.example:192.0.2.1:600\n\
         +multi.example:192.0.2.2:300\n\
         +multi.example:192.0.2.1:900\n",
    )
    .unwrap();
    let request = query("multi.example", RecordType::A);
    let wire = rgbdns::server::respond(&zone, &request.encode().unwrap(), 4096).unwrap();
    let response = Message::decode(&wire).unwrap();
    assert_eq!(response.answers.len(), 2);
    assert!(response.answers.iter().all(|record| record.ttl == 300));
}

#[test]
fn rfc2181_cname_cannot_coexist_with_other_data_or_multiple_targets() {
    assert!(
        rgbdns::zone::Zone::parse(
            ".example::ns.example\n\
             Calias.example:first.example\n\
             +alias.example:192.0.2.1\n",
        )
        .is_err()
    );
    assert!(
        rgbdns::zone::Zone::parse(
            ".example::ns.example\n\
             Calias.example:first.example\n\
             Calias.example:second.example\n",
        )
        .is_err()
    );
}
