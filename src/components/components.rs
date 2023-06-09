use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    pub speed: f32,
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct ContinuousImpulse;

#[derive(Component)]
pub struct EntityInfo {
    pub last_position: Vec3,
}

#[derive(Component)]
pub struct DestroyLeaveScreen;

#[derive(Component)]
pub struct ParallaxBackgroundLayer {
    pub speed: f32,
    pub handle_key: String,
    pub order_index: i32,
}

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct DamageableCollider;

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct Destroyable;

#[derive(Component)]
pub struct DeathCollider;