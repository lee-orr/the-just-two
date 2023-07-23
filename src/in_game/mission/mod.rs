pub mod mission_types;

use bevy::prelude::*;

use bevy_inspector_egui::InspectorOptions;
use bevy_ui_dsl::{node, root, text};

use crate::ui::{
    buttons::{focus_button, focused_button_activated, TypedFocusedButtonQuery},
    classes::*,
    intermediary_node_bundles::IntoIntermediaryNodeBundle,
};

use self::mission_types::{Mission, MissionAssetsPlugin, MissionStage};

use super::{
    encounter::encounter_setup_types::{self},
    game_state::GameState,
    story::PhaseRound,
    InGameUpdate,
};

pub struct MissionPlugin;

impl Plugin for MissionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MissionAssetsPlugin)
            .register_type::<UiButton>()
            .add_systems(
                OnEnter(GameState::Mission),
                (draw_encounter_selection_ui, check_progression),
            )
            .add_systems(OnExit(GameState::Mission), clear_world_map)
            .add_systems(
                InGameUpdate,
                (focused_button_activated.pipe(process_input)).run_if(in_state(GameState::Mission)),
            );
    }
}

#[derive(Component)]
pub struct MissionEntity;

#[derive(Component, Reflect, InspectorOptions)]
pub struct UiButton(encounter_setup_types::EncounterInitialDetails);

fn clear_world_map(mut commands: Commands, mission_entities: Query<Entity, With<MissionEntity>>) {
    for entity in mission_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn draw_encounter_selection_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mission: Res<Mission>,
    mission_stage: Res<MissionStage>,
) {
    let Some(encounters) = mission.encounters.get(mission_stage.0) else {
        return;
    };

    let title = mission.title.as_str();

    let mut buttons = Vec::new();

    let r = root(mission_root, &asset_server, &mut commands, |p| {
        node(mission_container, p, |p| {
            node(mission_encounter_title.nb(), p, |p| {
                text(title, (), (main_text, knight_text), p);
            });

            buttons = encounters
                .iter()
                .map(|encounter| {
                    let button =
                        focus_button(encounter_listing.nb(), apply_encounter_state, p, |p| {
                            text(
                                encounter.title.clone().unwrap_or("Encounter".to_string()),
                                (),
                                standard_text,
                                p,
                            );
                        });
                    (button, encounter)
                })
                .collect();
        });
    });
    commands.entity(r).insert(MissionEntity);
    for (button, encounter) in buttons.into_iter() {
        commands.entity(button).insert(UiButton(encounter.clone()));
    }
}

fn process_input(
    In(focused): In<Option<Entity>>,
    mut commands: Commands,
    interaction_query: TypedFocusedButtonQuery<'_, '_, '_, UiButton>,
    mission_stage: Option<Res<MissionStage>>,
) {
    let Some(mission_stage) = mission_stage else {
        return;
    };
    let Some(focused) = focused else {
        return;
    };
    let Some((_, btn)) = interaction_query.get(focused).ok() else {
        return;
    };
    commands.insert_resource(btn.0.clone());
    commands.insert_resource(MissionStage(mission_stage.0 + 1));
    commands.insert_resource(NextState(Some(GameState::Encounter)));
}

fn check_progression(
    mission_stage: Res<MissionStage>,
    mission: Res<Mission>,
    mut phase_round: ResMut<PhaseRound>,
    mut commands: Commands,
) {
    if mission_stage.0 >= mission.encounters.len() {
        commands.remove_resource::<Mission>();
        commands.remove_resource::<MissionStage>();
        phase_round.0 += 1;
        commands.insert_resource(NextState(Some(GameState::WorldMap)));
    }
}
