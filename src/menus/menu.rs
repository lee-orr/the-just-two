use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    app_state::AppState,
    assets::MainGameAssets,
    toon_material::{BaseMaterial, ToonMaterial},
    ui_classes::*,
};
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup)
            .add_systems(OnExit(AppState::MainMenu), exit)
            .add_systems(Update, process_input.run_if(in_state(AppState::MainMenu)));
    }
}

#[derive(Component)]
struct Screen;

fn setup(
    mut commands: Commands,
    assets: Res<MainGameAssets>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ToonMaterial>>,
) {
    commands.insert_resource(BaseMaterial(materials.add(ToonMaterial {
        color_texture: Some(assets.base_colors.clone()),
        shadow_texture: Some(assets.shadow_gradient.clone()),
    })));

    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    let r = root(c_root, &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb(), centered.nb()), p, |p| {
                text("The Just", (), (main_text, knight_text), p);
                text("Two", (), (main_text, druid_text), p);
            });
            text(
                "Press ENTER to start",
                primary_box_item.nb(),
                standard_text,
                p,
            );
            text(
                "Press C for the credits",
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
        commands.insert_resource(NextState(Some(AppState::InGame)));
    } else if keys.just_pressed(KeyCode::C) {
        commands.insert_resource(NextState(Some(AppState::Credits)));
    }
}
