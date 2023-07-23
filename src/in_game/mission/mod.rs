pub mod mission_types;

use bevy::prelude::*;

use bevy_inspector_egui::InspectorOptions;
use bevy_ui_dsl::{root, text};

use crate::ui::{
    buttons::{focus_button, TypedFocusedButtonQuery},
    classes::*,
    intermediary_node_bundles::IntoIntermediaryNodeBundle,
};

use self::mission_types::{Mission, MissionAssetsPlugin, MissionStage};

use super::{
    encounter::encounter_setup_types::{self},
    game_state::GameState,
};

pub struct MissionPlugin;

impl Plugin for MissionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MissionAssetsPlugin)
            .register_type::<UiButton>()
        .add_systems(
            OnEnter(GameState::Mission),
            draw_encounter_selection_ui,
        )
        .add_systems(OnExit(GameState::Mission), clear_world_map)
        // .add_systems(
        //     Update,
        //     (
        //         draw_encounter_locations,
        //         find_encounter_locations,
        //         draw_encounter_selection_ui,
        //         update_encounter_selection_ui_position,
        //     )
        //         .run_if(in_state(GameState::Mission)),
        // )
        // .add_systems(
        //     InGameUpdate,
        //     (focused_button_activated.pipe(process_input)).run_if(in_state(GameState::Mission)),
        // )
        ;
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

    let r = root((), &asset_server, &mut commands, |p| {
        text(title, (), (main_text, knight_text), p);
        buttons = encounters
            .iter()
            .map(|encounter| {
                let button = focus_button(encounter_listing.nb(), apply_encounter_state, p, |p| {
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
    commands.entity(r).insert(MissionEntity);
    for (button, encounter) in buttons.into_iter() {
        commands.entity(button).insert(UiButton(encounter.clone()));
    }
}

fn process_input(
    In(focused): In<Option<Entity>>,
    mut commands: Commands,
    interaction_query: TypedFocusedButtonQuery<'_, '_, '_, UiButton>,
) {
    let Some(focused) = focused else {
        return;
    };
    let Some((_, btn)) = interaction_query.get(focused).ok() else {
        return;
    };
    commands.insert_resource(btn.0.clone());
    commands.insert_resource(NextState(Some(GameState::Encounter)));
}
