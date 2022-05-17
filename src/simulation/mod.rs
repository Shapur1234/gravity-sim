use vector2d::Vector2D;
mod graphics

// ----------------------------------------------------------------

struct Space {
    bodies: Vec<PhysicsBody>,
    gravitational_constant: f64,
}

impl Space {
    pub fn new(bodies: Vec<PhysicsBody>, gravitational_constant: Option<f32>) -> Space {
        Space {
            bodies,
            gravitational_constant: gravitational_constant.unwrap_or(default: T) // FIFNISH
        }
    }
}

// ----------------------------------------------------------------

struct Force {
    direction: Vector2D<f32>,
    amplitude: f32,
}

impl Force {
    pub fn new(direction: Vector2D<f32>, amplitude: f32) -> Force {
        Force {
            direction, amplitude
        }
    }
}

// ----------------------------------------------------------------

struct PhysicsBody {
    pos: Vector2D<f32>,
    mass: f32,
    force: Force
    shape: graphics::Shape,
}

impl PhysicsBody {
    pub fn new(pos: Vector2D<f32>, mass: f32, force: Force) {
        PhysicsBody {
            pos, mass, force, shape: graphics::Shape::Circle
        }
    }
}

// ----------------------------------------------------------------
