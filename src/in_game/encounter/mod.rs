mod challenger;
mod encounter_assets;
mod introduction;
mod location;

use bevy::{gltf::Gltf, input::common_conditions::input_toggle_active, prelude::*};
use bevy_asset_loader::prelude::DynamicAssets;
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::StateInspectorPlugin, InspectorOptions,
};

use crate::assets::MainGameAssets;

use self::{
    challenger::{ChallengerPlugin, ChallengerReference},
    encounter_assets::{
        setup_encounter_assets, EncounterAssetPlugin, EncounterAssets, Materials, SceneBundler,
    },
    introduction::IntroductionPlugin,
    location::{LocationPlugin, LocationReference},
};

use super::{factions::Faction, game_state::GameState};

pub use self::challenger::Challengers;
pub use self::location::Locations;

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
            ))
            .add_systems(
                OnEnter(GameState::Encounter),
                generate_encounter.run_if(not(resource_exists::<EncounterSetup>())),
            )
            .add_systems(OnEnter(EncounterState::Introduction), spawn_encounter)
            .add_systems(OnEnter(EncounterState::None), despawn_encounter)
            .add_systems(
                Update,
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

#[derive(Clone, Eq, PartialEq, Copy, Debug, Hash, Default, States, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum EncounterState {
    #[default]
    None,
    Loading,
    Introduction,
    ActionChoice,
    ProbabilitySetup,
    OutcomeResolution,
    EncounterResolved,
}

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct EncounterSetup {
    pub title: Option<String>,
    pub introduction: Option<String>,
    pub player_faction: Faction,
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
        }
    }
}

fn generate_encounter(
    mut commands: Commands,
    assets: Res<MainGameAssets>,
    locations: Res<Assets<Locations>>,
    challengers: Res<Assets<Challengers>>,
) {
    let (Some(locations), Some(challengers)) = (
        locations.get(&assets.locations),
        challengers.get(&assets.challengers),
    ) else {
        return;
    };
    commands.insert_resource(EncounterSetup {
        location: locations.get("grass").cloned(),
        challengers: match challengers.get("monster") {
            Some(c) => vec![(1, c.clone())],
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
) {
    let bundler = SceneBundler::new(&assets, &materials, &gltf);
    if let Some(location) = &setup.location {
        info!("Spawning Location {location:?}");
        if let Some(bundle) = bundler.scene(&location.scene) {
            commands.spawn((bundle, EncounterEntity));
        } else {
            error!("Couldn't setup bundle");
        }
    }

    for (count, challenger) in setup.challengers.iter() {
        if let Some(bundle) = bundler.scene(&challenger.scene) {
            for _ in 0..*count {
                commands.spawn((bundle.clone(), EncounterEntity));
            }
        }
    }
}

fn despawn_encounter(mut commands: Commands, query: Query<Entity, With<EncounterEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
