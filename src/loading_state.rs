use std::ops::Mul;

use bevy::prelude::*;
use bevy_vector_shapes::{prelude::ShapePainter, shapes::DiscPainter};

use crate::{state::AppState, ui_colors};

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_loading.run_if(in_state(AppState::LoadingMenu)));
    }
}

const LOADING_ANIMATION_SPEED: f32 = 5.;
const LOADING_ANIMATION_SIZE: f32 = 50.0;
const LOADING_ANIMATION_VERTICAL: f32 = 20.0;
const CIRCLE_SIZE: f32 = 15.;
const CIRCLE_DELAY: f32 = 2.;
const NUM_CIRCLES: u8 = 3;

fn draw_loading(mut painter: ShapePainter, time: Res<Time>) {
    painter.set_2d();

    let left_edge = (NUM_CIRCLES as f32 * LOADING_ANIMATION_SIZE) / -2.;

    for circle in 0..NUM_CIRCLES {
        let time_offset =
            (time.elapsed_seconds() - CIRCLE_DELAY * (circle as f32)).mul(LOADING_ANIMATION_SPEED);
        let offset_y = time_offset.cos();
        let location = Vec3::new(
            left_edge + (circle as f32) * LOADING_ANIMATION_SIZE,
            offset_y * LOADING_ANIMATION_VERTICAL,
            0.,
        );

        painter.set_translation(location);
        painter.color = ui_colors::PRIMARY_COLOR;
        painter.circle(CIRCLE_SIZE);
    }
}
