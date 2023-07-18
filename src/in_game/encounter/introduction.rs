use bevy::prelude::*;

use bevy_ui_dsl::*;

use crate::{in_game::game_state::GameState, ui::classes::*};

use super::{EncounterSetup, EncounterState};
pub struct IntroductionPlugin;

impl Plugin for IntroductionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(EncounterState::Introduction), setup)
            .add_systems(OnExit(EncounterState::Introduction), exit)
            .add_systems(
                Update,
                process_input.run_if(in_state(EncounterState::Introduction)),
            );
    }
}

#[derive(Component)]
struct Screen;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    setup: Option<Res<EncounterSetup>>,
) {
    let Some(setup) = setup else {
        commands.insert_resource(NextState(Some(GameState::WorldMap)));
        return;
    };
    let r = root(c_root, &asset_server, &mut commands, |p| {
        node(primary_box, p, |p| {
            node((span.nb(), primary_box_main.nb()), p, |p| {
                text(
                    setup.title.as_deref().unwrap_or("An Encounter Awaits"),
                    (),
                    (
                        main_text,
                        match setup.player_faction {
                            crate::in_game::factions::Faction::Knights => knight_text,
                            crate::in_game::factions::Faction::Druids => druid_text,
                        },
                    ),
                    p,
                );
            });
            if let Some(intro) = setup.introduction.as_deref() {
                text(intro, primary_box_item.nb(), standard_text, p);
            }
            text(
                "Press Enter to Start",
                primary_box_item.nb(),
                standard_text,
                p,
            );
        });
    });
    commands.entity(r).insert(Screen);
}

fn exit(mut commands: Commands, query: Query<Entity, With<Screen>>) {
    commands.insert_resource(NextState(Some(EncounterState::None)));
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn process_input(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Return) {
        commands.insert_resource(NextState(Some(EncounterState::ActionChoice)));
    }
}
