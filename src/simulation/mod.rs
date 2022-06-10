use super::graphics;

use itertools::Itertools;
use rand::prelude::*;
use std::fmt;
use termion::color;
use vector2d::Vector2D;

const DEFAULT_GRAV_CONST: f32 = 0.005;
const MAX_FORCE_AMPLITUDE: Option<f32> = Some(10.0);
const MAX_TRAIL_LENGTH: Option<usize> = Some(1000);

// ----------------------------------------------------------------

pub struct Simulation {
    bodies: Vec<PhysicsBody>,
    grav_const: f32,
    physics_speed: u32,
}

#[allow(dead_code)]
impl Simulation {
    // Constructor
    pub fn new(bodies: Vec<PhysicsBody>, grav_const: Option<f32>, physics_speed: Option<u32>) -> Simulation {
        Simulation {
            bodies,
            grav_const: grav_const.unwrap_or(DEFAULT_GRAV_CONST),
            physics_speed: physics_speed.unwrap_or(1),
        }
    }

    // Immutable access
    // pub fn bodies(&self) -> &Vec<PhysicsBody> {
    //     &self.bodies
    // }

    pub fn grav_const(&self) -> &f32 {
        &self.grav_const
    }

    pub fn physics_speed(&self) -> &u32 {
        &self.physics_speed
    }

    // Mutable access
    pub fn bodies_mut(&mut self) -> &mut Vec<PhysicsBody> {
        &mut self.bodies
    }

    // Setters
    pub fn set_grav_const(&mut self, val: f32) {
        self.grav_const = val
    }

    pub fn set_physics_speed(&mut self, val: u32) {
        self.physics_speed = val.clamp(1, 16)
    }

    // Methods
    pub fn shapes(&self) -> Vec<Box<dyn graphics::Draw>> {
        let mut out: Vec<Box<dyn graphics::Draw>> = vec![];
        for i in &self.bodies {
            i.shape().into_iter().for_each(|x| out.push(x))
        }
        out
    }

    pub fn get_body(&self, i: usize) -> Option<&PhysicsBody> {
        if i < self.bodies.len() {
            Some(&self.bodies[i])
        } else {
            None
        }
    }

    pub fn remove_body(&mut self, i: usize) {
        if i < self.bodies.len() {
            self.bodies.remove(i);
        }
    }

    pub fn add_body(&mut self, physics_body: PhysicsBody) {
        self.bodies.push(physics_body);
    }

    pub fn gravity_between(&self, body1: &PhysicsBody, body2: &PhysicsBody) -> Force {
        let dist_between = body1.distance_between(body2);
        Force::new(
            Vector2D::new(body1.pos().x - body2.pos().x, body1.pos().y - body2.pos().y).normalise(),
            (self.grav_const * body1.mass() * body2.mass())
                / ((if dist_between > 1.0 { dist_between } else { 1.0 }).powf(2.0)),
        )
    }

    pub fn get_bodies_on_point(&self, p: Vector2D<f32>) -> Vec<&PhysicsBody> {
        self.bodies
            .iter()
            .filter(|x| ((x.pos.x - p.x).powf(2.0) + (x.pos.y - p.y).powf(2.0)) < x.radius.powf(2.0))
            .collect()
    }

    pub fn get_body_on_point_index(&self, p: Vector2D<f32>) -> Option<usize> {
        self.bodies
            .iter()
            .position(|x| ((x.pos.x - p.x).powf(2.0) + (x.pos.y - p.y).powf(2.0)) < x.radius.powf(2.0))
    }

    // Physics

    pub fn physics_tick(&mut self) {
        for _ in 0..self.physics_speed {
            self.gravity_tick();
            self.movement_tick();
        // self.collision_tick();
    }
    }

    pub fn movement_tick(&mut self) {
        self.bodies.iter_mut().for_each(|x| {
            x.move_self();
            x.add_trail();
        })
    }

