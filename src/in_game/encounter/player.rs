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
    actions::{ActionChoice, PlayerActionBundle},
    dice_pools::{DicePool, DicePoolType, DiceType, InitialPools},
    sequencing::{EncounterState, PublishAvailableActions},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_type::<PlayerReference>()
            .register_type::<Player>()
            .register_type::<Players>()
            .add_plugins(YamlAssetPlugin::<Players>::new(&["pl.yaml"]))
            .add_systems(
                OnEnter(EncounterState::ActionChoice),
                say_hello_action.in_set(PublishAvailableActions),
            );
    }
}

#[derive(Reflect, InspectorOptions, Deserialize, Clone, Debug)]
pub struct PlayerReference {
    pub name: String,
    pub scene: MaterializedSceneReference,
}

#[derive(Reflect, InspectorOptions, Component)]
pub struct Player {
    pub name: String,
}

#[derive(Reflect, InspectorOptions, Deserialize, TypeUuid)]
#[uuid = "4c70a2d8-8e22-4a7a-9bee-289fb6d417e8"]
pub struct Players(HashMap<String, PlayerReference>);

impl Players {
    pub fn get(&self, key: &str) -> Option<&PlayerReference> {
        self.0.get(key)
    }
}

fn say_hello_action(mut commands: Commands, players: Query<(Entity, &Player)>) {
    for (entity, player) in players.iter() {
        commands.entity(entity).with_children(|p| {
            p.spawn(PlayerActionBundle {
                action_choice: ActionChoice {
                    title: "Hello!".to_string(),
                    content: format!("Say Hi as {}", player.name),
                    fail: 4,
                    success: 10,
                    critical_success: 15,
                    dice_pool: InitialPools::new(vec![
                        DicePool {
                            dice: DiceType::D8,
                            pool: DicePoolType::Single,
                        },
                        DicePool {
                            dice: DiceType::D8,
                            pool: DicePoolType::Single,
                        },
                        DicePool {
                            dice: DiceType::Static { value: 2 },
                            pool: DicePoolType::Single,
                        },
                    ]),
                },
            });
            p.spawn(PlayerActionBundle {
                action_choice: ActionChoice {
                    title: "Goodbye!".to_string(),
                    content: format!("Say Bye as {}", player.name),
                    fail: 5,
                    success: 8,
                    critical_success: 10,
                    dice_pool: InitialPools::new(vec![
                        DicePool {
                            dice: DiceType::D6,
                            pool: DicePoolType::Advantage,
                        },
                        DicePool {
                            dice: DiceType::D8,
                            pool: DicePoolType::Single,
                        },
                    ]),
                },
            });
        });
    }
}
