use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use crate::components::components::*;
use crate::plugins::loading::{ProjectileHandles, ShipHandles};
use crate::state::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
        app.add_systems(
            (
                move_system,
                shoot_system,
                dampening_system,
            ).in_set(OnUpdate(GameState::Playing)),
        );
    }
}

fn setup(commands: Commands, ship_textures: Res<ShipHandles>, image_assets: Res<Assets<Image>>) {
    create_player(commands, ship_textures.by_key("blue1"), image_assets);
}

fn create_player(mut commands: Commands, handle: Handle<Image>, image_assets: Res<Assets<Image>>) {
    let image_asset = image_assets.get(&handle).unwrap();
    let size_asset = Vec2 {
        x: image_asset.texture_descriptor.size.width as f32,
        y: image_asset.texture_descriptor.size.height as f32,
    };
    commands.spawn(Player)
        .insert(Ship {
            speed: 1.,
            rotation_speed: 1.,
        })
        .insert(SpriteBundle {
            texture: handle,
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.))
        .insert(Damping::default())
        .insert(ExternalImpulse::default())
        .insert(Velocity::linear(Vec2::ZERO))
        .insert(Collider::cuboid(size_asset.x / 2., size_asset.y / 2.))
        .insert(ColliderMassProperties::Density(1.))
        .insert(ColliderMassProperties::Mass(1.))
        .insert(ContinuousImpulse);
}

fn dampening_system(time: Res<Time>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        let elapsed = time.delta_seconds();
        velocity.angvel *= 0.1f32.powf(elapsed);
        velocity.linvel *= 0.4f32.powf(elapsed);
    }
}

fn shoot(cursor_position: Vec2, player_position: Vec3, image_assets: &Res<Assets<Image>>, window: &Window, commands: &mut Commands, projectile_textures: &Res<ProjectileHandles>) {
    let direction = Vec2::new(cursor_position.x - window.width() / 2., cursor_position.y - window.height() / 2.);
    let mut angle_to_target = direction.y.atan2(direction.x) - PI / 2.;
    if angle_to_target < 0. {
        angle_to_target += 2.0 * PI;
    }

    let handle = projectile_textures.by_key("projectile1");
    let image_asset = image_assets.get(&handle).unwrap();
    let size_asset = Vec2 {
        x: image_asset.texture_descriptor.size.width as f32,
        y: image_asset.texture_descriptor.size.height as f32,
    };

    let projectile_speed = 1000.;

    commands.spawn(Projectile)
        .insert(RigidBody::Dynamic)
        .insert(ExternalForce {
            force: direction.normalize() * projectile_speed,
            ..default()
        })
        .insert(DestroyLeaveScreen)
        .insert(Sensor::default())
        .insert(Collider::cuboid(size_asset.x / 2., size_asset.y / 2.))
        .insert(ColliderMassProperties::Density(1.))
        .insert(ColliderMassProperties::Mass(1.))
        .insert(GravityScale(0.))
        .insert(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(player_position.x, player_position.y, 0.),
                rotation: Quat::from_rotation_z(angle_to_target - PI / 2.),
                ..default()
            },
            texture: handle,
            ..default()
        });
}

fn move_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(
        &mut ExternalImpulse,
        &mut Velocity,
        &Transform,
        &mut Ship,
        Option<&ContinuousImpulse>
    ), With<Player>>,
) {
    for (mut impulse, mut velocity, transform, ship, continuous_impulse) in query.iter_mut() {
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
        if let Some(_continuous_impulse) = continuous_impulse {
            impulse.impulse = (transform.rotation * (Vec3::Y * ship.speed)).truncate();
        }
    }
}

fn shoot_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    projectile_textures: Res<ProjectileHandles>,
    image_assets: Res<Assets<Image>>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<&Transform, With<Player>>,
) {
    let window = window_query.get_single().unwrap();

    for transform in query.iter_mut() {
        if let Some(cursor_position) = window.cursor_position() {
            if buttons.just_pressed(MouseButton::Left) {
                shoot(cursor_position, transform.translation, &image_assets, window, &mut commands, &projectile_textures);
            }
        }
    }
}