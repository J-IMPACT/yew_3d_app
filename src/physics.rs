use std::{cell::RefCell, ops::Add};

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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
    pub bodies: Vec<Body>,
    forces: Vec<Vec3>,
    g: f64,
    dt: f64,
}

impl NBodySimulation {
    fn new(n: usize) -> Self {
        let mut bodies = Vec::with_capacity(n);
        for i in 0..n {
            let angle = (i as f64) * 0.2;
            bodies.push(Body::new(
                Vec3 {
                    x: angle.cos() * 10.0,
                    y: angle.sin() * 10.0,
                    z: i as f64 * 0.01,
                },
                1.0 + (i as f64) * 0.01,
            ));
        }

        Self {
            bodies,
            forces: vec![Vec3::zero(); n],
            g: 1.0,
            dt: 0.016,
        }
    }

    fn step(&mut self) {
        let n = self.bodies.len();
        self.forces.fill(Vec3::zero());

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let a = &self.bodies[i];
                    let b = &self.bodies[j];
                    let dist = a.position.distance(b.position);
                    let dir = a.position.direction_to(b.position);
                    let f = self.g * a.mass * b.mass / (dist * dist + 1e-10);
                    self.forces[i] = self.forces[i] + dir.scale(f);
                }
            }
        }

        for (body, &force) in self.bodies.iter_mut().zip(self.forces.iter()) {
            let accel = force.scale(1.0 / body.mass);
            body.velocity = body.velocity + accel.scale(self.dt);
            body.position = body.position + body.velocity.scale(self.dt);
        }
    }

    pub fn positions_slice(&self) -> &[Body] {
        &self.bodies
    }
}

thread_local! {
    static SIM: RefCell<Option<NBodySimulation>> = RefCell::new(None);
}

pub fn init_simulation(n: usize) {
    SIM.with(|sim| {
        *sim.borrow_mut() = Some(NBodySimulation::new(n));
    });
}

pub fn step_simulation() {
    SIM.with(|sim| {
        if let Some(sim) = &mut *sim.borrow_mut() {
            sim.step();
        }
    });
}

pub fn fill_xy_f32(out: &mut Vec<f32>, scale: f64) {
    SIM.with(|sim| {
        out.clear();
        if let Some(sim) = &*sim.borrow() {
            let bodies = sim.positions_slice();
            out.reserve(bodies.len() * 2);
            for b in bodies {
                out.push((b.position.x * scale) as f32);
                out.push((b.position.y * scale) as f32);
            }
        }
    });
}