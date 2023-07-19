use bevy::prelude::*;

pub mod classes;
pub mod colors;

pub type ButtonQuery<'w, 's, 'a> =
    Query<'w, 's, (Entity, &'a Interaction), (Changed<Interaction>, With<Button>)>;
pub type TypedButtonQuery<'w, 's, 'a, T> =
    Query<'w, 's, (Entity, &'a Interaction, &'a T), (Changed<Interaction>, With<Button>)>;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {}
}
