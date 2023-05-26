use bevy::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

use plugins::{
    camera::*,
    infinite_background::*,
    loading::*,
    player::*,
};
use state::*;

mod state;
mod plugins;
mod components;

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "GameSpace".into(),
                        resolution: (700., 525.).into(),
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(InfiniteBackgroundPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}