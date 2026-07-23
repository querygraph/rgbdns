//! Shared UDP/TCP transport for specialized authoritative responders.

use crate::Result;
use std::{
    io::{Read, Write},
    net::{TcpListener, UdpSocket},
    sync::Arc,
    thread,
};

pub type Handler = dyn Fn(&[u8], usize) -> Result<Vec<u8>> + Send + Sync;

pub fn serve(address: &str, handler: Arc<Handler>) -> Result<()> {
    let udp = UdpSocket::bind(address)?;
    let tcp = TcpListener::bind(address)?;
    let udp_handler = handler.clone();
    thread::spawn(move || {
        let mut packet = [0; 65535];
        loop {
            if let Ok((length, peer)) = udp.recv_from(&mut packet)
                && let Ok(response) = udp_handler(&packet[..length], 4096)
            {
                let _ = udp.send_to(&response, peer);
            }
        }
    });
    for stream in tcp.incoming() {
        let handler = handler.clone();
        thread::spawn(move || {
            if let Ok(mut stream) = stream {
                let mut length = [0; 2];
                if stream.read_exact(&mut length).is_ok() {
                    let mut packet = vec![0; u16::from_be_bytes(length) as usize];
                    if stream.read_exact(&mut packet).is_ok()
                        && let Ok(response) = handler(&packet, 65535)
                    {
                        let _ = stream.write_all(&(response.len() as u16).to_be_bytes());
                        let _ = stream.write_all(&response);
                    }
                }
            }
        });
    }
    Ok(())
}
