use bevy::prelude::*;
use bevy_ui_dsl::{AssetClass, Class};
use bevy_ui_navigation::{components::FocusableButtonBundle, prelude::FocusState};

use super::colors::{self, *};

pub trait IntermediaryNodeBundleHandler {
    fn style(&mut self) -> &mut Style;
    fn background_color(&mut self) -> &mut BackgroundColor;
}

impl IntermediaryNodeBundleHandler for NodeBundle {
    fn style(&mut self) -> &mut Style {
        &mut self.style
    }

    fn background_color(&mut self) -> &mut BackgroundColor {
        &mut self.background_color
    }
}

impl IntermediaryNodeBundleHandler for TextBundle {
    fn style(&mut self) -> &mut Style {
        &mut self.style
    }

    fn background_color(&mut self) -> &mut BackgroundColor {
        &mut self.background_color
    }
}

impl IntermediaryNodeBundleHandler for ButtonBundle {
    fn style(&mut self) -> &mut Style {
        &mut self.style
    }

    fn background_color(&mut self) -> &mut BackgroundColor {
        &mut self.background_color
    }
}

impl IntermediaryNodeBundleHandler for FocusableButtonBundle {
    fn style(&mut self) -> &mut Style {
        &mut self.button_bundle.style
    }

    fn background_color(&mut self) -> &mut BackgroundColor {
        &mut self.button_bundle.background_color
    }
}

type Inner = Box<dyn FnOnce(&mut dyn IntermediaryNodeBundleHandler)>;

pub struct IntermediaryNodeBundle(Inner);

impl AssetClass<TextBundle> for IntermediaryNodeBundle {
    fn apply(self, _assets: &AssetServer, b: &mut TextBundle) {
        self.0(b)
    }
}
impl AssetClass<ButtonBundle> for IntermediaryNodeBundle {
    fn apply(self, _assets: &AssetServer, b: &mut ButtonBundle) {
        self.0(b)
    }
}
impl AssetClass<FocusableButtonBundle> for IntermediaryNodeBundle {
    fn apply(self, _assets: &AssetServer, b: &mut FocusableButtonBundle) {
        self.0(b)
    }
}

impl Class<NodeBundle> for IntermediaryNodeBundle {
    fn apply(self, b: &mut NodeBundle) {
        self.0(b)
    }
}

impl<F: FnOnce(&mut dyn IntermediaryNodeBundleHandler) + 'static> From<F>
    for IntermediaryNodeBundle
{
    fn from(value: F) -> Self {
        IntermediaryNodeBundle(Box::new(value))
    }
}

pub trait IntoIntermediaryNodeBundle {
    fn nb(self) -> IntermediaryNodeBundle;
}

impl<F: FnOnce(&mut dyn IntermediaryNodeBundleHandler) + 'static> IntoIntermediaryNodeBundle for F {
    fn nb(self) -> IntermediaryNodeBundle {
        self.into()
    }
}

pub fn c_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
    b.style.display = Display::Flex;
    b.style.justify_content = JustifyContent::Center;
    b.style.align_items = AlignItems::Center;
    b.style.position_type = PositionType::Absolute;
    b.style.left = Val::Px(0.);
    b.style.top = Val::Px(0.);
}

pub fn c_action_choice_root(b: &mut NodeBundle) {
    b.style.width = Val::Percent(100.);
    b.style.height = Val::Percent(100.);
    b.style.display = Display::Flex;
    b.style.flex_direction = FlexDirection::Row;
    b.style.justify_content = JustifyContent::Center;
    b.style.align_items = AlignItems::End;
    b.style.padding = UiRect::all(Val::Px(30.));
    b.style.column_gap = Val::Px(5.);
}

