use raylib::{
    color::Color,
    drawing::{RaylibDraw, RaylibDrawHandle},
};

use crate::SimState;

pub fn draw_fps(d: &mut RaylibDrawHandle, fps: i32) {
    let fps_text = &format!("FPS: {fps}");
    d.draw_text(&fps_text, 10, 10, 20, Color::RAYWHITE);
}

pub fn draw_planets(d: &mut RaylibDrawHandle, state: &SimState) {
    let screen_width = d.get_screen_width();
    let screen_height = d.get_screen_height();

    let center_x = screen_width / 2;
    let center_y = screen_height / 2;

    for i in 0..state.bodies.len() {
        let body = &state.bodies[i];
        let interp_x = interpolate(body.last_position.x, body.position.x, state.alpha);
        let interp_y = interpolate(body.last_position.y, body.position.y, state.alpha);
        let screen_x = center_x as f64 + interp_x * state.distance_scale;
        let screen_y = center_y as f64 - interp_y * state.distance_scale;
        d.draw_circle(
            screen_x as i32,
            screen_y as i32,
            (body.radius * state.radius_scale) as f32,
            body.color,
        );
    }
}

pub fn draw_velocities(d: &mut RaylibDrawHandle, state: &SimState) {
    d.draw_text("Velocities:", 10, 40, 20, Color::RAYWHITE);
    for i in 0..state.bodies.len() {
        let body = &state.bodies[i];
        let name = &body.name;
        let pos = i as i32;
        let text_y: i32 = 65 + pos * 22;
        let velocity_text = &format!("{name} [{:.3}, {:.3}]", body.velocity.x, body.velocity.y);
        d.draw_text(&velocity_text, 10, text_y, 20, Color::RAYWHITE);
    }
}

fn interpolate(previous: f64, current: f64, alpha: f64) -> f64 {
    previous + (current - previous) * alpha
}
