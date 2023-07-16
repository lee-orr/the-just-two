mod toon_material;

use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*};
use toon_material::{ToonMaterial, ToonMaterialPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs_f32(0.5)),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            ToonMaterialPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (spawn_audio, set_material))
        .run();
}

#[derive(Resource)]
struct BaseMaterial(Handle<ToonMaterial>);

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ToonMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(-2., 5., -5.))
            .looking_at(Vec3::Y, Vec3::Y),
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("base-models.gltf#Scene0"),

        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load("ground.gltf#Scene0"),

        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    commands.spawn(PointLightBundle {
        // transform: Transform::from_xyz(5.0, 8.0, 2.0),
        transform: Transform::from_xyz(1.0, 10.0, -3.0),
        point_light: PointLight {
            intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
            color: Color::WHITE,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.insert_resource(BaseMaterial(materials.add(ToonMaterial {
        color_texture: Some(asset_server.load("color-pallet.png")),
        shadow_texture: Some(asset_server.load("shadow-gradient.png")),
    })));
}

fn set_material(
    mut commands: Commands,
    query: Query<Entity, With<Handle<StandardMaterial>>>,
    base: Option<Res<BaseMaterial>>,
) {
    let Some(base) = base else {
        return;
    };

    for entity in query.iter() {
        commands
            .entity(entity)
            .remove::<Handle<StandardMaterial>>()
            .insert(base.0.clone());
    }
}

fn spawn_audio(
    mut commands: Commands,
    query: Query<&AudioSink>,
    asset_server: Res<AssetServer>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if query.is_empty() {
            commands.spawn(AudioBundle {
                source: asset_server.load("test.flac"),
                ..default()
            });
        } else {
            for item in query.iter() {
                if item.is_paused() {
                    item.play()
                } else {
                    item.pause()
                }
            }
        }
    }
}
