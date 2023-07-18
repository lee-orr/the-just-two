mod introduction;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::StateInspectorPlugin, InspectorOptions,
};

use self::introduction::IntroductionPlugin;

use super::{factions::Faction, game_state::GameState};

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
            .add_plugins(IntroductionPlugin)
            .add_systems(
                OnEnter(GameState::Encounter),
                generate_encounter.run_if(not(resource_exists::<EncounterSetup>())),
            )
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

#[derive(Clone, Eq, PartialEq, Copy, Debug, Hash, Default, States, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub enum EncounterState {
    #[default]
    None,
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
}

impl Default for EncounterSetup {
    fn default() -> Self {
        Self {
            title: Some("An Encounter".to_string()),
            introduction: Some("Let me introduce myself...".to_string()),
            player_faction: Faction::Knights,
        }
    }
}

fn generate_encounter(mut commands: Commands) {
    commands.insert_resource(EncounterSetup::default());
    info!("Generating Encounter");
}

fn start_encounter(mut commands: Commands) {
    commands.insert_resource(NextState(Some(EncounterState::Introduction)));
}
