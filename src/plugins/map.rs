use bevy::prelude::*;
use crate::state::GameState;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                create_map,
            ).in_schedule(OnEnter(GameState::Playing)),
        );
    }
}

fn create_map() {
    // create procedural map

}