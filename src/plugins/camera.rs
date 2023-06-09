use bevy::prelude::*;

use crate::components::components::EntityInfo;
use crate::plugins::player::Player;
use crate::state::GameState;

#[derive(Component)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
        app.add_systems(
            (
                movement_system,
            ),
        );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        EntityInfo {
            last_position: Vec3::ZERO
        }
    ));
}

fn movement_system(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let camera_transform = camera_query.get_single_mut();
    let player_transform = player_query.get_single();

    if let Ok(player_transform) = player_transform {
        if let Ok(mut camera_transform) = camera_transform {
            camera_transform.translation.x = player_transform.translation.x;
            camera_transform.translation.y = player_transform.translation.y;
        }
    }
}
