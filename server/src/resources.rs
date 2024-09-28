use std::collections::HashMap;

use bevy_ecs::{entity::Entity, prelude::Resource};

use naia_bevy_server::{RoomKey, UserKey};

#[derive(Resource)]
pub struct GlobalServerState {
    pub main_room_key: RoomKey,
    pub user_to_entity: HashMap<UserKey, Entity>,
    pub entity_to_user: HashMap<Entity, UserKey>,
    pub request_index: u8,
}
