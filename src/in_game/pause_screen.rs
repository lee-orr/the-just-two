use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    app_state::AppState, assets::MainGameAssets, toon_material::ToonMaterial, ui_classes::*,
};

use super::game_state::PauseState;
pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<PauseState>()
            .add_systems(OnEnter(PauseState::Paused), setup)
            .add_systems(OnExit(PauseState::Paused), exit)
            .add_systems(Update, process_input.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct Screen;

fn setup(
    mut commands: Commands,
    _assets: Res<MainGameAssets>,
    asset_server: Res<AssetServer>,
    _materials: ResMut<Assets<ToonMaterial>>,
) {
    let r = root(c_root, &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb()), p, |p| {
                text("Game", (), (main_text, knight_text), p);
                text("Paused", (), (main_text, druid_text), p);
            });
            text(
                "Press Esc to Resume",
                primary_box_item.nb(),
                standard_text,
                p,
            );
            text(
                "Press X to return to Main Menu",
                primary_box_item.nb(),
                standard_text,
                p,
            );
        });
    });
    commands.entity(r).insert(Screen);
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn process_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    paused: Res<State<PauseState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(Some(match paused.get() {
            PauseState::None => PauseState::Paused,
            PauseState::Paused => PauseState::None,
        })));
    }
}
