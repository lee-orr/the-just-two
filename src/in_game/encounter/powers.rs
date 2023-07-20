use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use super::dice_pools::{DiceType, InitialPools};

#[derive(Component, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub enum Power {
    SplitDice,
    // CombineDice,
    AddDice(DiceType),
    Advantage,
    StaticBonus(u8),
    // DefaceDice(DiceType),
    // ExplodeDice,
    // PreRoll,
    // ReRoll,
}

impl Power {}

impl Default for Power {
    fn default() -> Self {
        Self::StaticBonus(1)
    }
}
