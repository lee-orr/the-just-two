use bevy::{prelude::*, reflect::Reflect};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_ui_dsl::UiChildBuilder;

use crate::{
    assets::MainGameAssets,
    ui::{colors, spawn_icon, DisplayBundle},
};

use super::dice_pools::DiceType;

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

impl DisplayBundle for Power {
    fn display_bundle(&self, assets: &MainGameAssets, icon_size: Val, parent: &mut UiChildBuilder) {
        match self {
            Power::SplitDice => {
                parent.spawn(spawn_icon(8, assets, icon_size));
            }
            Power::AddDice(dice) => {
                parent
                    .spawn(NodeBundle {
                        ..Default::default()
                    })
                    .with_children(|p| {
                        p.spawn(TextBundle::from_section(
                            "+".to_string(),
                            TextStyle {
                                font: assets.knights_font.clone(),
                                font_size: 40.,
                                color: colors::PRIMARY_BUTTON_TEXT,
                            },
                        ));
                        dice.display_bundle(assets, icon_size, p);
                    });
            }
            Power::Advantage => {
                parent.spawn(spawn_icon(9, assets, icon_size));
            }
            Power::StaticBonus(v) => {
                parent.spawn(
                    TextBundle::from_section(
                        format!("+{v}"),
                        TextStyle {
                            font: assets.druids_font.clone(),
                            font_size: 30.,
                            color: colors::PRIMARY_BUTTON_TEXT,
                        },
                    )
                    .with_style(Style {
                        padding: UiRect::all(Val::Px(5.)),
                        ..default()
                    })
                    .with_background_color(colors::PRIMARY_BACKGROUND_COLOR),
                );
            }
        };
    }
}
