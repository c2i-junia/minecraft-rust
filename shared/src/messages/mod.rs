mod auth;
mod chat_message;

use crate::messages::chat_message::ChatMessage;
pub use auth::Auth;
use naia_bevy_shared::{Protocol, ProtocolPlugin};

pub struct MessagesPlugin;

impl ProtocolPlugin for MessagesPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol.add_message::<Auth>().add_message::<ChatMessage>();
    }
}
