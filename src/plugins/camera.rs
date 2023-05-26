use bevy::prelude::*;

use crate::plugins::infinite_background::InfiniteBackground;
use crate::plugins::player::Player;
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
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>, Without<InfiniteBackground>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>, Without<InfiniteBackground>)>,
    mut background_query: Query<&mut Transform, (With<InfiniteBackground>, Without<Camera>, Without<Player>)>,
) {
    let mut camera_transform = camera_query.get_single_mut();
    let mut background_transform = background_query.get_single_mut();
    let player_transform = player_query.get_single();

    if let Ok(player_transform) = player_transform {
        if let Ok(mut camera_transform) = camera_transform {
            camera_transform.translation = player_transform.translation;
        }
        if let Ok(mut background_transform) = background_transform {
            background_transform.translation = player_transform.translation;
            background_transform.translation.z = -1.;
        }
    }
}