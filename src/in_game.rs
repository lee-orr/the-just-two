use bevy::prelude::*;

use crate::{
    assets::MainGameAssets,
    state::AppState,
    toon_material::{BaseMaterial, ToonMaterial},
};
pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup)
            .add_systems(OnExit(AppState::InGame), exit)
            .add_systems(Update, process_input.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
struct InGame;

fn setup(
    mut commands: Commands,
    assets: Res<MainGameAssets>,
    _asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ToonMaterial>>,
) {
    commands.insert_resource(BaseMaterial(materials.add(ToonMaterial {
        color_texture: Some(assets.base_colors.clone()),
        shadow_texture: Some(assets.shadow_gradient.clone()),
    })));

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
                ..Default::default()
            });
        });
}

fn exit(mut commands: Commands, query: Query<Entity, With<InGame>>) {
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn process_input(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::Escape) {
        commands.insert_resource(NextState(Some(AppState::MainMenu)));
    }
}
