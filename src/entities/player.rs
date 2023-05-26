use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::entities::ship::Ship;

#[derive(Component)]
pub struct Player;

pub fn create_player<'a>(mut commands: Commands, handle: Handle<Image>, image_assets: Res<Assets<Image>>) {
    let image_asset = image_assets.get(&handle).unwrap();
    let size_asset = Vec2 {
        x: image_asset.texture_descriptor.size.width as f32,
        y: image_asset.texture_descriptor.size.height as f32,
    };
    commands.spawn(Player)
        .insert(Ship {
            speed: 1.0,
            rotation_speed: 1.0,
        })
        .insert(SpriteBundle {
            transform: Transform {
                scale: Vec3::new(1.0, 1.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                ..default()
            },
            texture: handle,
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(0.0))
        .insert(Damping::default())
        .insert(ExternalImpulse::default())
        .insert(Velocity::linear(Vec2::ZERO))
        .insert(Collider::cuboid(size_asset.x / 2.0, size_asset.y / 2.0))
        .insert(ColliderMassProperties::Density(1.0))
        .insert(ColliderMassProperties::Mass(1.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
}