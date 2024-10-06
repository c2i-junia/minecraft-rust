use crate::init::acquire_socket_by_port;
use clap::Parser;

mod chat;
mod dispatcher;
mod init;
mod player;
mod world;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    port: u16,
}

fn main() {
    let args = Args::parse();
    init::init(acquire_socket_by_port(args.port));
}
