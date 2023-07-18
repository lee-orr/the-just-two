mod encounter;
mod factions;
mod game_state;
mod pause_screen;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::StateInspectorPlugin;

use crate::{
    app_state::AppState,
    assets::{MainGameAssets, Materials},
    scene_spawner::{MaterializedScene, MaterializedSceneBundle},
};

use self::{
    encounter::EncounterPlugin,
    game_state::{GameState, PauseState},
    pause_screen::PausePlugin,
};
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PausePlugin, EncounterPlugin))
            .add_state::<GameState>()
            .register_type::<GameState>()
            .add_plugins(
                StateInspectorPlugin::<GameState>::default()
                    .run_if(input_toggle_active(false, KeyCode::F1)),
            )
            .add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::InGame), (exit, clear_audio))
            .add_systems(Update, (enable_audio).run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct InGame;

fn setup(mut commands: Commands, assets: Res<MainGameAssets>, materials: Res<Materials>) {
    commands.insert_resource(NextState(Some(GameState::Encounter)));
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
            p.spawn(MaterializedSceneBundle {
                spawner: MaterializedScene {
                    scene: assets.ground.clone(),
                    material: materials.base_material.clone(),
                },
                ..Default::default()
            });
            p.spawn(MaterializedSceneBundle {
                spawner: MaterializedScene {
                    scene: assets.player_scene.clone(),
                    material: materials.base_material.clone(),
                },
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
    commands.insert_resource(NextState(Some(GameState::None)));
    commands.insert_resource(NextState(Some(PauseState::None)));
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn clear_audio(audio: Query<&AudioSink>) {
    for audio in audio.iter() {
        audio.stop();
    }
}

fn enable_audio(audio: Query<&AudioSink>) {
    for audio in audio.iter() {
        if audio.is_paused() {
            audio.play();
        }
    }
}
