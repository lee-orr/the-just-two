use bevy::prelude::*;

use self::buttons::apply_button_styles;

pub mod buttons;
pub mod classes;
pub mod colors;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PreUpdate, apply_button_styles);
    }
}
