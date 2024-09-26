use std::{error::Error, net::SocketAddr, sync::Arc};

use bevy::{
    app::{App, Plugin, PostStartup, Startup},
    prelude::{Res, Resource},
};
use connection::connect;
use tokio::runtime::{Handle, Runtime};

mod connection;
mod packet_io;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, print);
        build_plugin(app);
    }
}

fn build_plugin(app: &mut App) -> Result<(), Box<dyn Error>> {
    let runtime = Runtime::new()?;
    let handle = runtime.handle().clone();

    let network_state = SharedNetworkState {
        inner: Arc::new(SharedNetworkStateInner {
            _runtime: runtime,
            handle,
            addr: "0.0.0.0:25565".parse().unwrap(),
        }),
    };

    app.insert_resource(network_state.clone());

    // System for starting the accept loop.
    let start_accept_loop = move |shared: Res<SharedNetworkState>| {
        let _guard = shared.inner.handle.enter();

        // Connect to the server.
        tokio::spawn(connect(shared.clone()));
    };

    app.add_systems(PostStartup, start_accept_loop);

    Ok(())
}

#[derive(Debug, Resource, Clone)]
struct SharedNetworkState {
    inner: Arc<SharedNetworkStateInner>,
}

#[derive(Debug)]
struct SharedNetworkStateInner {
    _runtime: Runtime,
    handle: Handle,
    pub addr: SocketAddr,
}

fn print() {
    println!("Hello from protocol");
}
