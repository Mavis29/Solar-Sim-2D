use std::f64::consts::PI;

use raylib::{color::Color, drawing::RaylibDraw, ffi::GetTime};

#[derive(Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn add(&mut self, b: Vec2) {
        let x = self.x + b.x;
        let y = self.y + b.y;
        self.x = x;
        self.y = y;
    }

    fn subtract(&mut self, b: Vec2) {
        let x = self.x - b.x;
        let y = self.y - b.y;
        self.x = x;
        self.y = y;
    }

    fn scale(&mut self, s: f64) {
        self.x *= s;
        self.y *= s;
    }

    fn add_vec(&self, b: Vec2) -> Vec2 {
        let x = self.x + b.x;
        let y = self.y + b.y;
        Vec2 { x, y }
    }

    fn subtract_vec(&self, b: Vec2) -> Vec2 {
        let x = self.x - b.x;
        let y = self.y - b.y;
        Vec2 { x, y }
    }

    fn scale_vec(&self, s: f64) -> Vec2 {
        let x = self.x * s;
        let y = self.y * s;
        Vec2 { x, y }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalise(&mut self) {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
    }

    fn normalise_vec(&self) -> Vec2 {
        let mag = self.magnitude();
        let x = self.x / mag;
        let y = self.y / mag;
        Vec2 { x, y }
    }
}

#[derive(Clone)]
struct CelestialBody {
    name: String,
    mass: f64,          // in Solar Mass
    radius: f64,        // in AU
    position: Vec2,     // in AU
    velocity: Vec2,     // in AU/s
    acceleration: Vec2, // in AU/s^2
    color: Color,
}

const G: f64 = 4.0 * PI * PI;
const DT: f64 = 0.001;
const UD: f64 = 0.005;

fn main() {
    let mut bodies: Vec<CelestialBody> = Vec::new();

    let sun = CelestialBody {
        name: "Sun".to_string(),
        mass: 1.0,
        radius: 1.0,
        position: Vec2 { x: 0.0, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 0.0 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::YELLOW,
    };

    let earth = CelestialBody {
        name: "Earth".to_owned(),
        mass: 0.000003003,
        radius: 0.5,
        position: Vec2 { x: 1.0, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 6.283 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::BLUE,
    };

    let moon = CelestialBody {
        name: "Moon".to_owned(),
        mass: 0.0000000369397,
        radius: 0.1,
        position: earth.position.add_vec(Vec2 { x: 0.00257, y: 0.0 }),
        velocity: earth.velocity.add_vec(Vec2 { x: 0.0, y: 0.215 }),
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::WHITESMOKE,
    };

    let mars = CelestialBody {
        name: "Mars".to_owned(),
        mass: 0.0000003226,
        radius: 0.25,
        position: Vec2 { x: 1.52, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 5.09 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::RED,
    };

    bodies.push(sun);
    bodies.push(earth);
    bodies.push(mars);

    let (mut rl, thread) = raylib::init().size(800, 600).title("My Game").build();
    rl.set_target_fps(200);

    let dist_scale = 100.0;

    let mut accumulator = 0.0;
    let mut last_frame: f64;
    unsafe {
        last_frame = GetTime();
    }
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        let now = d.get_time();

        let delta_time = now - last_frame;
        last_frame = now;

        accumulator += delta_time;

        // Updating
        while accumulator >= UD {
            update(&mut bodies);
            accumulator -= UD;
        }

        // Rendering
        d.clear_background(Color::BLACK);

        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();

        // 0, 0
        let center_x = screen_width / 2;
        let center_y = screen_height / 2;

        // Drawing planets
        d.draw_text("Velocities:", 10, 10, 20, Color::RAYWHITE);
        for i in 0..bodies.len() {
            let body = &bodies[i];
            let screen_x = center_x as f64 + body.position.x * dist_scale;
            let screen_y = center_y as f64 - body.position.y * dist_scale;
            d.draw_circle(
                screen_x as i32,
                screen_y as i32,
                body.radius as f32 * 20.0,
                body.color,
            );
            let v_x = body.velocity.x.to_string();
            let v_y = body.velocity.y.to_string();
            let j: i32 = i.try_into().unwrap();
            let text_y: i32 = 35 + j * 22;
            let name = body.name.clone();
            let velocity_text = &format!("{name} [{:.4}, {:.5}]", v_x, v_y);
            d.draw_text(&velocity_text, 10, text_y, 20, Color::RAYWHITE);
        }
    }

    fn update(bodies: &mut Vec<CelestialBody>) {
        // Acceleration
        for i in 0..bodies.len() {
            let mut acceleration = Vec2 { x: 0.0, y: 0.0 };

            for j in 0..bodies.len() {
                if i == j {
                    continue;
                }

                let direction = bodies[j].position.subtract_vec(bodies[i].position);
                let distance = direction.magnitude();

                let scale = G * bodies[j].mass / (distance * distance);
                acceleration.add(direction.normalise_vec().scale_vec(scale));
            }

            bodies[i].acceleration = acceleration;
        }

        // Velocity
        for body in bodies.iter_mut() {
            body.velocity.add(body.acceleration.scale_vec(DT));
            body.position.add(body.velocity.scale_vec(DT));
        }
    }
}