    pub fn gravity_tick(&mut self) {
        let bodies = self.bodies.to_vec();

        (0..self.bodies.len()).permutations(2).into_iter().for_each(|x| {
            let grav_force_temp = -self.gravity_between(&bodies[x[0]], &bodies[x[1]]);
            let self_momentum_temp = self.bodies[x[0]].momentum;
            self.bodies[x[0]].set_momentum(self_momentum_temp + grav_force_temp);
        })
    }

    pub fn collision_tick(&mut self) {
        let bodies = self.bodies.to_vec();

        let mut to_del: Vec<usize> = vec![];
        (0..self.bodies.len()).combinations(2).into_iter().for_each(|x| {
            if bodies[x[0]].intersects(&bodies[x[1]]) {
                to_del.push(x[0]);
                to_del.push(x[1]);
            }
        });
        to_del.dedup();
        to_del.into_iter().rev().for_each(|x| {
            self.bodies.remove(x);
        });
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Force {
    direction: Vector2D<f32>,
    amplitude: f32,
}

#[allow(dead_code)]
impl Force {
    // Constructor
    pub fn new(direction: Vector2D<f32>, amplitude: f32) -> Force {
        Force {
            direction: direction.normalise(),
            amplitude: amplitude.abs().clamp(0.0, MAX_FORCE_AMPLITUDE.unwrap_or(f32::MAX)),
        }
    }

    pub fn new_rand() -> Force {
        let mut rng = rand::thread_rng();

        Force {
            direction: Vector2D::new(
                rng.gen::<f32>() * if rng.gen() { -1.0 } else { 1.0 },
                rng.gen::<f32>() * if rng.gen() { -1.0 } else { 1.0 },
            )
            .normalise(),
            amplitude: rng.gen::<f32>().abs(),
        }
    }

    // Immutable access
    pub fn direction(&self) -> &Vector2D<f32> {
        &self.direction
    }

    pub fn amplitude(&self) -> &f32 {
        &self.amplitude
    }

    // Setters
    pub fn set_direction(&mut self, val: Vector2D<f32>) {
        self.direction = val.normalise()
    }

    pub fn set_amplitude(&mut self, val: f32) {
        self.amplitude = val.abs().clamp(0.0, MAX_FORCE_AMPLITUDE.unwrap_or(f32::MAX))
    }

    // Methods
    pub fn as_vector2d(&self) -> Vector2D<f32> {
        Vector2D::new(self.direction.x * self.amplitude, self.direction.y * self.amplitude)
    }
}

impl std::ops::Add for Force {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let out_vector = Vector2D::new(self.direction.x * self.amplitude, self.direction.y * self.amplitude)
            + Vector2D::new(other.direction.x * other.amplitude, other.direction.y * other.amplitude);
        let normal = out_vector.normalise();

        Self::new(
            normal,
            out_vector.length() / if normal.length() != 0.0 { normal.length() } else { 1.0 },
        )
    }
}

impl std::ops::AddAssign for Force {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Neg for Force {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Force {
            direction: self.direction.neg(),
            amplitude: self.amplitude,
        }
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct PhysicsBody {
    pos: Vector2D<f32>,
    mass: f32,
    radius: f32,
    momentum: Force,
    color: graphics::Color,
    trail: Vec<Vector2D<f32>>,
}

#[allow(dead_code)]
impl PhysicsBody {
    // Constructor
    pub fn new(pos: Vector2D<f32>, mass: f32, momentum: Force, color: graphics::Color) -> PhysicsBody {
        PhysicsBody {
            pos,
            mass,
            radius: mass / 5.0,
            momentum,
            color,
            trail: Vec::with_capacity(MAX_TRAIL_LENGTH.unwrap_or(255)),
        }
    }

