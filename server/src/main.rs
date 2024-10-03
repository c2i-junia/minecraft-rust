use crate::init::acquire_local_ephemeral_udp_socket;

mod chat;
mod dispatcher;
mod init;

fn main() {
    init::init(acquire_local_ephemeral_udp_socket());
}
