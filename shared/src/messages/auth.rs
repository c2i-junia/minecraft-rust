use naia_bevy_shared::Message;

#[derive(Message)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

impl Auth {
    pub fn new(username: &str, password: &str) -> Self {
        println!("instance auth {}", username);
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}
