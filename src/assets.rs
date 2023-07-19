use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::{
    app_state::AppState,
    in_game::{Challengers, Locations, Players},
};

pub struct MainGameAssetPlugin;

impl Plugin for MainGameAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingMenu).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_, MainGameAssets>(AppState::LoadingMenu);
    }
}

#[derive(AssetCollection, Resource, Default, Reflect, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct MainGameAssets {
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
    #[asset(path = "challengers.ch.yaml")]
    pub challengers: Handle<Challengers>,
    #[asset(path = "locations.lc.yaml")]
    pub locations: Handle<Locations>,
    #[asset(path = "players.pl.yaml")]
    pub players: Handle<Players>,
}
