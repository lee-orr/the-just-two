use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    app_state::AppState,
    assets::MainGameAssets,
    ui::{classes::*, TypedButtonQuery},
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

#[derive(Component)]
enum Buttons {
    Start,
    Credits,
}

fn setup(mut commands: Commands, _assets: Res<MainGameAssets>, asset_server: Res<AssetServer>) {
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    let mut start_button = None;
    let mut credits_button = None;

    let r = root(c_root, &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb(), centered.nb()), p, |p| {
                text("The Just", (), (main_text, knight_text), p);
                text("Two", (), (main_text, druid_text), p);
            });
            text_button(
                "Start Game",
                (c_button.nb(), primary_box_item.nb()),
                button_text,
                p,
            )
            .set(&mut start_button);
            text_button(
                "Credits",
                (c_button.nb(), primary_box_item.nb()),
                button_text,
                p,
            )
            .set(&mut credits_button);
        });
    });
    commands.entity(r).insert(Screen);
    commands
        .entity(start_button.unwrap())
        .insert(Buttons::Start);
    commands
        .entity(credits_button.unwrap())
        .insert(Buttons::Credits);
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn process_input(mut commands: Commands, interaction_query: TypedButtonQuery<'_, '_, '_, Buttons>) {
    for (entity, interaction, btn) in interaction_query.iter() {
        let mut bundle = NodeBundle::default();
        c_button(&mut bundle);
        primary_box_item(&mut bundle);
        match interaction {
            Interaction::Pressed => {
                c_button_pressed(&mut bundle);
                match btn {
                    Buttons::Start => commands.insert_resource(NextState(Some(AppState::InGame))),
                    Buttons::Credits => {
                        commands.insert_resource(NextState(Some(AppState::Credits)))
                    }
                };
            }
            Interaction::Hovered => c_button_hovered(&mut bundle),
            Interaction::None => {}
        };
        commands.entity(entity).insert(bundle);
    }
}
