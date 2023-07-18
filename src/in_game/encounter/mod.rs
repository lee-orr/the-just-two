use bevy::prelude::*;

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Clone, Eq, PartialEq, Copy, Debug, Hash, Default, States)]
pub enum EncounterState {
    #[default]
    None,
    Introduction,
    ActionChoice,
    ProbabilitySetup,
    OutcomeResolution,
    EncounterResolved,
}
