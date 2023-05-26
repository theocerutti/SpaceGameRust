use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::plugins::loading::BackgroundHandles;
use crate::plugins::player::Player;
use crate::state::GameState;

#[derive(Component)]
pub struct InfiniteBackground;

pub struct InfiniteBackgroundPlugin;

impl Plugin for InfiniteBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
        app.add_system(update_map_system);
    }
}

// TODO: should follow camera instead of player
fn update_map_system(
    player_query: Query<&ExternalImpulse, With<Player>>,
    mut backgrounds: Query<&mut Sprite, With<InfiniteBackground>>,
) {
    let player_impulse = player_query.get_single();

    match player_impulse {
        Ok(player_impulse) => {
            let size_asset = Vec2 {
                x: 1400.,
                y: 1050.,
            };
            for mut background in backgrounds.iter_mut() {
                let mut background_rect = background.rect.unwrap();
                background_rect.min.x += player_impulse.impulse.x;
                background_rect.max.x += player_impulse.impulse.x;
                if background_rect.min.x > size_asset.x / 2. {
                    background_rect.min.x = 0.;
                    background_rect.max.x = size_asset.x / 2.;
                }
                background_rect.min.y += player_impulse.impulse.y;
                background_rect.max.y += player_impulse.impulse.y;
                if background_rect.min.y > size_asset.y / 2. {
                    background_rect.min.y = 0.;
                    background_rect.max.y = size_asset.y / 2.;
                }
                background.rect = Option::from(background_rect);
            }
        }
        Err(_) => {
            return;
        },
    }
}

fn setup(mut commands: Commands, bg_textures: Res<BackgroundHandles>, image_assets: Res<Assets<Image>>) {
    let handle = bg_textures.by_key("bg1");
    let image_asset = image_assets.get(&handle).unwrap();
    let size_asset = Vec2 {
        x: image_asset.texture_descriptor.size.width as f32,
        y: image_asset.texture_descriptor.size.height as f32,
    };
    commands.spawn((
        InfiniteBackground,
        SpriteBundle {
            sprite: Sprite {
                rect: Option::from(Rect {
                    min: Vec2::new(0., 0.),
                    max: Vec2::new(size_asset.x / 2., size_asset.y / 2.)
                }),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., -1.),
                ..default()
            },
            texture: bg_textures.by_key("bg1"),
            ..default()
        }
    ));
}