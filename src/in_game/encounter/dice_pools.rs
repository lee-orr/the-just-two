use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_turborand::TurboRand;
use bevy_ui_dsl::{node, UiChildBuilder};

use crate::{
    assets::MainGameAssets,
    ui::{
        classes::dice_pool_modifier, colors, intermediary_node_bundles::IntoIntermediaryNodeBundle,
        spawn_icon, DisplayBundle,
    },
};

use super::powers::Power;

pub trait Roll {
    fn roll(&self, rng: &mut impl TurboRand) -> u8;
}

#[derive(InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub enum DiceType {
    D2,
    D3,
    D4,
    D6,
    D8,
    D12,
    Static { value: u8 },
}

impl Default for DiceType {
    fn default() -> Self {
        Self::D12
    }
}

impl Roll for DiceType {
    fn roll(&self, rng: &mut impl TurboRand) -> u8 {
        match self {
            DiceType::Static { value } => *value,
            DiceType::D2 => rng.u8(1..2),
            DiceType::D3 => rng.u8(1..3),
            DiceType::D4 => rng.u8(1..4),
            DiceType::D6 => rng.u8(1..6),
            DiceType::D8 => rng.u8(1..8),
            DiceType::D12 => rng.u8(1..12),
        }
    }
}

impl DisplayBundle for DiceType {
    fn display_bundle(&self, assets: &MainGameAssets, icon_size: Val, parent: &mut UiChildBuilder) {
        let (is_atlas, position) = match self {
            DiceType::D2 => (true, 0),
            DiceType::D3 => (true, 1),
            DiceType::D4 => (true, 2),
            DiceType::D6 => (true, 3),
            DiceType::D8 => (true, 4),
            DiceType::D12 => (true, 5),
            DiceType::Static { value } => (false, *value),
        };
        if is_atlas {
            parent.spawn(spawn_icon(position as usize, assets, icon_size));
        } else {
            parent.spawn(TextBundle::from_section(
                format!("+{position}"),
                TextStyle {
                    font: assets.druids_font.clone(),
                    font_size: 30.,
                    color: colors::PRIMARY_BUTTON_TEXT,
                },
            ));
        }
    }
}

#[derive(InspectorOptions, Reflect, Default, PartialEq, Eq)]
#[reflect(InspectorOptions)]
pub enum DicePoolType {
    #[default]
    Additive,
    Advantage,
}

#[derive(InspectorOptions, Reflect, Component)]
#[reflect(InspectorOptions)]
pub struct DicePool {
    pub num_dice: u8,
    pub dice: DiceType,
    pub pool: DicePoolType,
}

impl Roll for DicePool {
    fn roll(&self, rng: &mut impl TurboRand) -> u8 {
        if self.num_dice == 0 {
            return 1;
        }
        match self.pool {
            DicePoolType::Additive => {
                let mut result = 0;
                for _ in 1..self.num_dice {
                    result += self.dice.roll(rng);
                }
                result
            }
            DicePoolType::Advantage => {
                let mut result = 1;
                for _ in 1..self.num_dice {
                    result = result.max(self.dice.roll(rng));
                }
                result
            }
        }
    }
}

impl Default for DicePool {
    fn default() -> Self {
        Self {
            num_dice: 1,
            dice: Default::default(),
            pool: Default::default(),
        }
    }
}

impl DisplayBundle for DicePool {
    fn display_bundle(&self, assets: &MainGameAssets, icon_size: Val, parent: &mut UiChildBuilder) {
        if DicePoolType::Advantage == self.pool {
            node(dice_pool_modifier.nb(), parent, |p| {
                Power::Advantage.display_bundle(assets, icon_size, p);
            });
        }
        for _ in 0..self.num_dice {
            self.dice.display_bundle(assets, icon_size, parent);
        }
    }
}

#[derive(InspectorOptions, Reflect, Component)]
#[reflect(InspectorOptions)]
pub struct InitialPools(Vec<DicePool>);

impl Default for InitialPools {
    fn default() -> Self {
        Self(vec![Default::default()])
    }
}

impl InitialPools {
    pub fn new(pools: Vec<DicePool>) -> Self {
        Self(pools)
    }
    pub fn iter(&self) -> impl Iterator<Item = &DicePool> {
        self.0.iter()
    }
}
