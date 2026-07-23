use rgbdns::{
    packet::{Message, Question, RecordType},
    server,
    zone::Zone,
};
fn main() {
    let a = std::env::args().skip(1).collect::<Vec<_>>();
    if a.len() != 2 {
        eprintln!("usage: tinydns-get type name");
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
    let r = Message::decode(&server::respond(&z, &q.encode().unwrap(), 65535).unwrap()).unwrap();
    println!("{r:#?}")
}
