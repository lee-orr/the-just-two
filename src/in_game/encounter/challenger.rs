use bevy::{
    prelude::*,
    reflect::{Reflect, TypeUuid},
    utils::HashMap,
};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::InspectorOptions;
use serde::Deserialize;

use crate::materialized_scene::{MaterializedScene, MaterializedSceneReference};

pub struct ChallengerPlugin;

impl Plugin for ChallengerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(YamlAssetPlugin::<Challengers>::new(&["ch.yaml"]));
    }
}

#[derive(Reflect, InspectorOptions, Deserialize, Clone, Debug)]
pub struct ChallengerReference {
    pub name: String,
    pub scene: MaterializedSceneReference,
}

#[derive(Reflect, InspectorOptions)]
pub struct Challenger {
    pub name: String,
    pub scene: MaterializedScene,
}

#[derive(Reflect, InspectorOptions, Deserialize, TypeUuid)]
#[uuid = "e3cb22e9-0e2b-4af0-be00-c9c3fc18dbc7"]
pub struct Challengers(HashMap<String, ChallengerReference>);

impl Challengers {
    pub fn get(&self, key: &str) -> Option<&ChallengerReference> {
        self.0.get(key)
    }
}
