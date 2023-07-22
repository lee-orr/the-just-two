use bevy::prelude::*;
use bevy_ui_navigation::prelude::FocusState;

use crate::ui::{colors, intermediary_node_bundles::IntermediaryNodeBundleHandler};

pub fn encounter_listing(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(30.));
    b.background_color().0 = colors::OVERLAY_COLOR;
    b.style().bottom = Val::Px(20.);
}

pub fn encounter_prioritized(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_COLOR_PRIORITIZED;
}

pub fn encounter_focused(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_COLOR_FOCUSED;
}

pub fn encounter_active(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_COLOR_ACTIVE;
}

pub fn encounter_blocked(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::ENCOUNTER_COLOR_BLOCKED;
}

pub fn apply_encounter_state(state: FocusState) -> NodeBundle {
    let mut bundle = NodeBundle::default();
    encounter_listing(&mut bundle);
    match state {
        FocusState::Prioritized => encounter_prioritized(&mut bundle),
        FocusState::Focused => encounter_focused(&mut bundle),
        FocusState::Active => encounter_active(&mut bundle),
        FocusState::Blocked => encounter_blocked(&mut bundle),
        FocusState::Inert => {}
    };
    bundle
}
