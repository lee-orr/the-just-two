use bevy::prelude::States;

#[derive(Clone, Eq, PartialEq, Copy, Debug, Hash, Default, States)]
pub enum AppState {
    #[default]
    LoadingMenu,
    MainMenu,
    Credits,
    InGame,
}
