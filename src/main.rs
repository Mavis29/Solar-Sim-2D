use std::{
    f64::consts::PI,
    ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign},
};

use raylib::{color::Color, drawing::RaylibDraw, ffi::GetTime};

mod renderer;

#[derive(Clone)]
struct SimState {
    distance_scale: f64, // 1 AU in pixels
    radius_scale: f64,   // radius scale in pixels
    ups: f64,            // updates per second
    ud: f64,             // update delay
    yps: f64,            // simulated earth years per second
    dt: f64,             // simulated time per update
    alpha: f64,
    bodies: Vec<CelestialBody>,
}

#[derive(Clone, Copy)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl MulAssign<f64> for Vec2 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Vec2 {
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
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
    name: &'static str,
    mass: f64,           // in Solar Mass
    radius: f64,         // in AU
    position: Vec2,      // in AU
    last_position: Vec2, // in AU
    velocity: Vec2,      // in AU/s
    acceleration: Vec2,  // in AU/s^2
    color: Color,
}

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 900;
const G: f64 = 4.0 * PI * PI;

const DEFAULT_STATE: SimState = SimState {
    distance_scale: 150.0,
    radius_scale: 15.0,
    ups: 1000.0,
    ud: 1.0 / 1000.0,
    yps: 0.27397,
    dt: 0.27397 / 1000.0,
    alpha: 0.0,
    bodies: Vec::new(),
};

fn main() {
    let mut state = DEFAULT_STATE.clone();
    state.bodies = create_bodies();

    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
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
        while accumulator >= state.ud {
            update(&mut state);
            accumulator -= state.ud;
        }

        state.alpha = accumulator / state.ud;

        // Rendering
        if now - last_time >= 1.0 {
            last_time += 1.0;
            fps = frames;
            frames = 0;
        }

        d.clear_background(Color::BLACK);
        renderer::draw_fps(&mut d, fps);
        renderer::draw_planets(&mut d, &state);
        renderer::draw_velocities(&mut d, &state);

        frames += 1;
    }
}

fn update(state: &mut SimState) {
    // Updating state
    state.ud = 1.0 / state.ups;
    state.dt = state.yps / state.ups;

    // Acceleration
    for i in 0..state.bodies.len() {
        let mut acceleration = Vec2 { x: 0.0, y: 0.0 };

        for j in 0..state.bodies.len() {
            if i == j {
                continue;
            }

            let direction = state.bodies[j].position - state.bodies[i].position;

            let distance = direction.magnitude();

            let scale = G * state.bodies[j].mass / (distance * distance);
            acceleration += direction.normalise_vec() * scale;
        }

        state.bodies[i].acceleration = acceleration;
    }

    // Velocity & Position
    for body in state.bodies.iter_mut() {
        body.last_position = body.position;
        body.velocity += body.acceleration * state.dt;
        body.position += body.velocity * state.dt;
    }
}

fn create_bodies() -> Vec<CelestialBody> {
    let mut bodies: Vec<CelestialBody> = Vec::new();

    let sun = CelestialBody {
        name: "Sun",
        mass: 1.0,
        radius: 1.5,
        position: Vec2 { x: 0.0, y: 0.0 },
        last_position: Vec2 { x: 0.0, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 0.0 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::YELLOW,
    };

    let earth = CelestialBody {
        name: "Earth",
        mass: 3.003e-6,
        radius: 0.5,
        position: Vec2 { x: 1.0, y: 0.0 },
        last_position: Vec2 { x: 1.0, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 6.283 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::AQUA,
    };

    #[allow(unused_variables)]
    let moon = CelestialBody {
        name: "Moon",
        mass: 3.69e-8,
        radius: 0.1,
        position: earth.position + Vec2 { x: 0.00257, y: 0.0 },
        last_position: earth.position + Vec2 { x: 0.00257, y: 0.0 },
        velocity: earth.velocity + Vec2 { x: 0.0, y: 0.215 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::WHITESMOKE,
    };

    let mars = CelestialBody {
        name: "Mars",
        mass: 3.27e-10,
        radius: 0.25,
        position: Vec2 { x: 1.52, y: 0.0 },
        last_position: Vec2 { x: 1.52, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 5.09 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::RED,
    };

    let mercury = CelestialBody {
        name: "Mercury",
        mass: 1.66e-7,
        radius: 0.1,
        position: Vec2 { x: 0.387, y: 0.0 },
        last_position: Vec2 { x: 0.387, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 10.1 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::GRAY,
    };

    let venus = CelestialBody {
        name: "Venus",
        mass: 2.45e-6,
        radius: 0.5,
        position: Vec2 { x: 0.72, y: 0.0 },
        last_position: Vec2 { x: 0.72, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 7.39 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::BEIGE,
    };

    let jupiter = CelestialBody {
        name: "Jupiter",
        mass: 9.54e-4,
        radius: 0.8,
        position: Vec2 { x: 5.2, y: 0.0 },
        last_position: Vec2 { x: 5.2, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 2.76 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::BEIGE,
    };

    let saturn = CelestialBody {
        name: "Saturn",
        mass: 2.86e-4,
        radius: 0.7,
        position: Vec2 { x: 9.5, y: 0.0 },
        last_position: Vec2 { x: 9.5, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 2.04 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::BEIGE,
    };

    let uranus = CelestialBody {
        name: "Uranus",
        mass: 4.37e-5,
        radius: 0.6,
        position: Vec2 { x: 19.2, y: 0.0 },
        last_position: Vec2 { x: 19.2, y: 0.0 },
        velocity: Vec2 { x: 0.0, y: 1.44 },
        acceleration: Vec2 { x: 0.0, y: 0.0 },
        color: Color::LIGHTBLUE,
    };

    let neptune = CelestialBody {
        name: "Neptune",
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

    bodies
}
