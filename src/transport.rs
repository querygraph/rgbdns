//! Shared bounded UDP and persistent DNS-over-TCP serving.

use crate::Result;
use std::{
    io::{Read, Write},
    net::{IpAddr, TcpListener, TcpStream, UdpSocket},
    sync::Arc,
    thread,
    time::Duration,
};

pub(crate) type Handler = dyn Fn(&[u8], usize, IpAddr) -> Result<Vec<u8>> + Send + Sync;

const TCP_WORKERS: usize = 32;
const TCP_TIMEOUT: Duration = Duration::from_secs(10);

pub(crate) fn serve(address: &str, handler: Arc<Handler>) -> Result<()> {
    serve_sockets(
        UdpSocket::bind(address)?,
        TcpListener::bind(address)?,
        handler,
    )
}

pub(crate) fn serve_sockets(udp: UdpSocket, tcp: TcpListener, handler: Arc<Handler>) -> Result<()> {
    let udp_handler = handler.clone();
    thread::spawn(move || serve_udp(udp, &udp_handler));

    let mut workers = Vec::with_capacity(TCP_WORKERS);
    for _ in 0..TCP_WORKERS {
        let handler = handler.clone();
        let listener = tcp.try_clone()?;
        workers.push(thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else {
                    continue;
                };
                let client = match stream.peer_addr() {
                    Ok(peer) => peer.ip(),
                    Err(_) => continue,
                };
                let _ = stream.set_read_timeout(Some(TCP_TIMEOUT));
                let _ = stream.set_write_timeout(Some(TCP_TIMEOUT));
                serve_tcp_connection(&mut stream, client, &handler);
            }
        }));
    }
    for worker in workers {
        let _ = worker.join();
    }
    Ok(())
}

fn serve_udp(socket: UdpSocket, handler: &Arc<Handler>) {
    let mut packet = [0; u16::MAX as usize];
    loop {
        if let Ok((length, peer)) = socket.recv_from(&mut packet)
            && let Ok(response) = handler(&packet[..length], 4096, peer.ip())
        {
            let _ = socket.send_to(&response, peer);
        }
    }
}

fn serve_tcp_connection(stream: &mut TcpStream, client: IpAddr, handler: &Arc<Handler>) {
    loop {
        let mut length = [0; 2];
        if stream.read_exact(&mut length).is_err() {
            return;
        }
        let mut packet = vec![0; u16::from_be_bytes(length) as usize];
        if stream.read_exact(&mut packet).is_err() {
            return;
        }
        let Ok(response) = handler(&packet, u16::MAX as usize, client) else {
            continue;
        };
        let Ok(response_length) = u16::try_from(response.len()) else {
            return;
        };
        let mut framed = Vec::with_capacity(response.len() + 2);
        framed.extend(response_length.to_be_bytes());
        framed.extend(response);
        if stream.write_all(&framed).is_err() {
            return;
        }
    }
}
