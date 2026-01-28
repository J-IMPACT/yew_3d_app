use std::cell::RefCell;
use glam::Vec3;
use once_cell::unsync::OnceCell;

#[derive(Debug)]
struct Body {
    position: Vec3,
    velocity: Vec3,
    mass: f32,
}

impl Body {
    fn new(position: Vec3, mass: f32) -> Self {
        Self {
            position,
            velocity: Vec3::ZERO,
            mass
        }
    }
}

struct NBodySimulation {
    bodies: Vec<Body>,
    forces: Vec<Vec3>,
    g: f32,
    dt: f32,
}

impl NBodySimulation {
    fn new(n: usize) -> Self {
        let mut bodies = Vec::with_capacity(n);
        let radius = 8.0;
        let height = 16.0;

        for i in 0..n {
            let t = i as f32 / n as f32;
            let angle = t * std::f32::consts::TAU * 5.0;

            bodies.push(Body::new(
                Vec3 {
                    x: angle.cos() * radius,
                    y: angle.sin() * radius,
                    z: (t - 0.5) * height,
                },
                1.0 + t,
            ));
        }

        Self {
            bodies,
            forces: vec![Vec3::ZERO; n],
            g: 1.0,
            dt: 0.016,
        }
    }

    fn step(&mut self) {
        let n = self.bodies.len();
        self.forces.fill(Vec3::ZERO);

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let a = &self.bodies[i];
                    let b = &self.bodies[j];

                    let diff = b.position - a.position;
                    let dist_sq = diff.length_squared() + 1e-6;
                    let dir = diff.normalize();

                    let f = self.g * a.mass * b.mass / dist_sq;
                    self.forces[i] += dir * f;
                }
            }
        }

        for (body, force) in self.bodies.iter_mut().zip(self.forces.iter()) {
            let accel = *force / body.mass;
            body.velocity += accel * self.dt;
            body.position += body.velocity * self.dt;
        }
    }
}

thread_local! {
    static SIM: OnceCell<RefCell<NBodySimulation>> = OnceCell::new();
}

pub fn init_simulation(n: usize) {
    SIM.with(|cell| {
        cell.get_or_init(|| {
            RefCell::new(NBodySimulation::new(n))
        });
    });
}

pub fn step_simulation() {
    SIM.with(|cell| {
        let sim = cell.get().expect("Simulation not initialized");
        sim.borrow_mut().step();
    });
}

pub fn fill_xyz_f32(out: &mut Vec<f32>) {
    SIM.with(|cell| {
        out.clear();
        let sim = cell.get()
            .expect("Simulation not initialized")
            .borrow();
        out.reserve(sim.bodies.len() * 3);
        for b in &sim.bodies {
            out.push(b.position.x);
            out.push(b.position.y);
            out.push(b.position.z);
        }
    });
}