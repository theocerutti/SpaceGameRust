use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::player::{create_player, Player};
use crate::entities::ship::Ship;
use crate::plugins::loading::ShipHandles;
use crate::state::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
        app.add_systems(
            (
                ship_input_system,
                ship_dampening_system,
            ).in_set(OnUpdate(GameState::Playing)),
        );
    }
}

fn setup(commands: Commands, ship_textures: Res<ShipHandles>, image_assets: Res<Assets<Image>>) {
    create_player(commands, ship_textures.by_key("blue1"), image_assets);
}

fn ship_dampening_system(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        let elapsed = time.delta_seconds();
        velocity.angvel *= 0.1f32.powf(elapsed);
        velocity.linvel *= 0.4f32.powf(elapsed);
    }
}

fn ship_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut ExternalImpulse,
        &mut Velocity,
        &Transform,
        &Ship,
    )>,
) {
    for (mut impulse, mut velocity, transform, ship) in query.iter_mut() {
        let rotation = if keyboard_input.pressed(KeyCode::Q) || keyboard_input.pressed(KeyCode::Left) {
            1
        } else if keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right) {
            -1
        } else {
            0
        };
        if rotation != 0 {
            velocity.angvel = rotation as f32 * ship.rotation_speed;
        }
        impulse.impulse = (transform.rotation * (Vec3::Y * ship.speed)).truncate();
    }
}