    pub fn new_rand() -> PhysicsBody {
        let mut rng = rand::thread_rng();
        let mass = rng.gen::<f32>() * 50.0;

        PhysicsBody {
            pos: Vector2D::new(rng.gen::<f32>() * 500.0, rng.gen::<f32>() * 500.0),
            mass,
            radius: mass / 5.0,
            momentum: Force::new_rand(),
            color: graphics::Color::new(
                (10.0 + rng.gen::<f32>() * 245.0) as u8,
                (10.0 + rng.gen::<f32>() * 245.0) as u8,
                (10.0 + rng.gen::<f32>() * 245.0) as u8,
            ),
            trail: Vec::with_capacity(MAX_TRAIL_LENGTH.unwrap_or(255)),
        }
    }

    // Immutable access
    pub fn pos(&self) -> &Vector2D<f32> {
        &self.pos
    }

    pub fn mass(&self) -> &f32 {
        &self.mass
    }

    pub fn momentum(&self) -> &Force {
        &self.momentum
    }

    pub fn color(&self) -> &graphics::Color {
        &self.color
    }

    pub fn trail(&self) -> &Vec<Vector2D<f32>> {
        &self.trail
    }

    // Setters
    pub fn set_pos(&mut self, val: Vector2D<f32>) {
        self.pos = val
    }
    pub fn set_mass(&mut self, val: f32) {
        self.mass = val
    }
    pub fn set_momentum(&mut self, val: Force) {
        self.momentum = val
    }
    pub fn set_color(&mut self, val: graphics::Color) {
        self.color = val
    }
    // pub fn pos_mut(&mut self) -> &mut Vector2D<f32> {
    //     &mut self.pos
    // }

    // pub fn mass_mut(&mut self) -> &mut f32 {
    //     &mut self.mass
    // }

    // pub fn momentum_mut(&mut self) -> &mut Force {
    //     &mut self.momentum
    // }

    // pub fn color_mut(&mut self) -> &mut graphics::Color {
    //     &mut self.color
    // }

    // Methods
    pub fn move_self(&mut self) {
        self.pos += self.momentum.as_vector2d();
    }

    pub fn add_trail(&mut self) {
        self.trail.push(self.pos);

        match MAX_TRAIL_LENGTH {
            Some(v) => {
                if self.trail.len() > v {
                    self.trail.remove(0);
                }
            }
            None => {}
        }
    }

    pub fn shape(&self) -> Vec<Box<dyn graphics::Draw>> {
        let mut out: Vec<Box<dyn graphics::Draw>> = vec![
            Box::new(graphics::Circle::new(self.pos, self.radius, 1, self.color)),
            Box::new(graphics::Line::new(
                self.pos,
                Vector2D::new(
                    self.pos.x + (self.momentum.direction().x * self.momentum.amplitude() * 20.0),
                    self.pos.y + (self.momentum.direction().y * self.momentum.amplitude() * 20.0),
                ),
                2,
                graphics::Color::new(255, 255, 255),
            )),
        ];
        for i in 1..self.trail.len() {
            out.push(Box::new(graphics::Line::new(
                self.trail[i - 1],
                self.trail[i],
                0,
                self.color,
            )))
        }
        // for i in 2..self.trail.len() {
        //     if i % 2 == 0 {
        //         out.push(Box::new(graphics::Line::new(
        //             self.trail[i - 1],
        //             self.trail[i],
        //             0,
        //             self.color,
        //         )))
        //     }
        // }
        out
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.distance_between(other) < (self.radius + other.radius)
    }

    pub fn distance_between(&self, other: &Self) -> f32 {
        ((other.pos.x - self.pos.x).powf(2.0) + (other.pos.y - self.pos.y).powf(2.0)).sqrt()
    }
}

impl fmt::Display for PhysicsBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}Pos: {:?}, Mass: {:?}, Radius: {:?}, Momentum: {:?}{}",
            self.color.bg_string(),
            self.pos,
            self.mass,
            self.radius,
            self.momentum,
            color::Fg(color::Reset)
        )
    }
}
