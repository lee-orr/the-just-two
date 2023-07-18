use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{assets::MainGameAssets, state::AppState, toon_material::ToonMaterial, ui_classes::*};
pub struct CreditsPlugin;

impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Credits), setup)
            .add_systems(OnExit(AppState::Credits), exit)
            .add_systems(Update, process_input.run_if(in_state(AppState::Credits)));
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
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    let r = root(c_root, &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb()), p, |p| {
                text("The Just", (), (main_text, knight_text), p);
                text("Two", (), (main_text, druid_text), p);
            });
            text("by Lee-Orr", primary_box_item.nb(), standard_text, p);
            text(
                "Built using the Bevy Game Engine",
                primary_box_item.nb(),
                standard_text,
                p,
            );
            text(
                "Fonts by Appostrophic Labs, sourced from 1001freefonts.com",
                primary_box_item.nb(),
                standard_text,
                p,
            );
            text(
                "All other artistic assets created by Lee-Orr",
                primary_box_item.nb(),
                standard_text,
                p,
            );
            text(
                "Press Enter for the Main Menu",
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

fn process_input(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Return) {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
    }
}
