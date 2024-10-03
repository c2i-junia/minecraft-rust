use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

mod chat;
mod dispatcher;
mod init;

fn main() {
    init::init("127.0.0.1:5000".parse().unwrap());
}
