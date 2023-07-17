use bevy::prelude::*;

use crate::ui_colors::*;

pub fn c_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
    b.style.display = Display::Flex;
    b.style.justify_content = JustifyContent::Center;
    b.style.align_items = AlignItems::Center;
}

pub fn primary_box(b: &mut NodeBundle) {
    b.style.margin = UiRect::all(Val::Px(10.));
    b.style.padding = UiRect::all(Val::Px(30.));
    b.background_color.0 = PRIMARY_BACKGROUND_COLOR;
    b.border_color.0 = BORDER_COLOR;
    b.style.border = UiRect::all(Val::Px(2.));
    b.style.display = Display::Grid;

    b.style.grid_template_columns = vec![GridTrack::auto(), GridTrack::auto(), GridTrack::auto()];
    b.style.grid_template_rows = vec![
        GridTrack::percent(50.),
        GridTrack::fr(1.),
        GridTrack::fr(1.),
    ];
    b.style.grid_auto_flow = GridAutoFlow::Column;

    b.style.align_items = AlignItems::Center;
    b.style.justify_content = JustifyContent::Center;

    b.style.row_gap = Val::Px(20.);
}

pub fn primary_box_main(_: &AssetServer, b: &mut TextBundle) {
    b.style.grid_row = GridPlacement::start(1);
    b.style.grid_column = GridPlacement::start(1).set_span(3);
}

pub fn primary_box_item(_: &AssetServer, b: &mut TextBundle) {
    b.style.grid_column = GridPlacement::start(2).set_span(1);
}

pub fn main_text(_: &AssetServer, t: &mut TextStyle) {
    t.font_size = 150.;
    t.color = PRIMARY_COLOR;
}

pub fn standard_text(_: &AssetServer, t: &mut TextStyle) {
    t.font_size = 20.;
    t.color = PRIMARY_COLOR;
}
