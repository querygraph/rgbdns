use rgbdns::{
    packet::{Message, Question, RecordType},
    server,
    zone::Zone,
};
fn main() {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    if !(2..=3).contains(&a.len()) {
        eprintln!("usage: tinydns-get type name [ip]");
        std::process::exit(100)
    }
    let typ = a[0].parse::<RecordType>().unwrap();
    let q = Message {
        id: 1,
        questions: vec![Question {
            name: a[1].parse().unwrap(),
            qtype: typ,
            qclass: 1,
        }],
        ..Default::default()
    };
    let z = Zone::from_file("data").unwrap();
    let wire = if let Some(address) = a.get(2) {
        server::respond_from(&z, &q.encode().unwrap(), 65535, address.parse().unwrap())
    } else {
        server::respond(&z, &q.encode().unwrap(), 65535)
    }
    .unwrap();
    let r = Message::decode(&wire).unwrap();
    println!("{r:#?}")
}
