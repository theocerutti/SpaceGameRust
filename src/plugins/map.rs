use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use crate::components::components::{Asteroid, DeathCollider, Destroyable};
use crate::plugins::loading::AsteroidHandles;
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

fn spawn_asteroid(image_assets: &Res<Assets<Image>>, commands: &mut Commands, asteroid_textures: &Res<AsteroidHandles>) {
    let random_handle = asteroid_textures.random();
    let image_asset = image_assets.get(&random_handle).unwrap();
    let size_asset = Vec2 {
        x: image_asset.texture_descriptor.size.width as f32,
        y: image_asset.texture_descriptor.size.height as f32,
    };

    let spawn_area = Vec2 {
      x: rand::thread_rng().gen_range(-2000..2000) as f32, // TODO: should be procedurally generated
      y: rand::thread_rng().gen_range(-2000..2000) as f32
    };

    commands.spawn(Asteroid)
        .insert(Destroyable)
        .insert(DeathCollider)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(SpriteBundle {
            texture: random_handle,
            transform: Transform {
                translation: Vec3::new(spawn_area.x, spawn_area.y, 20.),
                scale: Vec3::new(2., 2., 1.),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(size_asset.x / 2., size_asset.y / 2.));
}

fn create_map(image_assets: Res<Assets<Image>>, mut commands: Commands, asteroid_textures: Res<AsteroidHandles>) {
    for _ in 0..100 {
        spawn_asteroid(&image_assets, &mut commands, &asteroid_textures);
    }
}