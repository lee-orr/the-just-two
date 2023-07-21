use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use super::dice_pools::InitialPools;

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct ActionChoice {
    pub title: String,
    pub content: String,
    pub fail: usize,
    pub success: usize,
    pub critical_success: usize,
    pub dice_pool: InitialPools,
}

impl Default for ActionChoice {
    fn default() -> Self {
        Self {
            title: "An Action".to_string(),
            content: Default::default(),
            fail: 2,
            success: 6,
            critical_success: 9,
            dice_pool: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct ChosenAction;

#[derive(Component, Default)]
pub struct ChallengerAction;

#[derive(Bundle, Default)]
pub struct PlayerActionBundle {
    pub action_choice: ActionChoice,
}

#[derive(Bundle, Default)]
pub struct ChallengerActionBundle {
    pub action_choice: ActionChoice,
    pub challenger_action: ChallengerAction,
}
