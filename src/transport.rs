//! Shared bounded UDP and persistent DNS-over-TCP serving.

use crate::Result;
use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream, UdpSocket},
    sync::Arc,
    thread,
    time::Duration,
};

pub(crate) type Handler = dyn Fn(&[u8], usize, SocketAddr) -> Result<Vec<u8>> + Send + Sync;
pub(crate) type StreamHandler =
    dyn Fn(&[u8], SocketAddr) -> Result<Option<Vec<Vec<u8>>>> + Send + Sync;

const TCP_WORKERS: usize = 32;
const TCP_TIMEOUT: Duration = Duration::from_secs(10);

pub(crate) fn serve(
    address: &str,
    handler: Arc<Handler>,
    stream_handler: Option<Arc<StreamHandler>>,
) -> Result<()> {
    serve_sockets(
        UdpSocket::bind(address)?,
        TcpListener::bind(address)?,
        handler,
        stream_handler,
    )
}

pub(crate) fn serve_sockets(
    udp: UdpSocket,
    tcp: TcpListener,
    handler: Arc<Handler>,
    stream_handler: Option<Arc<StreamHandler>>,
) -> Result<()> {
    let udp_handler = handler.clone();
    thread::spawn(move || serve_udp(udp, &udp_handler));

    let mut workers = Vec::with_capacity(TCP_WORKERS);
    for _ in 0..TCP_WORKERS {
        let handler = handler.clone();
        let stream_handler = stream_handler.clone();
        let listener = tcp.try_clone()?;
        workers.push(thread::spawn(move || {
            for stream in listener.incoming() {
                let Ok(mut stream) = stream else {
                    continue;
                };
                let client = match stream.peer_addr() {
                    Ok(peer) => peer,
                    Err(_) => continue,
                };
                let _ = stream.set_read_timeout(Some(TCP_TIMEOUT));
                let _ = stream.set_write_timeout(Some(TCP_TIMEOUT));
                serve_tcp_connection(&mut stream, client, &handler, stream_handler.as_ref());
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
            && let Ok(response) = handler(&packet[..length], 4096, peer)
        {
            let _ = socket.send_to(&response, peer);
        }
    }
}

fn serve_tcp_connection(
    stream: &mut TcpStream,
    client: SocketAddr,
    handler: &Arc<Handler>,
    stream_handler: Option<&Arc<StreamHandler>>,
) {
    loop {
        let mut length = [0; 2];
        if stream.read_exact(&mut length).is_err() {
            return;
        }
        let mut packet = vec![0; u16::from_be_bytes(length) as usize];
        if stream.read_exact(&mut packet).is_err() {
            return;
        }
        let responses = match stream_handler
            .map(|stream_handler| stream_handler(&packet, client))
            .transpose()
        {
            Ok(Some(Some(responses))) => responses,
            Ok(_) => match handler(&packet, u16::MAX as usize, client) {
                Ok(response) => vec![response],
                Err(_) => continue,
            },
            Err(_) => return,
        };
        for response in responses {
            let Ok(response_length) = u16::try_from(response.len()) else {
                return;
            };
            if stream.write_all(&response_length.to_be_bytes()).is_err()
                || stream.write_all(&response).is_err()
            {
                return;
            }
        }
    }
}
