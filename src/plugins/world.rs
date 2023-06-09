use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::{CollisionEvent, ContactForceEvent, RapierContext};
use crate::components::components::{Destroyable, DestroyLeaveScreen, Player, Projectile};
use crate::state::GameState;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                handle_destroyable,
                destroy_leave_screen,
            ).in_set(OnUpdate(GameState::Playing)),
        );
    }
}

fn handle_destroyable(
    rapier_context: Res<RapierContext>,
    query_projectile: Query<Entity, With<Projectile>>,
    query_destroyable: Query<Entity, With<Destroyable>>,
    mut commands: Commands
) {
    for entity_projectile in query_projectile.iter() {
        for entity_destroyable in query_destroyable.iter() {
            if rapier_context.intersection_pair(entity_projectile, entity_destroyable) == Some(true) {
                commands.entity(entity_destroyable).despawn();
                commands.entity(entity_projectile).despawn();
            }
        }
    }
}

fn destroy_leave_screen(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform), With<DestroyLeaveScreen>>,
    player_query: Query<&Transform, (With<Player>, Without<DestroyLeaveScreen>)>,
    window: Query<&Window, With<PrimaryWindow>>
) {
    let window = window.get_single().unwrap();
    let player_transform = player_query.get_single();

    if let Ok(player_transform) = player_transform {
        let destroy_size = Vec2 {
            x: window.width() * 2.,
            y: window.height() * 2.
        };

        for (entity, transform) in query.iter_mut() {
            // destroy entity if it goes off screen
            let entity_normalized_pos = Vec2 {
                x: f32::abs(transform.translation.x - player_transform.translation.x),
                y: f32::abs(transform.translation.y - player_transform.translation.y),
            };
            if entity_normalized_pos.x < 0. || entity_normalized_pos.x > destroy_size.x || entity_normalized_pos.y < 0. || entity_normalized_pos.y > destroy_size.y {
                commands.entity(entity).despawn();
            }
        }
    }
}

