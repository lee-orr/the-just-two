use bevy::{prelude::*, reflect::TypeUuid};
use bevy_common_assets::yaml::YamlAssetPlugin;
use bevy_inspector_egui::InspectorOptions;
use serde::Deserialize;

use crate::{
    assets::MainGameAssets,
    materialized_scene::{MaterializedScene, MaterializedSceneBundle, MaterializedSceneReference},
    toon_material::ToonMaterial,
};

use super::game_state::GameState;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WorldMapAsset>()
            .add_plugins(YamlAssetPlugin::<WorldMapAsset>::new(&["wm.yaml"]))
            .add_systems(OnEnter(GameState::WorldMap), spawn_world_map)
            .add_systems(OnExit(GameState::WorldMap), clear_world_map);
    }
}

#[derive(Reflect, InspectorOptions, Deserialize, Clone, Debug, TypeUuid)]
#[uuid = "32067930-8002-4be1-aef3-e72a6d0d1612"]
pub struct WorldMapAsset {
    pub name: String,
    pub scene: MaterializedSceneReference,
}

#[derive(Component)]
pub struct WorldMapEntity;

fn spawn_world_map(
    mut commands: Commands,
    assets: Res<MainGameAssets>,
    mut materials: ResMut<Assets<ToonMaterial>>,
    camera: Query<Entity, With<Camera3d>>,
) {
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
