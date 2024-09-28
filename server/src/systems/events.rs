use bevy_ecs::prelude::*;
use naia_bevy_server::events::AuthEvents;
use naia_bevy_server::Server;
use shared::messages::Auth;

pub fn auth_events(mut server: Server, mut event_reader: EventReader<AuthEvents>) {
    for events in event_reader.read() {
        for (user_key, auth) in events.read::<Auth>() {
            println!("Incoming auth request for username {}", &auth.username);
            if auth.username == "charlie" && auth.password == "12345" {
                // Accept incoming connection
                server.accept_connection(&user_key);
            } else {
                // Reject incoming connection
                server.reject_connection(&user_key);
            }
        }
    }
}
