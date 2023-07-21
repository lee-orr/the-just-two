use crate::ui::intermediary_node_bundles::IntermediaryNodeBundleHandler;

use super::super::colors;
use bevy::prelude::*;

pub fn c_probability_setup_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
    b.style.display = Display::Flex;
    b.style.flex_direction = FlexDirection::Column;
    b.style.justify_content = JustifyContent::FlexEnd;
    b.style.align_items = AlignItems::Center;
    b.style.padding = UiRect::all(Val::Px(30.));
    b.style.row_gap = Val::Px(5.);
    b.style.column_gap = Val::Px(5.);
}

pub fn probability_grid(b: &mut NodeBundle) {
    b.style.display = Display::Flex;
    b.style.flex_wrap = FlexWrap::Wrap;
    b.style.row_gap = Val::Px(5.);
    b.style.column_gap = Val::Px(5.);
    b.style.flex_direction = FlexDirection::Row;
    b.style.justify_content = JustifyContent::Center;
}

pub fn probability_card(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().border = UiRect::all(Val::Px(2.));
    b.background_color().0 = colors::CARD_COLOR;

    b.style().display = Display::Grid;
    b.style().grid_template_rows = vec![GridTrack::fr(1.), GridTrack::fr(1.), GridTrack::fr(3.)];
    b.style().height = Val::VMin(20.);
    b.style().width = Val::VMin(30.);
}

pub fn player_card(b: &mut dyn IntermediaryNodeBundleHandler) {
    if let Some(b) = b.border_color() {
        b.0 = colors::SUCCESS_COLOR;
    }
}

pub fn challenger_card(b: &mut dyn IntermediaryNodeBundleHandler) {
    if let Some(b) = b.border_color() {
        b.0 = colors::FAIL_COLOR;
    }
}

pub fn probability_card_title(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(5.));
    b.style().justify_content = JustifyContent::Center;
    b.style().align_items = AlignItems::Center;
    b.background_color().0 = colors::OVERLAY_COLOR;
}

pub fn probability_card_title_text(_: &AssetServer, t: &mut TextStyle) {
    t.font_size = 20.;
    t.color = colors::PRIMARY_COLOR;
}

pub fn probability_card_dice_pool_container(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(5.));
    b.style().justify_content = JustifyContent::Center;
    b.style().align_items = AlignItems::Center;
}

pub fn probability_card_visualizer(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().justify_content = JustifyContent::Center;
    b.style().align_items = AlignItems::FlexEnd;
    b.style().flex_direction = FlexDirection::Row;
    b.style().width = Val::Percent(100.);
    b.background_color().0 = colors::VISUALIZER_BACKGROUND;
}

pub fn probability_power_container(b: &mut NodeBundle) {
    b.background_color.0 = colors::POWER_TOOLBAR_COLOR;
    b.style.padding = UiRect::all(Val::Px(5.));
}
