use bevy::app::CoreSet::PreUpdate;
use bevy::prelude::*;
use crate::components::components::EntityInfo;

pub struct EntityInfoPlugin;

impl Plugin for EntityInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                update_system.in_base_set(PreUpdate),
            ),
        );
    }
}

fn update_system(
    mut entities: Query<(&mut EntityInfo, &Transform)>,
) {
    for (mut entity_info, transform) in entities.iter_mut() {
        entity_info.last_position = transform.translation;
    }
}
