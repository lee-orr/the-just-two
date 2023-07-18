mod app_state;
mod assets;
mod in_game;
mod menus;
mod toon_material;
mod ui_classes;
mod ui_colors;

use std::time::Duration;

use app_state::AppState;
use assets::MainGameAssets;
use bevy::{asset::ChangeWatcher, core_pipeline::clear_color::ClearColorConfig, prelude::*};
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};
use bevy_vector_shapes::Shape2dPlugin;
use credits::CreditsPlugin;
use in_game::InGamePlugin;
use loading_state::LoadingScreenPlugin;
use menu::MainMenuPlugin;
use menus::{credits, loading_state, menu};
use toon_material::{ToonMaterial, ToonMaterialPlugin};

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

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
            Shape2dPlugin::default(),
        ))
        .insert_resource(ClearColor(ui_colors::SCREEN_BACKGROUND_COLOR))
        .add_plugins((
            ToonMaterialPlugin,
            LoadingScreenPlugin,
            MainMenuPlugin,
            CreditsPlugin,
            InGamePlugin,
        ))
        .add_state::<AppState>()
        .add_loading_state(
            LoadingState::new(AppState::LoadingMenu).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_, MainGameAssets>(AppState::LoadingMenu)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    _materials: ResMut<Assets<ToonMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(-2., 5., -5.))
            .looking_at(Vec3::Y, Vec3::Y),
        ..default()
    });

    commands.spawn(Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        ..default()
    });
}
