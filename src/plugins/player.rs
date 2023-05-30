use std::f32::consts::PI;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

use crate::components::components::*;
use crate::plugins::loading::{ProjectileHandles, ShipHandles};
use crate::plugins::projectile::Projectile;
use crate::state::GameState;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
        app.add_systems(
            (
                input_system,
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

fn shoot(cursor_pos: Vec2, position: Vec3, commands: &mut Commands, projectile_textures: &Res<ProjectileHandles>) {
    let rot = Quat::from_rotation_z(f32::atan2(cursor_pos.x - position.x, cursor_pos.y - position.y));
    commands.spawn(Projectile {
        speed: 1.,
        target: cursor_pos,
    })
        .insert(RigidBody::Fixed)
        .insert(Damping::default())
        .insert(ExternalForce::default())
        .insert(Collider::cuboid(10., 10.))
        .insert(ColliderMassProperties::Density(1.))
        .insert(ColliderMassProperties::Mass(1.))
        .insert(SpriteBundle {
            transform: Transform {
                translation: position,
                rotation: rot,
                ..default()
            },
            texture: projectile_textures.by_key("projectile1"),
            ..default()
        });
}

fn input_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    projectile_textures: Res<ProjectileHandles>,
    keyboard_input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut query: Query<(
        &mut ExternalImpulse,
        &mut Velocity,
        &mut Transform,
        &Ship,
        Option<&ContinuousImpulse>
    )>,
) {
    let window = window_query.get_single().unwrap();

    for (mut impulse, mut velocity, mut transform, ship, continuous_impulse) in query.iter_mut() {
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
        velocity.angvel = 0.;
        velocity.linvel = Vec2::ZERO;
        if let Some(_continuous_impulse) = continuous_impulse {
            impulse.impulse = (transform.rotation * (Vec3::Y * ship.speed)).truncate();
        }

        if let Some(cursor_position) = window.cursor_position() {
            let ww = transform.translation.x + window.width() / 2. as f32;
            let wh = transform.translation.y + window.height() / 2. as f32;
            let delta_x = ww - cursor_position.x;
            let delta_y = wh - cursor_position.y;
            let rad = f32::atan2(delta_y, delta_x);
            let mut deg = rad * (180. / PI);
            if deg < 0. {
                deg = (deg + 360.) % 360.;
            }
            transform.rotation = Quat::from_rotation_z((deg + 90.).to_radians());

            if buttons.just_pressed(MouseButton::Left) {
                // shoot(cursor_position, transform.translation - Vec3::new(window.width(), window.height(), 0.0), &commands, &projectile_textures); // TODO: weird to pass textures by params here

                let position = transform.translation - Vec3::new(window.width(), window.height(), 0.0);
                let rot = Quat::from_rotation_z(f32::atan2(cursor_position.x - position.x, cursor_position.y - position.y));
                commands.spawn(Projectile {
                    speed: 1.,
                    target: cursor_position,
                })
                    .insert(RigidBody::Fixed)
                    .insert(Damping::default())
                    .insert(ExternalForce::default())
                    .insert(Collider::cuboid(10., 10.))
                    .insert(ColliderMassProperties::Density(1.))
                    .insert(ColliderMassProperties::Mass(1.))
                    .insert(SpriteBundle {
                        transform: Transform {
                            translation: position,
                            rotation: rot,
                            ..default()
                        },
                        texture: projectile_textures.by_key("projectile1"),
                        ..default()
                    });
            }
        }
    }

}
