use bevy::prelude::*;
use bevy_ui_dsl::*;

use crate::{
    app_state::AppState,
    ui::{classes::*, ButtonQuery},
};
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

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            text_button(
                "Main Menu",
                (c_button.nb(), primary_box_item.nb()),
                button_text,
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

fn process_input(mut commands: Commands, interaction_query: ButtonQuery) {
    for (entity, interaction) in interaction_query.iter() {
        let mut bundle = NodeBundle::default();
        c_button(&mut bundle);
        primary_box_item(&mut bundle);
        match interaction {
            Interaction::Pressed => {
                c_button_pressed(&mut bundle);
                commands.insert_resource(NextState(Some(AppState::MainMenu)));
            }
            Interaction::Hovered => c_button_hovered(&mut bundle),
            Interaction::None => {}
        };
        commands.entity(entity).insert(bundle);
    }
}
