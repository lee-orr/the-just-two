use bevy::{
    prelude::*,
    reflect::{Reflect, TypeUuid},
    utils::HashMap,
};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::InspectorOptions;
use serde::Deserialize;

use crate::materialized_scene::MaterializedSceneReference;

use super::{
    actions::{ActionChoice, ChallengerActionBundle},
    sequencing::{EncounterState, PublishAvailableActions},
};

pub struct ChallengerPlugin;

impl Plugin for ChallengerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<ChallengerReference>()
            .register_type::<Challenger>()
            .register_type::<Challengers>()
            .add_plugins(YamlAssetPlugin::<Challengers>::new(&["ch.yaml"]))
            .add_systems(
                OnEnter(EncounterState::ActionChoice),
                say_challenge_action.in_set(PublishAvailableActions),
            );
    }
}

#[derive(Reflect, InspectorOptions, Deserialize, Clone, Debug)]
pub struct ChallengerReference {
    pub name: String,
    pub scene: MaterializedSceneReference,
}

#[derive(Reflect, InspectorOptions, Component)]
pub struct Challenger {
    pub id: usize,
    pub name: String,
}

#[derive(Reflect, InspectorOptions, Deserialize, TypeUuid)]
#[uuid = "e3cb22e9-0e2b-4af0-be00-c9c3fc18dbc7"]
pub struct Challengers(HashMap<String, ChallengerReference>);

impl Challengers {
    pub fn get(&self, key: &str) -> Option<&ChallengerReference> {
        self.0.get(key)
    }
}

fn say_challenge_action(mut commands: Commands, challengers: Query<(Entity, &Challenger)>) {
    for (entity, challenger) in challengers.iter() {
        commands.entity(entity).with_children(|p| {
            p.spawn(ChallengerActionBundle {
                action_choice: ActionChoice {
                    title: "A CHALLENGE!".to_string(),
                    content: format!(
                        "I, {} ({}), Challenge you to a game of fiddlesticks!",
                        challenger.name, challenger.id
                    ),
                    fail: 4,
                    success: 10,
                    critical_success: 15,
                    ..Default::default()
                },
                ..default()
            });
        });
    }
}
