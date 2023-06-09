use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use rand::prelude::*;
use field_count::FieldCount;

use crate::state::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Load).continue_to_state(GameState::Playing),
        );
        app.add_collection_to_loading_state::<_, ShipHandles>(GameState::Load);
        app.add_collection_to_loading_state::<_, BackgroundHandles>(GameState::Load);
        app.add_collection_to_loading_state::<_, ProjectileHandles>(GameState::Load);
        app.add_collection_to_loading_state::<_, AsteroidHandles>(GameState::Load);
    }
}

#[derive(AssetCollection, Resource)]
pub struct BackgroundHandles {
    #[asset(path = "backgrounds/bg1.png")]
    bg1: Handle<Image>,
    #[asset(path = "backgrounds/bg2.png")]
    bg2: Handle<Image>,
    #[asset(path = "backgrounds/bg3.png")]
    bg3: Handle<Image>,
    #[asset(path = "backgrounds/bg4.png")]
    bg4: Handle<Image>,
}

impl BackgroundHandles {
    pub fn by_key(&self, key: &str) -> Handle<Image> {
        match key {
            "bg1" => self.bg1.clone(),
            "bg2" => self.bg2.clone(),
            "bg3" => self.bg3.clone(),
            "bg4" => self.bg4.clone(),
            _ => panic!("background atlas does not exist"),
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct ShipHandles {
    #[asset(path = "ships/blue_ship1.png")]
    blue1: Handle<Image>,
    #[asset(path = "ships/blue_ship2.png")]
    blue2: Handle<Image>,
    #[asset(path = "ships/blue_ship3.png")]
    blue3: Handle<Image>,
    #[asset(path = "ships/red_ship1.png")]
    red1: Handle<Image>,
    #[asset(path = "ships/red_ship2.png")]
    red2: Handle<Image>,
    #[asset(path = "ships/red_ship3.png")]
    red3: Handle<Image>,
}

impl ShipHandles {
    pub fn by_key(&self, key: &str) -> Handle<Image> {
        match key {
            "blue1" => self.blue1.clone(),
            "blue2" => self.blue2.clone(),
            "blue3" => self.blue3.clone(),
            "red1" => self.red1.clone(),
            "red2" => self.red2.clone(),
            "red3" => self.red3.clone(),
            _ => panic!("ship atlas does not exist"),
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct ProjectileHandles {
    #[asset(path = "projectiles/projectile1.png")]
    projectile1: Handle<Image>,
}

impl ProjectileHandles {
    pub fn by_key(&self, key: &str) -> Handle<Image> {
        match key {
            "projectile1" => self.projectile1.clone(),
            _ => panic!("projectile atlas does not exist"),
        }
    }
}

#[derive(AssetCollection, Resource, FieldCount)]
pub struct AsteroidHandles {
    #[asset(path = "asteroids/asteroid1.png")]
    asteroid1: Handle<Image>,
    #[asset(path = "asteroids/asteroid2.png")]
    asteroid2: Handle<Image>,
    #[asset(path = "asteroids/asteroid3.png")]
    asteroid3: Handle<Image>,
    #[asset(path = "asteroids/asteroid4.png")]
    asteroid4: Handle<Image>,
}

impl AsteroidHandles {
    pub fn by_key(&self, key: &str) -> Handle<Image> {
        match key {
            "asteroid1" => self.asteroid1.clone(),
            "asteroid2" => self.asteroid2.clone(),
            "asteroid3" => self.asteroid3.clone(),
            "asteroid4" => self.asteroid4.clone(),
            _ => panic!("asteroid atlas does not exist"),
        }
    }

    pub fn random(&self) -> Handle<Image> {
        self.by_key(format!("asteroid{}", rand::thread_rng().gen_range(1..AsteroidHandles::field_count())).as_str())
    }
}
