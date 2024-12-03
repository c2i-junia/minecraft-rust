use crate::init::ServerTime;
use crate::init::TickCounter;
use bevy::prelude::*;

pub fn update_server_time(mut time: ResMut<ServerTime>, tick_counter: Res<TickCounter>) {
    if tick_counter.tick % 60 == 0 {
        time.0 += 1;
        debug!("Server time updated: {}", time.0);
    }
}
