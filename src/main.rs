use std::io::Result;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

mod cache;
mod protocol;
mod server;
use cache::DnsCache;
use server::utils::handle_query;

fn main() -> Result<()> {
    let socket = UdpSocket::bind(("0.0.0.0", 2053))?;
    let cache = Arc::new(Mutex::new(DnsCache::new()));

    loop {
        match handle_query(&socket, cache.clone()) {
            Ok(_) => {}
            Err(e) => eprintln!("An error occurred: {}", e),
        }
    }
}
