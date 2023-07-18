use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::{
    prelude::ReflectInspectorOptions, quick::StateInspectorPlugin, InspectorOptions,
};

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<EncounterState>()
            .register_type::<EncounterState>()
            .add_plugins(
                StateInspectorPlugin::<EncounterState>::default()
                    .run_if(input_toggle_active(false, KeyCode::F1)),
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
