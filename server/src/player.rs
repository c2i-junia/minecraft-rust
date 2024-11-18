use crate::init::TickCounter;
use bevy::prelude::*;
use bevy_ecs::prelude::Res;
use shared::messages::PlayerInputs;

pub fn handle_player_inputs(player_inputs: PlayerInputs, ticker: &Res<TickCounter>) {
    if ticker.tick % 60 == 0 {
        trace!("Received inputs: {:?}", player_inputs);
    }
}
