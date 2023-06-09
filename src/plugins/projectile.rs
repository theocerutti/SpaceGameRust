use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, _app: &mut App) {
    }
}
