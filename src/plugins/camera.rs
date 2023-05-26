use bevy::prelude::*;

use crate::entities::player::Player;
use crate::state::GameState;

#[derive(Component)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
        app.add_systems(
            (
                camera_movement_system,
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_movement_system(
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut players: Query<&Transform, (With<Player>, Without<Camera>)>
) {
    for mut transform in cameras.iter_mut() {
        for transform_player in players.iter_mut() {
            transform.translation = transform_player.translation;
        }
    }
}