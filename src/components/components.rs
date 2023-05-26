use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    pub speed: f32,
    pub rotation_speed: f32,
}

#[derive(Component)]
pub struct ContinuousImpulse;