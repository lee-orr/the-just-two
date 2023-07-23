use bevy::{prelude::*, utils::HashMap};

use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};
use bevy_ui_dsl::{node, root, text};
use bevy_vector_shapes::{prelude::ShapePainter, shapes::DiscPainter};

use crate::{
    assets::MainGameAssets,
    materialized_scene::{MaterializedScene, MaterializedSceneBundle},
    toon_material::ToonMaterial,
    ui::{
        buttons::{focus_button, focused_button_activated, TypedFocusedButtonQuery},
        classes::*,
        colors,
        intermediary_node_bundles::IntoIntermediaryNodeBundle,
        DisplayBundle,
    },
};

use super::{
    encounter::{powers::Power, EncounterInitialDetails},
    game_state::GameState,
    InGameUpdate,
};

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PotentialEncounters>()
            .register_type::<EncounterLocation>()
            .add_systems(
                OnEnter(GameState::WorldMap),
                (spawn_world_map, draw_available_powers),
            )
            .add_systems(OnExit(GameState::WorldMap), clear_world_map)
            .add_systems(
                Update,
                (
                    draw_encounter_locations,
                    find_encounter_locations,
                    draw_encounter_selection_ui,
                    update_encounter_selection_ui_position,
                )
                    .run_if(in_state(GameState::WorldMap)),
            )
            .add_systems(
                InGameUpdate,
                (focused_button_activated.pipe(process_input))
                    .run_if(in_state(GameState::WorldMap)),
            );
    }
}

#[derive(Component)]
pub struct WorldMapEntity;

#[allow(dead_code)]
const NUM_LOCATIONS_ON_MAP: usize = 14;

#[derive(Resource, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct PotentialEncounters(HashMap<usize, EncounterInitialDetails>);

#[derive(Component, Reflect, InspectorOptions)]
pub struct EncounterLocation(usize);

#[derive(Component)]
pub struct UiButtonLocation(Entity);

#[derive(Component)]
pub struct UiButton(usize);

impl Default for PotentialEncounters {
    fn default() -> Self {
        Self(
            [
                (0, EncounterInitialDetails::default()),
                (5, EncounterInitialDetails::default()),
                (13, EncounterInitialDetails::default()),
            ]
            .into_iter()
            .collect::<HashMap<usize, EncounterInitialDetails>>(),
        )
    }
}

fn spawn_world_map(
    mut commands: Commands,
    assets: Res<MainGameAssets>,
    mut materials: ResMut<Assets<ToonMaterial>>,
    camera: Query<Entity, With<Camera3d>>,
) {
    commands.insert_resource(PotentialEncounters::default());

    commands.insert_resource(AmbientLight {
        color: Color::rgba_u8(32, 20, 19, 255),
        brightness: 0.02,
    });
    commands.insert_resource(ClearColor(Color::rgb(0.75, 0.75, 0.75)));
    let material = materials.add(ToonMaterial {
        color_texture: Some(assets.default_color_pallet.clone()),
        shadow_texture: Some(assets.shadow_gradient.clone()),
    });
    commands.spawn((
        WorldMapEntity,
        MaterializedSceneBundle {
            spawner: MaterializedScene {
                scene: assets.world_map.clone(),
                material,
            },
            ..default()
        },
    ));

    for camera in camera.iter() {
        commands.entity(camera).insert((
            Transform::from_translation(Vec3::Y * 20.).looking_at(Vec3::ZERO, Vec3::Y),
            Projection::Orthographic(OrthographicProjection {
                scaling_mode: bevy::render::camera::ScalingMode::AutoMin {
                    min_width: 15.,
                    min_height: 15.,
                },
                ..Default::default()
            }),
        ));
    }
}

