use bevy::{
    prelude::*,
    reflect::{Reflect, TypeUuid},
    utils::HashMap,
};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::InspectorOptions;
use serde::Deserialize;

use crate::materialized_scene::{MaterializedScene, MaterializedSceneReference};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(YamlAssetPlugin::<Players>::new(&["pl.yaml"]));
    }
}

#[derive(Reflect, InspectorOptions, Deserialize, Clone, Debug)]
pub struct PlayerReference {
    pub name: String,
    pub scene: MaterializedSceneReference,
}

#[derive(Reflect, InspectorOptions)]
pub struct Player {
    pub name: String,
    pub scene: MaterializedScene,
}

#[derive(Reflect, InspectorOptions, Deserialize, TypeUuid)]
#[uuid = "4c70a2d8-8e22-4a7a-9bee-289fb6d417e8"]
pub struct Players(HashMap<String, PlayerReference>);

impl Players {
    pub fn get(&self, key: &str) -> Option<&PlayerReference> {
        self.0.get(key)
    }
}
