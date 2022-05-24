mod graphics;
mod simulation;

use simulation::*;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use vector2d::Vector2D;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const NUM_OF_BODIES: usize = 50;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut window = Window::new(
        "Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale_mode: ScaleMode::UpperLeft,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to create window");

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut scene = graphics::Scene::new(
        vec![],
        Vector2D::new(WIDTH as u32, HEIGHT as u32),
        1.0,
        Some(Vector2D::new(0.1, 5.0)),
    );
    let mut simulation = Simulation::new(
        (0..NUM_OF_BODIES)
            .into_iter()
            .map(|_| PhysicsBody::new_rand())
            .collect(),
        None,
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        keyboard_input(&mut scene, &mut simulation, &window);
        simulation.physics_tick();

        *scene.contents_mut() = simulation.shapes();
        scene.sort_contents();

        window
            .update_with_buffer(&scene.to_frame_buffer().to_vec_u32(), WIDTH, HEIGHT)
            .unwrap();
    }
}

fn keyboard_input(scene: &mut graphics::Scene, simulation: &mut Simulation, window: &Window) {
    if window.is_key_down(Key::Down) {
        scene.offset_mut().y -= 5.0 / *scene.scale();
    }
    if window.is_key_down(Key::Up) {
        scene.offset_mut().y += 5.0 / *scene.scale();
    }
    if window.is_key_down(Key::Right) {
        scene.offset_mut().x -= 5.0 / *scene.scale();
    }
    if window.is_key_down(Key::Left) {
        scene.offset_mut().x += 5.0 / *scene.scale();
    }

    if window.is_key_down(Key::M) {
        scene.change_scale(0.05)
    }
    if window.is_key_down(Key::N) {
        scene.change_scale(-0.05)
    }

    if window.is_key_down(Key::R) {
        *scene.offset_mut() = Vector2D::new(0.0, 0.0);
        scene.set_scale(1.0);
        *simulation.bodies_mut() = (0..NUM_OF_BODIES)
            .into_iter()
            .map(|_| PhysicsBody::new_rand())
            .collect();
    }
}
