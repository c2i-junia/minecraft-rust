use crate::init::TickCounter;
use bevy_ecs::prelude::Res;
use shared::messages::PlayerInputs;

pub fn handle_player_inputs(player_inputs: PlayerInputs, ticker: &Res<TickCounter>) {
    if ticker.tick % 60 == 0 {
        println!("Received inputs: {:?}", player_inputs);
    }
}
