use bevy::prelude::*;

use crate::plugins::loading::BackgroundHandles;
use crate::state::GameState;

#[derive(Component)]
pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn setup(mut commands: Commands, bg_textures: Res<BackgroundHandles>) {
    commands.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, -1.0),
            ..default()
        },
        texture: bg_textures.by_key("bg1"),
        ..default()
    });
}