fn clear_world_map(
    mut commands: Commands,
    camera: Query<Entity, With<Camera3d>>,
    world_map_entities: Query<Entity, With<WorldMapEntity>>,
) {
    for camera in camera.iter() {
        commands
            .entity(camera)
            .insert(Projection::Perspective(PerspectiveProjection::default()));
    }
    for entity in world_map_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

type EncounterLocationTracking<'w, 's, 'a> =
    Query<'w, 's, (Entity, &'a Name), (With<GlobalTransform>, Without<EncounterLocation>)>;

fn find_encounter_locations(mut commands: Commands, locations: EncounterLocationTracking) {
    for (entity, location) in locations.iter() {
        if location.as_str().starts_with("Location.") {
            let name = location.as_str().replace("Location.", "");
            if let Ok(id) = name.parse::<usize>() {
                commands.entity(entity).insert(EncounterLocation(id));
            }
        }
    }
}

fn draw_encounter_locations(
    mut painter: ShapePainter,
    camera: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
    camera_2d: Query<(&GlobalTransform, &Camera), With<Camera2d>>,
    locations: Query<(&GlobalTransform, &EncounterLocation)>,
    potential_encounters: Res<PotentialEncounters>,
) {
    let Ok((camera_transform, camera)) = camera.get_single() else {
        return;
    };
    let Ok((camera_2d_transform, camera_2d)) = camera_2d.get_single() else {
        return;
    };

    for (transform, location) in locations.iter() {
        let Some(_) = potential_encounters.0.get(&location.0) else {
            continue;
        };
        let Some(normalized_coordinates) =
            camera.world_to_ndc(camera_transform, transform.translation())
        else {
            continue;
        };
        let Some(position) = camera_2d.ndc_to_world(camera_2d_transform, normalized_coordinates)
        else {
            continue;
        };
        painter.set_translation(position);
        painter.color = colors::FAIL_COLOR;
        painter.circle(13.);
        painter.color = colors::CRITICAL_FAIL_COLOR;
        painter.circle(10.);
    }
}

fn draw_encounter_selection_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    potential_encounters: Res<PotentialEncounters>,
    camera: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
    locations: Query<(Entity, &GlobalTransform, &EncounterLocation), Added<EncounterLocation>>,
) {
    if !potential_encounters.is_changed() && locations.is_empty() {
        return;
    }

    let Ok((camera_transform, camera)) = camera.get_single() else {
        return;
    };

    for (entity, transform, location) in locations.iter() {
        let Some(encounter) = potential_encounters.0.get(&location.0) else {
            continue;
        };
        let Some(viewport_coordinates) =
            camera.world_to_ndc(camera_transform, transform.translation())
        else {
            continue;
        };

        let mut button = None;

        let locator = root(
            move |b: &mut NodeBundle| {
                b.style.position_type = PositionType::Absolute;
                b.style.bottom = Val::Percent((viewport_coordinates.y + 1.) * 50.);
                b.style.left = Val::Percent((viewport_coordinates.x + 1.) * 50.);
                b.style.width = Val::Px(0.);
                b.style.height = Val::Px(0.);
                b.style.justify_content = JustifyContent::Center;
                b.style.align_items = AlignItems::FlexEnd;
            },
            &asset_server,
            &mut commands,
            |p| {
                button = Some(focus_button(
                    encounter_listing.nb(),
                    apply_encounter_state,
                    p,
                    |p| {
                        text(
                            encounter.title.clone().unwrap_or("Encounter".to_string()),
                            (),
                            standard_text,
                            p,
                        );
                    },
                ));
            },
        );
        commands
            .entity(locator)
            .insert((UiButtonLocation(entity), WorldMapEntity));
        if let Some(button) = button {
            commands.entity(button).insert(UiButton(location.0));
        }
    }
}

fn update_encounter_selection_ui_position(
    camera: Query<(&GlobalTransform, &Camera), With<Camera3d>>,
    locations: Query<(&GlobalTransform, &EncounterLocation)>,
    mut ui: Query<(&mut Style, &UiButtonLocation)>,
) {
    let Ok((camera_transform, camera)) = camera.get_single() else {
        return;
    };

    for (mut style, UiButtonLocation(entity)) in ui.iter_mut() {
        let Ok((transform, _)) = locations.get(*entity) else {
            continue;
        };
        let Some(viewport_coordinates) =
            camera.world_to_ndc(camera_transform, transform.translation())
        else {
            continue;
        };
        style.bottom = Val::Percent((viewport_coordinates.y + 1.) * 50.);
        style.left = Val::Percent((viewport_coordinates.x + 1.) * 50.);
    }
}

fn process_input(
    In(focused): In<Option<Entity>>,
    mut commands: Commands,
    interaction_query: TypedFocusedButtonQuery<'_, '_, '_, UiButton>,
    potential_encounters: Res<PotentialEncounters>,
) {
    let Some(focused) = focused else {
        return;
    };
    let Some((_, btn)) = interaction_query.get(focused).ok() else {
        return;
    };
    info!("Got Here...");
    if let Some(encounter) = potential_encounters.0.get(&btn.0) {
        commands.insert_resource(encounter.clone());
        commands.insert_resource(NextState(Some(GameState::Encounter)));
    }
}

fn draw_available_powers(
    mut commands: Commands,
    powers: Query<&Power>,
    assets: Res<MainGameAssets>,
    asset_server: Res<AssetServer>,
) {
    let r = root(map_powers_root, &asset_server, &mut commands, |p| {
        node(map_powers_container, p, |p| {
            for power in powers.iter() {
                node(map_power_card, p, |p| {
                    power.display_bundle(&assets, 50., p);
                });
            }
            node(map_powers_overlay, p, |_p| {});
        });
    });

    commands
        .entity(r)
        .insert((WorldMapEntity, Name::new("Available Powers Panel")));
}
