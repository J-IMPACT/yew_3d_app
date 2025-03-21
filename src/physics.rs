use std::ops::Add;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn zero() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    fn scale(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar
        }
    }

    fn distance(self, other: Vec3) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    fn direction_to(self, target: Vec3) -> Vec3 {
        let dx = target.x - self.x;
        let dy = target.y - self.y;
        let dz = target.z - self.z;
        let dist = (dx * dx + dy * dy + dz * dz).sqrt() + 1e-10;
        Vec3 {
            x: dx / dist,
            y: dy / dist,
            z: dz / dist, 
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

#[derive(Debug)]
struct Body {
    position: Vec3,
    velocity: Vec3,
    mass: f64,
}

impl Body {
    fn new(position: Vec3, mass: f64) -> Self {
        Self {
            position,
            velocity: Vec3::zero(),
            mass
        }
    }
}

struct NBodySimulation {
    bodies: Vec<Body>,
    g: f64,
    dt: f64,
}

impl NBodySimulation {
    fn new(n: usize) -> Self {
        let mut bodies = Vec::with_capacity(n);
        for i in 0..n {
            bodies.push(Body::new(
                Vec3 {
                    x: (i as f64) * 0.01,
                    y: (i as f64) * 0.02,
                    z: (i as f64) * 0.03,
                },
                1.0 + (i as f64) * 0.01,
            ));
        }

        Self {
            bodies,
            g: 6.674e-11,
            dt: 0.016,
        }
    }

    fn step(&mut self) {
        let n = self.bodies.len();
        let mut forces = vec![Vec3::zero(); n];

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let a = &self.bodies[i];
                    let b = &self.bodies[j];
                    let dist = a.position.distance(b.position);
                    let dir = a.position.direction_to(b.position);
                    let f = self.g * a.mass * b.mass / (dist * dist + 1e-10);
                    forces[i] = forces[i] + dir.scale(f);
                }
            }
        }

        for (body, &force) in self.bodies.iter_mut().zip(forces.iter()) {
            let accel = force.scale(1.0 / body.mass);
            body.velocity = body.velocity + accel.scale(self.dt);
            body.position = body.position + body.velocity.scale(self.dt);
        }
    }

    fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            self.step();
        }
    }
}

#[wasm_bindgen]
pub async fn simulate_n_body(n: usize) {
    let mut sim = NBodySimulation::new(n);
    sim.run(100);
    web_sys::console::log_1(&format!("Simulated {} bodies", n).into());
}