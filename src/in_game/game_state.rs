use bevy::prelude::States;

#[derive(Clone, Eq, PartialEq, Copy, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    None,
    WorldMap,
    Encounter,
}

#[derive(Clone, Eq, PartialEq, Copy, Debug, Hash, Default, States)]
pub enum PauseState {
    #[default]
    None,
    Paused,
}
