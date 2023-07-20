use bevy::prelude::*;
use bevy_ui_navigation::{systems::InputMapping, DefaultNavigationPlugins};

use self::buttons::apply_button_styles;

pub mod buttons;
pub mod classes;
pub mod colors;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(DefaultNavigationPlugins)
            .insert_resource(InputMapping {
                keyboard_navigation: true,
                focus_follows_mouse: true,
                key_action: KeyCode::Return,
                ..default()
            })
            .add_systems(PreUpdate, apply_button_styles);
    }
}
