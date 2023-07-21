#![allow(dead_code)]
use bevy::prelude::Color;

pub const SCREEN_BACKGROUND_COLOR: Color = Color::rgb(0.36, 0.47, 0.65);
pub const BORDER_COLOR: Color = Color::rgb(0.19, 0.25, 0.35);
pub const PRIMARY_BACKGROUND_COLOR: Color = Color::rgb(0.29, 0.47, 0.42);
pub const PRIMARY_COLOR: Color = Color::rgb(0.83, 0.95, 0.83);

pub const PRIMARY_COLOR_PRIORITIZED: Color = Color::rgb(0.56, 0.9, 0.56);
pub const PRIMARY_COLOR_FOCUSED: Color = Color::rgb(0.39, 0.45, 0.39);
pub const PRIMARY_COLOR_ACTIVE: Color = Color::rgb(0.39, 0.45, 0.39);
pub const PRIMARY_COLOR_BLOCKED: Color = Color::rgb(0.4, 0.3, 0.4);
pub const PRIMARY_BUTTON_TEXT: Color = Color::rgb(0.07, 0.15, 0.12);

pub const OVERLAY_COLOR: Color = Color::rgba(0., 0., 0., 0.9);

pub const CRITICAL_COLOR: Color = Color::rgb(0.9, 0.64, 0.26);
pub const SUCCESS_COLOR: Color = Color::rgb(0.04, 0.91, 0.66);
pub const FAIL_COLOR: Color = Color::rgb(0.44, 0.29, 0.31);
pub const CRITICAL_FAIL_COLOR: Color = Color::rgb(0.98, 0.04, 0.3);

pub const CARD_COLOR: Color = Color::rgb(0.35, 0.42, 0.4);
pub const CARD_COLOR_PRIORITIZED: Color = PRIMARY_BACKGROUND_COLOR;
pub const CARD_COLOR_FOCUSED: Color = Color::rgb(0.23, 0.47, 0.4);
pub const CARD_COLOR_ACTIVE: Color = Color::rgb(0.18, 0.38, 0.32);
pub const CARD_COLOR_BLOCKED: Color = Color::rgb(0.4, 0.3, 0.4);

pub const VISUALIZER_BACKGROUND: Color = Color::rgba(0.9, 0.9, 0.9, 0.3);
pub const POWER_TOOLBAR_COLOR: Color = Color::rgba(0.83, 0.95, 0.83, 0.3);
