mod action_choice;
mod actions;
mod challenger;
mod dice_pools;
mod encounter_assets;
mod health;
mod introduction;
mod location;
mod player;
mod powers;
mod probability_setup;
pub mod sequencing;

use bevy::{
    gltf::{Gltf, GltfNode},
    input::common_conditions::input_toggle_active,
    prelude::*,
};
use bevy_asset_loader::prelude::DynamicAssets;
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::StateInspectorPlugin, InspectorOptions,
};

use crate::{
    assets::MainGameAssets,
    in_game::encounter::{
        challenger::Challenger,
        health::{CurrentHealth, MaxHealth},
        player::Player,
        powers::Power,
    },
    materialized_scene::MaterializedSceneBundle,
};

use self::{
    action_choice::ActionChoicePlugin,
    challenger::{ChallengerPlugin, ChallengerReference},
    encounter_assets::{
        setup_encounter_assets, EncounterAssetPlugin, EncounterAssets, Materials, SceneBundler,
    },
    health::HealthPlugin,
    introduction::IntroductionPlugin,
    location::{LocationPlugin, LocationReference},
    player::{PlayerPlugin, PlayerReference},
    probability_setup::ProbabilitySetupPlugin,
    sequencing::EncounterState,
};

use super::{factions::Faction, game_state::GameState, InGameUpdate};

pub use self::challenger::Challengers;
pub use self::location::Locations;
pub use self::player::Players;

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<EncounterState>()
            .register_type::<EncounterState>()
            .register_type::<EncounterSetup>()
            .add_plugins(
                StateInspectorPlugin::<EncounterState>::default()
                    .run_if(input_toggle_active(false, KeyCode::F1)),
            )
            .add_plugins((
                IntroductionPlugin,
                EncounterAssetPlugin,
                LocationPlugin,
                ChallengerPlugin,
                PlayerPlugin,
                ActionChoicePlugin,
                ProbabilitySetupPlugin,
                HealthPlugin,
            ))
            .add_systems(
                OnEnter(GameState::Encounter),
                generate_encounter.run_if(not(resource_exists::<EncounterSetup>())),
            )
            .add_systems(OnEnter(EncounterState::Introduction), spawn_encounter)
            .add_systems(OnEnter(EncounterState::None), despawn_encounter)
            .add_systems(
                InGameUpdate,
                start_encounter.run_if(
                    in_state(EncounterState::None)
                        .and_then(resource_exists::<EncounterSetup>())
                        .and_then(resource_changed::<EncounterSetup>()),
                ),
            );
    }
}

#[derive(Component)]
pub struct EncounterEntity;

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct EncounterSetup {
    pub title: Option<String>,
    pub introduction: Option<String>,
    pub player_faction: Faction,
    pub player: Option<PlayerReference>,
    pub challengers: Vec<(usize, ChallengerReference)>,
    pub location: Option<LocationReference>,
}

impl Default for EncounterSetup {
    fn default() -> Self {
        Self {
            title: Some("An Encounter".to_string()),
            introduction: Some("Let me introduce myself...".to_string()),
            player_faction: Faction::Knights,
            challengers: vec![],
            location: None,
            player: None,
        }
    }
}

fn generate_encounter(
    mut commands: Commands,
    assets: Res<MainGameAssets>,
    locations: Res<Assets<Locations>>,
    challengers: Res<Assets<Challengers>>,
    players: Res<Assets<Players>>,
) {
    let (Some(locations), Some(challengers), Some(players)) = (
        locations.get(&assets.locations),
        challengers.get(&assets.challengers),
        players.get(&assets.players),
    ) else {
        return;
    };
    commands.insert_resource(EncounterSetup {
        location: locations.get("sand").cloned(),
        player: players.get("player_knight").cloned(),
        challengers: match challengers.get("monster") {
            Some(c) => vec![(3, c.clone())],
            None => vec![],
        },
        ..Default::default()
    });
    info!("Generating Encounter");
}

fn start_encounter(
    mut commands: Commands,
    setup: Res<EncounterSetup>,
    mut dynamic_assets: ResMut<DynamicAssets>,
) {
    setup_encounter_assets(setup.as_ref(), dynamic_assets.as_mut());
    commands.insert_resource(NextState(Some(EncounterState::Loading)));
}

fn spawn_encounter(
    mut commands: Commands,
    setup: Res<EncounterSetup>,
    assets: Res<EncounterAssets>,
    materials: Res<Materials>,
    gltf: Res<Assets<Gltf>>,
    gltf_node: Res<Assets<GltfNode>>,
    camera: Query<Entity, With<Camera3d>>,
) {
    let bundler = SceneBundler::new(&assets, &materials, &gltf, &gltf_node);
    if let (Some(location), Some(player)) = (&setup.location, &setup.player) {
        info!("Spawning Location {location:?}");
        if let Some(bundle) = bundler.scene(&location.scene) {
            commands.spawn((bundle, EncounterEntity));
        } else {
            error!("Couldn't setup bundle");
        }
        if let Some(transform) = bundler.camera_position(&location.scene) {
            for camera in camera.iter() {
                commands.entity(camera).insert(transform);
            }
        }

        if let (Some(transform), Some(bundle)) = (
            bundler.player_position(&location.scene),
            bundler.scene(&player.scene),
        ) {
            let bundle = MaterializedSceneBundle {
                transform: TransformBundle {
                    local: transform,
                    global: GlobalTransform::default(),
                },
                ..bundle.clone()
            };
            commands
                .spawn((
                    Player {
                        name: player.name.clone(),
                    },
                    bundle,
                    EncounterEntity,
                    CurrentHealth(5),
                    MaxHealth(7),
                ))
                .with_children(|p| {
                    p.spawn(Power::SplitDice);
                    p.spawn(Power::Advantage);
                    p.spawn(Power::StaticBonus(2));
                    p.spawn(Power::AddDice(dice_pools::DiceType::D3));
                });
        }

        let mut challenger_id = 0usize;
        let challenger_slots = location.challenger_slots;

        for (count, challenger) in setup.challengers.iter() {
            if challenger_id >= challenger_slots {
                break;
            }
            if let Some(bundle) = bundler.scene(&challenger.scene) {
                for _ in 0..*count {
                    if challenger_id >= challenger_slots {
                        break;
                    }
                    let Some(transform) =
                        bundler.challenger_position(&location.scene, challenger_id)
                    else {
                        break;
                    };

                    let bundle = MaterializedSceneBundle {
                        transform: TransformBundle {
                            local: transform,
                            global: GlobalTransform::default(),
                        },
                        ..bundle.clone()
                    };
                    commands.spawn((
                        Challenger {
                            id: challenger_id,
                            name: challenger.name.clone(),
                        },
                        CurrentHealth(2),
                        MaxHealth(5),
                        bundle,
                        EncounterEntity,
                    ));
                    challenger_id += 1;
                }
            }
        }
    }
}

fn despawn_encounter(mut commands: Commands, query: Query<Entity, With<EncounterEntity>>) {
    commands.remove_resource::<EncounterSetup>();
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
