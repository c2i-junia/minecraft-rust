use crate::init::acquire_socket_by_port;
use clap::Parser;
use shared::GameServerConfig;

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

    #[arg(short, long, default_value = "default")]
    world: String,
}

fn main() {
    let args = Args::parse();
    let socket = acquire_socket_by_port(args.port);
    init::init(
        socket,
        GameServerConfig {
            world_name: args.world,
            is_solo: false,
        },
    );
}
