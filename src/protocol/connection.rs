use std::{error::Error, net::SocketAddr};

use tokio::net::TcpStream;
use tracing::{info, trace};
use valence_protocol::{
    packets::{
        handshaking::{handshake_c2s::HandshakeNextState, HandshakeC2s},
        login::{
            LoginCompressionS2c, LoginDisconnectS2c, LoginHelloC2s, LoginHelloS2c, LoginSuccessS2c,
        },
    },
    uuid::Uuid,
    Bounded, PacketDecoder, PacketEncoder, PROTOCOL_VERSION,
};

use crate::protocol::packet_io::PacketIo;

use super::SharedNetworkState;

pub(crate) async fn connect(shared: SharedNetworkState) {
    let stream = match TcpStream::connect(shared.inner.addr).await {
        Ok(stream) => stream,
        Err(e) => {
            tracing::error!("Failed to connect to server: {:?}", e);
            return;
        }
    };

    trace!("Connected to server");

    let io = PacketIo::new(stream, PacketEncoder::new(), PacketDecoder::new());

    match handshake(io, shared).await {
        Ok(_) => (),
        Err(e) => tracing::error!("Handshake failed: {:?}", e),
    }
}

async fn handshake(mut io: PacketIo, shared: SharedNetworkState) -> Result<(), Box<dyn Error>> {
    let ip = shared.inner.addr.ip().to_string();
    let handshake_packet = HandshakeC2s {
        protocol_version: PROTOCOL_VERSION.into(),
        server_address: ip.as_str().into(),
        server_port: shared.inner.addr.port().into(),
        next_state: HandshakeNextState::Login,
    };

    io.send_packet(&handshake_packet).await?;

    let login_hello_packet = LoginHelloC2s {
        username: "test".into(),
        // unused by notchian server
        profile_id: Some(Uuid::from_u128(0)),
    };

    io.send_packet(&login_hello_packet).await?;

    // if let Ok(disconect_packet) = io.recv_packet::<LoginDisconnectS2c>().await {
    //     info!("Disconnect: {}", disconect_packet.reason);
    // } else if let Ok(response_hello) = io.recv_packet::<LoginHelloS2c>().await {
    //     info!(" LoginHelloS2c: {:?}", response_hello);
    // }

    loop {
        match io.recv_frame().await {
            Ok(frame) => {
                if let Ok(disconect_packet) = frame.decode::<LoginDisconnectS2c>() {
                    info!("Disconnect: {}", disconect_packet.reason);
                } else if let Ok(encryption_request_packet) = frame.decode::<LoginHelloS2c>() {
                    info!(" Encryption Request: {:?}", encryption_request_packet);
                } else if let Ok(set_compression_packet) = frame.decode::<LoginCompressionS2c>() {
                    info!(" Set Compression: {:?}", set_compression_packet);
                    io.set_compression(set_compression_packet.threshold.0.into());
                } else if let Ok(login_success_packet) = frame.decode::<LoginSuccessS2c>() {
                    info!(" Login Success: {:?}", login_success_packet);
                    break;
                } else {
                    tracing::error!("Unexpected packet: {:?}", frame);
                }
            }
            Err(e) => {
                tracing::error!("Failed to receive packet: {:?}", e);
            }
        }
    }

    println!("Handshake complete");

    // loop on recv_frame and log the packets
    // tokio::spawn(async move {
    //     loop {
    //         match io.recv_frame().await {
    //             Ok(frame) => {
    //                 tracing::info!("Received packet: {:?}", frame);
    //             }
    //             Err(e) => {
    //                 tracing::error!("Failed to receive packet: {:?}", e);
    //             }
    //         }
    //     }
    // });

    Ok(())
}
