use bevy::{prelude::*, scene::Scene};
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::{app_state::AppState, toon_material::ToonMaterial};

pub struct MainGameAssetPlugin;

impl Plugin for MainGameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingMenu).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_, MainGameAssets>(AppState::LoadingMenu)
        .add_systems(OnExit(AppState::LoadingMenu), setup_materials);
    }
}

#[derive(AssetCollection, Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MainGameAssets {
    #[asset(path = "models/base-models.gltf#Scene0")]
    pub player_scene: Handle<Scene>,
    #[asset(path = "models/ground.gltf#Scene0")]
    pub ground: Handle<Scene>,
    #[asset(path = "textures/color-pallet.png")]
    pub base_colors: Handle<Image>,
    #[asset(path = "textures/shadow-gradient.png")]
    pub shadow_gradient: Handle<Image>,
    #[asset(path = "music/test.flac")]
    pub menu_music: Handle<AudioSource>,
    #[asset(path = "fonts/ENDOR___.ttf")]
    pub knights_font: Handle<Font>,
    #[asset(path = "fonts/IMMORTAL.ttf")]
    pub druids_font: Handle<Font>,
    #[asset(path = "fonts/AMERSN__.ttf")]
    pub default_font: Handle<Font>,
}

#[derive(Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Materials {
    pub base_material: Handle<ToonMaterial>,
}

fn setup_materials(
    mut commands: Commands,
    assets: Res<MainGameAssets>,
    mut materials: ResMut<Assets<ToonMaterial>>,
) {
    let base_material = materials.add(ToonMaterial {
        color_texture: Some(assets.base_colors.clone()),
        shadow_texture: Some(assets.shadow_gradient.clone()),
    });
    commands.insert_resource(Materials { base_material });
}
