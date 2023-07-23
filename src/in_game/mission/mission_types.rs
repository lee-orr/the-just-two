use bevy::{prelude::*, reflect::TypeUuid, utils::HashMap};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use serde::Deserialize;

use crate::in_game::encounter::encounter_setup_types::EncounterInitialDetails;

pub struct MissionAssetsPlugin;

impl Plugin for MissionAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Mission>()
            .register_type::<Missions>()
            .register_type::<MissionStage>()
            .add_plugins(YamlAssetPlugin::<Missions>::new(&["ms.yaml"]));
    }
}

#[derive(Resource, Default, Reflect, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct Mission {
    pub title: String,
    pub encounters: Vec<Vec<EncounterInitialDetails>>,
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MissionStage(pub usize);

#[derive(Default, Reflect, InspectorOptions, Deserialize)]
#[reflect(InspectorOptions)]
pub struct MissionGenerationInfo {
    pub titles: Vec<String>,
    pub encounters: Vec<Vec<String>>,
}

#[derive(Resource, Default, Reflect, InspectorOptions, Deserialize, TypeUuid)]
#[reflect(Resource, InspectorOptions)]
#[uuid = "2cc8fe4a-f06d-4aff-b863-ae1a5b743acd"]
pub struct Missions(HashMap<String, MissionGenerationInfo>);
