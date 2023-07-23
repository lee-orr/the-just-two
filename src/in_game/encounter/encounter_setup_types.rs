use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

use crate::in_game::factions::Faction;

use super::{
    challenger::ChallengerReference, location::LocationReference, player::PlayerReference,
};

#[derive(Resource, Reflect, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct EncounterInitialDetails {
    pub title: Option<String>,
    pub player_faction: Faction,
    pub challengers: Vec<(usize, String)>,
    pub location: Option<String>,
}

impl Default for EncounterInitialDetails {
    fn default() -> Self {
        Self {
            title: Some("An Encounter".to_string()),
            player_faction: Faction::Knights,
            challengers: vec![(1, "monster".to_string())],
            location: Some("grass".to_string()),
        }
    }
}

#[derive(Resource, Reflect, InspectorOptions, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct EncounterSetup {
    pub title: Option<String>,
    pub introduction: Option<String>,
    pub player_faction: Faction,
    pub player: Option<PlayerReference>,
    pub challengers: Vec<(usize, ChallengerReference)>,
    pub location: Option<LocationReference>,
}

impl Default for EncounterSetup {
    fn default() -> Self {
        Self {
            title: Some("An Encounter".to_string()),
            introduction: Some("Let me introduce myself...".to_string()),
            player_faction: Faction::Knights,
            challengers: vec![],
            location: None,
            player: None,
        }
    }
}
