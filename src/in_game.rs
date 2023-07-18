use bevy::prelude::*;

use crate::{assets::MainGameAssets, state::AppState, toon_material::ToonMaterial};
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::InGame), (exit, clear_audio))
            .add_systems(
                Update,
                (process_input, enable_audio).run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component)]
struct InGame;

fn setup(
    mut commands: Commands,
    assets: Res<MainGameAssets>,
    _asset_server: Res<AssetServer>,
    _materials: ResMut<Assets<ToonMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    commands
        .spawn((
            InGame,
            TransformBundle::default(),
            VisibilityBundle::default(),
        ))
        .with_children(|p| {
            p.spawn(SceneBundle {
                scene: assets.ground.clone(),
                ..Default::default()
            });
            p.spawn(SceneBundle {
                scene: assets.player_scene.clone(),
                ..Default::default()
            });
            p.spawn(PointLightBundle {
                transform: Transform::from_translation(Vec3::new(3., 5., -2.)),
                ..Default::default()
            });
            p.spawn(AudioBundle {
                source: assets.menu_music.clone(),
                settings: PlaybackSettings {
                    paused: true,
                    ..Default::default()
                },
            });
        });
}

fn exit(mut commands: Commands, query: Query<Entity, With<InGame>>) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn clear_audio(audio: Query<&AudioSink>) {
    for audio in audio.iter() {
        audio.stop();
    }
}

fn process_input(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
    }
}

fn enable_audio(audio: Query<&AudioSink>) {
    for audio in audio.iter() {
        if audio.is_paused() {
            audio.play();
        }
    }
}
