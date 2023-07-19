use crate::ui::classes::*;
use bevy::prelude::*;
use bevy_ui_dsl::*;

use super::EncounterState;

pub struct ActionChoicePlugin;

impl Plugin for ActionChoicePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(EncounterState::ActionChoice), setup)
            .add_systems(OnExit(EncounterState::ActionChoice), exit);
    }
}

#[derive(Component)]
struct Screen;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let r = root(c_action_choice_root, &asset_server, &mut commands, |p| {
        for i in 0..3 {
            node(card, p, |p| {
                text(
                    "Card Title",
                    card_title.nb(),
                    (card_title_text, druid_text),
                    p,
                );
                text(
                    "Card content goes on a little",
                    card_content.nb(),
                    standard_text,
                    p,
                );
                node(card_control.nb(), p, |p| {
                    text(format!("{i}"), (), (card_title_text, druid_text), p);
                });
                node(card_secondary_info.nb(), p, |p| {
                    text("1", (), (standard_text, druid_text), p);
                });
            });
        }
    });
    commands.entity(r).insert(Screen);
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}
