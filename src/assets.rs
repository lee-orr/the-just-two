use bevy::{prelude::*, scene::Scene};
use bevy_asset_loader::prelude::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct MainGameAssets {
    #[asset(path = "base-models.gltf#Scene0")]
    pub player_scene: Handle<Scene>,
    #[asset(path = "ground.gltf#Scene0")]
    pub ground: Handle<Scene>,
    #[asset(path = "color-pallet.png")]
    pub base_colors: Handle<Image>,
    #[asset(path = "shadow-gradient.png")]
    pub shadow_gradient: Handle<Image>,
    #[asset(path = "test.flac")]
    pub menu_music: Handle<AudioSource>,
    #[asset(path = "fonts/ENDOR___.ttf")]
    pub knights_font: Handle<Font>,
    #[asset(path = "fonts/IMMORTAL.ttf")]
    pub druids_font: Handle<Font>,
    #[asset(path = "fonts/AMERSN__.ttf")]
    pub default_font: Handle<Font>,
}
