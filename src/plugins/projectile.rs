use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::state::GameState;

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub target: Vec2,
}

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                destroy_projectile_system,
            ).in_set(OnUpdate(GameState::Playing)),
        );
    }
}

// TODO: maybe create a new Component DestroyLeaveScreen and a new plugin for that?
fn destroy_projectile_system(mut commands: Commands, mut query: Query<(Entity, &Transform), With<Projectile>>, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();

    for (entity, transform) in query.iter_mut() {
        // destroy entity if it goes off screen
        if transform.translation.x < -window.width() / 2. || transform.translation.x > window.width() / 2. || transform.translation.y < -window.height() / 2. || transform.translation.y > window.height() / 2. {
            commands.entity(entity).despawn();
        }
    }
}
