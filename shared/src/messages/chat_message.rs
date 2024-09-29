use naia_bevy_shared::Message;

#[derive(Message)]
pub struct ChatMessage {
    pub message: String,
}

impl ChatMessage {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
