use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::state::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                destroy_leave_screen,
            ).in_set(OnUpdate(GameState::Playing)),
        );
    }
}

fn destroy_leave_screen(mut commands: Commands, mut query: Query<(Entity, &Transform), With<Projectile>>, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();

    for (entity, transform) in query.iter_mut() {
        // destroy entity if it goes off screen
        if transform.translation.x < -window.width() / 2. || transform.translation.x > window.width() / 2. || transform.translation.y < -window.height() / 2. || transform.translation.y > window.height() / 2. {
            commands.entity(entity).despawn();
        }
    }
}
