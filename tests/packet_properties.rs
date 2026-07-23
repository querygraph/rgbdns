use proptest::prelude::*;
use rgbdns::Message;

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
}
