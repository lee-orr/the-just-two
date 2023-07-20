use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_turborand::TurboRand;

pub trait Roll {
    fn roll(&self, rng: &mut impl TurboRand) -> u8;
}

#[derive(InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub enum DiceType {
    Standard { sides: u8 },
    Static { value: u8 },
}

impl Default for DiceType {
    fn default() -> Self {
        Self::Standard { sides: 10 }
    }
}

impl Roll for DiceType {
    fn roll(&self, rng: &mut impl TurboRand) -> u8 {
        match self {
            DiceType::Standard { sides } => {
                if *sides > 0 {
                    rng.sample_iter(1..*sides).unwrap_or(1)
                } else {
                    1
                }
            }
            DiceType::Static { value } => *value,
        }
    }
}

#[derive(InspectorOptions, Reflect, Default)]
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

#[derive(InspectorOptions, Reflect, Component)]
#[reflect(InspectorOptions)]
pub struct InitialPools(Vec<DicePool>);

impl Default for InitialPools {
    fn default() -> Self {
        Self(vec![Default::default()])
    }
}
