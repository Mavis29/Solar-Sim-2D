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
    mass: f64,           // in Solar Mass
    radius: f64,         // in AU
    position: Vec2,      // in AU
    last_position: Vec2, // in AU
    velocity: Vec2,      // in AU/s
    acceleration: Vec2,  // in AU/s^2
    color: Color,
}

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 1200;
const DISTANCE_SCALE: f64 = 150.0;
const RADIUS_SCALE: f64 = 15.0;
const G: f64 = 4.0 * PI * PI;
const UPS: f64 = 1000.0; // updates/s
const UD: f64 = 1.0 / UPS; // update delay in s
const YPS: f64 = 0.27397; // yr/s
const DT: f64 = YPS / UPS; // years/update

fn main() {
    let mut bodies: Vec<CelestialBody> = Vec::new();

    let sun = CelestialBody {
        name: "Sun".to_string(),
        mass: 1.0,
        radius: 1.5,
        position: Vec2 { x: 0.0, y: 0.0 },
        last_position: Vec2 { x: 0.0, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 0.0 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::YELLOW,
    };

    let earth = CelestialBody {
        name: "Earth".to_owned(),
        mass: 3.003e-6,
        radius: 0.5,
        position: Vec2 { x: 1.0, y: 0.0 },
        last_position: Vec2 { x: 1.0, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 6.283 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::AQUA,
    };

    let moon = CelestialBody {
        name: "Moon".to_owned(),
        mass: 3.69e-8,
        radius: 0.1,
        position: earth.position.add_vec(Vec2 { x: 0.00257, y: 0.0 }),
        last_position: earth.position.add_vec(Vec2 { x: 0.00257, y: 0.0 }),
        velocity: earth.velocity.add_vec(Vec2 { x: 0.0, y: 0.215 }),
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::WHITESMOKE,
    };

    let mars = CelestialBody {
        name: "Mars".to_owned(),
        mass: 3.27e-10,
        radius: 0.25,
        position: Vec2 { x: 1.52, y: 0.0 },
        last_position: Vec2 { x: 1.52, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 5.09 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::RED,
    };

    let mercury = CelestialBody {
        name: "Mercury".to_owned(),
        mass: 1.66e-7,
        radius: 0.1,
        position: Vec2 { x: 0.387, y: 0.0 },
        last_position: Vec2 { x: 0.387, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 10.1 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::GRAY,
    };

    let venus = CelestialBody {
        name: "Venus".to_owned(),
        mass: 2.45e-6,
        radius: 0.5,
        position: Vec2 { x: 0.72, y: 0.0 },
        last_position: Vec2 { x: 0.72, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 7.39 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::BEIGE,
    };

    let jupiter = CelestialBody {
        name: "Jupiter".to_owned(),
        mass: 9.54e-4,
        radius: 0.8,
        position: Vec2 { x: 5.2, y: 0.0 },
        last_position: Vec2 { x: 5.2, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 2.76 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::BEIGE,
    };

    let saturn = CelestialBody {
        name: "Saturn".to_owned(),
        mass: 2.86e-4,
        radius: 0.7,
        position: Vec2 { x: 9.5, y: 0.0 },
        last_position: Vec2 { x: 9.5, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 2.04 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::BEIGE,
    };

    let uranus = CelestialBody {
        name: "Uranus".to_owned(),
        mass: 4.37e-5,
        radius: 0.6,
        position: Vec2 { x: 19.2, y: 0.0 },
        last_position: Vec2 { x: 19.2, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 1.44 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::LIGHTBLUE,
    };

    let neptune = CelestialBody {
        name: "Neptune".to_owned(),
        mass: 5.15e-5,
        radius: 0.6,
        position: Vec2 { x: 30.07, y: 0.0 },
        last_position: Vec2 { x: 30.07, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 1.15 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::BLUE,
    };

    bodies.push(sun);
    bodies.push(earth);
    bodies.push(mars);
    bodies.push(mercury);
    bodies.push(venus);
    bodies.push(jupiter);
    bodies.push(saturn);
    bodies.push(uranus);
    bodies.push(neptune);

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .fullscreen()
        .title("Solar Sim")
        .msaa_4x()
        .build();

    let mut accumulator = 0.0;
    let mut last_frame: f64;
    let mut last_time: f64;
    let mut frames: i32 = 0;
    let mut fps: i32 = 0;
    unsafe {
        last_frame = GetTime();
        last_time = last_frame;
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

        let alpha = accumulator / UD;

        // Rendering
        if now - last_time >= 1.0 {
            last_time += 1.0;
            fps = frames;
            frames = 0;
        }

        d.clear_background(Color::BLACK);

        let fps_text = &format!("FPS: {fps}");
        d.draw_text(&fps_text, 10, 10, 20, Color::RAYWHITE);

        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();

        // 0, 0
        let center_x = screen_width / 2;
        let center_y = screen_height / 2;

        // Drawing planets
        d.draw_text("Velocities:", 10, 40, 20, Color::RAYWHITE);
        for i in 0..bodies.len() {
            let body = &bodies[i];
            let interp_x = interpolate(body.last_position.x, body.position.x, alpha);
            let interp_y = interpolate(body.last_position.y, body.position.y, alpha);
            let screen_x = center_x as f64 + interp_x * DISTANCE_SCALE;
            let screen_y = center_y as f64 - interp_y * DISTANCE_SCALE;
            d.draw_circle(
                screen_x as i32,
                screen_y as i32,
                (body.radius * RADIUS_SCALE) as f32,
                body.color,
            );
            let name = body.name.clone();
            let name_size = d.measure_text(&name, 12);
            let name_x = screen_x as i32 - name_size / 2;
            /*
                        d.draw_text(
                            &name,
                            name_x,
                            (screen_y + body.radius * RADIUS_SCALE) as i32,
                            10,
                            Color::RAYWHITE,
                        );
            */
            let j: i32 = i.try_into().unwrap();
            let text_y: i32 = 65 + j * 22;
            let velocity_text = &format!("{name} [{:.3}, {:.3}]", body.velocity.x, body.velocity.y);
            d.draw_text(&velocity_text, 10, text_y, 20, Color::RAYWHITE);
        }
        frames += 1;
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
            body.last_position = body.position;
            body.velocity.add(body.acceleration.scale_vec(DT));
            body.position.add(body.velocity.scale_vec(DT));
        }
    }

    fn interpolate(previous: f64, current: f64, alpha: f64) -> f64 {
        previous + (current - previous) * alpha
    }
}
