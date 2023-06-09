use bevy::app::CoreSet::PostUpdate;
use bevy::prelude::*;

use crate::components::components::EntityInfo;
use crate::plugins::loading::BackgroundHandles;
use crate::state::GameState;

#[derive(Component)]
pub struct ParallaxBackgroundLayer {
    pub speed: f32,
    pub handle_key: String,
    pub order_index: i32,
}

pub struct ParallaxBackgroundPlugin;

impl Plugin for ParallaxBackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Playing)));
        app.add_system(update_map_system.run_if(in_state(GameState::Playing)).in_base_set(PostUpdate)); // TODO: why post update here.
    }
}

fn update_map_system(
    camera_query: Query<(&EntityInfo, &Transform), (With<Camera>, Without<ParallaxBackgroundLayer>)>,
    mut layers: Query<(&mut Sprite, &ParallaxBackgroundLayer, &mut Transform), Without<Camera>>,
    image_assets: Res<Assets<Image>>,
    bg_textures: Res<BackgroundHandles>,
) {
    let camera = camera_query.get_single();

    if let Ok((camera_entity_info, camera_transform)) = camera {
        for (mut sprite, layer, mut transform) in layers.iter_mut() {
            let handle = bg_textures.by_key(layer.handle_key.as_str());
            let image_asset = image_assets.get(&handle).unwrap();
            let size_asset = Vec2 {
                x: image_asset.texture_descriptor.size.width as f32,
                y: image_asset.texture_descriptor.size.height as f32,
            };
            let mut background_rect = sprite.rect.unwrap();
            let moved = (camera_entity_info.last_position - camera_transform.translation) * layer.speed;

            for axis in 0..2 {
                let is_y = axis == 1;
                if is_y {
                    background_rect.min[axis] += moved[axis];
                    background_rect.max[axis] += moved[axis];
                } else {
                    background_rect.min[axis] -= moved[axis];
                    background_rect.max[axis] -= moved[axis];
                }
                if background_rect.min[axis] > size_asset[axis] / 2. {
                    background_rect.min[axis] = 0.;
                    background_rect.max[axis] = size_asset[axis] / 2.;
                }
                if background_rect.min[axis] < 0. {
                    background_rect.min[axis] = size_asset[axis] / 2.;
                    background_rect.max[axis] = size_asset[axis];
                }
            }
            sprite.rect = Option::from(background_rect);
            transform.translation = camera_transform.translation; // TODO: bad: have to always move the background to the camera position.
            transform.translation.z = layer.order_index as f32;
        }
    }
}

fn setup(mut commands: Commands, bg_textures: Res<BackgroundHandles>, image_assets: Res<Assets<Image>>) {
    commands.spawn(create_layer("bg1", 4., 0.6, 4, &bg_textures, &image_assets));
    commands.spawn(create_layer("bg2", 2., 0.4, 3, &bg_textures, &image_assets));
    commands.spawn(create_layer("bg3", 1., 0.2, 2, &bg_textures, &image_assets));
    commands.spawn(create_layer("bg4", 0.5, 0.1, 1, &bg_textures, &image_assets));
}

fn create_layer(handle_key: &str, speed: f32, opacity: f32, order_index: i32, bg_textures: &Res<BackgroundHandles>, image_assets: &Res<Assets<Image>>) -> (SpriteBundle, ParallaxBackgroundLayer) {
    let handle = bg_textures.by_key(handle_key);
    let image_asset = image_assets.get(&handle).unwrap();
    let size_asset = Vec2 {
        x: image_asset.texture_descriptor.size.width as f32,
        y: image_asset.texture_descriptor.size.height as f32,
    };
    (
        SpriteBundle {
            texture: handle,
            transform: Transform::from_translation(Vec3::new(0., 0., -order_index as f32)),
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, opacity),
                rect: Option::from(Rect {
                    min: Vec2::new(0., 0.),
                    max: Vec2::new(size_asset.x / 2., size_asset.y / 2.),
                }),
                ..default()
            },
            ..default()
        },
        ParallaxBackgroundLayer {
            speed,
            handle_key: handle_key.parse().unwrap(),
            order_index,
        },
    )
}