pub fn opaque(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::SCREEN_BACKGROUND_COLOR;
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

pub fn primary_box_main(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().grid_row = GridPlacement::start(1);
    b.style().grid_column = GridPlacement::start(1).set_span(3);
}

pub fn primary_box_item(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().grid_column = GridPlacement::start(2).set_span(1);
}

pub fn c_button(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().padding = UiRect::all(Val::Px(10.));
    b.style().border = UiRect::all(Val::Px(2.));
    b.style().margin = UiRect::all(Val::Px(10.));
    b.style().justify_content = JustifyContent::Center;
    b.style().align_items = AlignItems::Center;
    b.background_color().0 = colors::PRIMARY_COLOR;
}

pub fn c_button_prioritized(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_PRIORITIZED;
}
pub fn c_button_focused(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_PRIORITIZED;
}
pub fn c_button_active(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_ACTIVE;
}
pub fn c_button_blocked(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.background_color().0 = colors::PRIMARY_COLOR_BLOCKED;
}

pub fn apply_button_state(state: FocusState) -> NodeBundle {
    let mut bundle = NodeBundle::default();
    c_button(&mut bundle);
    primary_box_item(&mut bundle);
    match state {
        FocusState::Prioritized => c_button_prioritized(&mut bundle),
        FocusState::Focused => c_button_focused(&mut bundle),
        FocusState::Active => c_button_active(&mut bundle),
        FocusState::Blocked => c_button_blocked(&mut bundle),
        FocusState::Inert => {}
    };
    bundle
}

pub fn button_text(assets: &AssetServer, t: &mut TextStyle) {
    t.font_size = 20.;
    t.color = colors::BORDER_COLOR;
    t.font = assets.load("fonts/AMERSN__.ttf");
}

pub fn main_text(_: &AssetServer, t: &mut TextStyle) {
    t.font_size = 150.;
    t.color = PRIMARY_COLOR;
}

pub fn standard_text(assets: &AssetServer, t: &mut TextStyle) {
    t.font_size = 20.;
    t.color = PRIMARY_COLOR;
    t.font = assets.load("fonts/AMERSN__.ttf");
}

pub fn knight_text(assets: &AssetServer, t: &mut TextStyle) {
    t.font = assets.load("fonts/ENDOR___.ttf");
}

pub fn druid_text(assets: &AssetServer, t: &mut TextStyle) {
    t.font = assets.load("fonts/IMMORTAL.ttf");
}

pub fn span(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().display = Display::Flex;
    b.style().flex_direction = FlexDirection::Row;
    b.style().justify_content = JustifyContent::FlexStart;
    b.style().align_items = AlignItems::Center;
}

pub fn centered(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().justify_content = JustifyContent::Center;
}

pub fn card(b: &mut NodeBundle) {
    b.style.padding = UiRect::all(Val::Px(10.));
    b.style.border = UiRect::all(Val::Px(2.));
    b.border_color.0 = colors::BORDER_COLOR;
    b.background_color.0 = colors::PRIMARY_BACKGROUND_COLOR;

    b.style.display = Display::Grid;
    b.style.grid_template_columns = vec![
        GridTrack::auto(),
        GridTrack::percent(75.),
        GridTrack::auto(),
    ];
    b.style.grid_template_rows = vec![
        GridTrack::min_content(),
        GridTrack::flex(10.),
        GridTrack::min_content(),
    ];
}

pub fn card_title(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().grid_column = GridPlacement::start(2).set_span(1);
    b.style().grid_row = GridPlacement::start(1).set_span(1);
}

pub fn card_title_text(_: &AssetServer, t: &mut TextStyle) {
    t.font_size = 40.;
    t.color = PRIMARY_COLOR;
}

pub fn card_control(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().grid_column = GridPlacement::start(1).set_span(1);
    b.style().grid_row = GridPlacement::start(3).set_span(1);
    b.style().flex_direction = FlexDirection::Row;
    b.style().align_items = AlignItems::FlexEnd;
    b.style().top = Val::Px(5.);
}

pub fn card_content(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().grid_column = GridPlacement::start(2).set_span(1);
    b.style().grid_row = GridPlacement::start(2).set_span(1);
}

pub fn card_secondary_info(b: &mut dyn IntermediaryNodeBundleHandler) {
    b.style().grid_column = GridPlacement::start(3).set_span(1);
    b.style().grid_row = GridPlacement::start(3).set_span(1);
    b.style().flex_direction = FlexDirection::RowReverse;
    b.style().align_items = AlignItems::FlexEnd;
}
