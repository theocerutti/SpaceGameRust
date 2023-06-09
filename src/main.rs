use bevy::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, RapierDebugRenderPlugin, RapierPhysicsPlugin};

use plugins::{
    camera::*,
    parallax_background::*,
    loading::*,
    player::*,
    entity_info::*,
    world::*,
};
use state::*;

mod state;
mod plugins;
mod components;

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        title: "GameSpace".into(),
                        resolution: (1024., 896.).into(),
                        ..default()
                    }),
                    ..default()
                }
            )
            .set(ImagePlugin::default_nearest())
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.))
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldPlugin)
        .add_plugin(EntityInfoPlugin)
        .add_plugin(ParallaxBackgroundPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
