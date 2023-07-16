use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, spawn_audio)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            color: Color::RED,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });
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
