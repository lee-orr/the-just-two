use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use super::dice_pools::InitialPools;

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct ActionChoice {
    pub title: String,
    pub content: String,
    pub fail: u8,
    pub success: u8,
    pub critical_success: u8,
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

pub enum ActionResult {
    CriticalFail,
    Fail,
    Success,
    CriticalSuccess,
}

impl ActionChoice {
    pub fn evaluate(&self, value: u8) -> (ActionResult, u8) {
        if value < self.fail {
            (ActionResult::CriticalFail, self.fail - value)
        } else if value < self.success {
            (ActionResult::Fail, self.success - value)
        } else if value < self.critical_success {
            (ActionResult::Success, value - self.success)
        } else {
            (ActionResult::CriticalSuccess, value - self.critical_success)
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
