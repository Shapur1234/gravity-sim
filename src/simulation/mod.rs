use super::graphics;
use vector2d::Vector2D;

// ----------------------------------------------------------------

pub struct Simulation {
    bodies: Vec<PhysicsBody>,
    gravitational_constant: f32,
}

#[allow(dead_code)]
impl Simulation {
    // Constructor
    pub fn new(bodies: Vec<PhysicsBody>, gravitational_constant: Option<f32>) -> Simulation {
        Simulation {
            bodies,
            gravitational_constant: gravitational_constant.unwrap_or(0.001),
        }
    }

    // Immutable access
    pub fn bodies(&self) -> &Vec<PhysicsBody> {
        &self.bodies
    }

    pub fn gravitational_constant(&self) -> &f32 {
        &self.gravitational_constant
    }

    // Mutable access
    pub fn bodies_mut(&mut self) -> &mut Vec<PhysicsBody> {
        &mut self.bodies
    }

    pub fn gravitational_constant_mut(&mut self) -> &mut f32 {
        &mut self.gravitational_constant
    }

    // Methods
    pub fn shapes(&self) -> Vec<Box<dyn graphics::Draw>> {
        let mut out: Vec<Box<dyn graphics::Draw>> = vec![];
        for i in &self.bodies {
            out.push(Box::new(i.shape(graphics::Color::new(
                (255.0 / i.mass()) as u8,
                (255.0 / i.mass()) as u8,
                (255.0 / i.mass()) as u8,
            ))))
        }
        out
    }

    pub fn move_all(&mut self) {
        self.bodies.iter_mut().for_each(|x| x.move_self())
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone)]
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
            amplitude,
        }
    }

    // Immutable access
    pub fn direction(&self) -> &Vector2D<f32> {
        &self.direction
    }

    pub fn amplitude(&self) -> &f32 {
        &self.amplitude
    }

    // Mutable access
    pub fn amplitude_mut(&mut self) -> &mut f32 {
        &mut self.amplitude
    }

    // Setters
    pub fn set_direction(&mut self, val: Vector2D<f32>) {
        self.direction = val.normalise()
    }

    // Methods
    pub fn as_vector2d(&self) -> Vector2D<f32> {
        Vector2D::new(self.direction.x * self.amplitude, self.direction.y * self.amplitude)
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct PhysicsBody {
    pos: Vector2D<f32>,
    mass: f32,
    radius: f32,
    momentum: Force,
}

#[allow(dead_code)]
impl PhysicsBody {
    // Constructor
    pub fn new(pos: Vector2D<f32>, mass: f32, radius: f32, momentum: Force) -> PhysicsBody {
        PhysicsBody {
            pos,
            mass,
            radius,
            momentum,
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

    // Mutable access
    pub fn pos_mut(&mut self) -> &mut Vector2D<f32> {
        &mut self.pos
    }

    pub fn mass_mut(&mut self) -> &mut f32 {
        &mut self.mass
    }

    pub fn momentum_mut(&mut self) -> &mut Force {
        &mut self.momentum
    }

    // Methods
    pub fn move_self(&mut self) {
        self.pos += self.momentum.as_vector2d();
    }

    pub fn shape(&self, color: graphics::Color) -> graphics::Circle {
        graphics::Circle::new(self.pos, self.radius, color)
    }
}
