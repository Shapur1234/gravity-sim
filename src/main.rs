mod graphics;
mod simulation;

use simulation::*;

use minifb::{Key, KeyRepeat, ScaleMode, Window, WindowOptions};
use vector2d::Vector2D;

const WIDTH: usize = 1260;
const HEIGHT: usize = 720;
const NUM_OF_BODIES: usize = 10;

// TODO:
// Console mode
// Wasm version
// Cursor insert mode
// Move input into struct
// Improve focused body after remove
// Resizing support

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

    let mut physics_on = true;

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
        None,
        CollisionMode::None,
    );

    let mut focused_body: Option<usize> = None;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        scene.handle_user_input(graphics::SceneUserInput {
            move_up: window.is_key_down(Key::Up) || window.is_key_down(Key::W),
            move_down: window.is_key_down(Key::Down) || window.is_key_down(Key::S),
            move_right: window.is_key_down(Key::Right) || window.is_key_down(Key::D),
            move_left: window.is_key_down(Key::Left) || window.is_key_down(Key::A),
            zoom_in: window.is_key_down(Key::M),
            zoom_out: window.is_key_down(Key::N),
            mouse_screen_pos: if let Some(v) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                Some(scene.screen_to_world_coords(Vector2D::new(v.0, v.1)))
            } else {
                None
            },
            mouse_scroll_wheel: if let Some(v) = window.get_scroll_wheel() {
                Some(v.1)
            } else {
                None
            },
        });
        simulation.handle_user_input(SimulationInput {
            add_body: window.is_key_pressed(Key::Q, KeyRepeat::Yes),
            remove_body: window.is_key_pressed(Key::E, KeyRepeat::Yes),
            print_body: window.is_key_pressed(Key::R, KeyRepeat::Yes),
            up_speed: window.is_key_pressed(Key::NumPadPlus, KeyRepeat::Yes),
            down_speed: window.is_key_pressed(Key::NumPadPlus, KeyRepeat::Yes),
            mouse_world_pos: if let Some(v) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                Some(scene.screen_to_world_coords(Vector2D::new(v.0, v.1)))
            } else {
                None
            },
            mouse_scroll_wheel: if let Some(v) = window.get_scroll_wheel() {
                Some(v.1)
            } else {
                None
            },
        });

        keyboard_input(&mut scene, &mut simulation, &mut focused_body, &window);

        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            physics_on = !physics_on;
        }

        if physics_on {
            simulation.physics_tick();
        }

        match focused_body {
            Some(v) => match simulation.get_body(v) {
                Some(v) => scene.focus_on(*v.pos()),
                None => {}
            },
            None => {}
        }
        *scene.contents_mut() = simulation.shapes();
        scene.sort_contents();

        window
            .update_with_buffer(&scene.to_frame_buffer().to_vec_u32(), WIDTH, HEIGHT)
            .unwrap();
    }
}
fn keyboard_input(
    scene: &mut graphics::Scene,
    simulation: &mut Simulation,
    focused_body: &mut Option<usize>,
    window: &Window,
) {
    if window.is_key_pressed(Key::V, KeyRepeat::No) {
        match window.get_mouse_pos(minifb::MouseMode::Discard) {
            Some(v) => {
                *focused_body =
                    simulation.get_body_on_point_index(scene.screen_to_world_coords(Vector2D::new(v.0, v.1)))
            }
            None => {}
        }
    }

    if window.is_key_pressed(Key::R, KeyRepeat::No) {
        scene.set_offset(Vector2D::new(0.0, 0.0));
        scene.set_scale(1.0);
        *focused_body = None;
        *simulation.bodies_mut() = (0..NUM_OF_BODIES)
            .into_iter()
            .map(|_| PhysicsBody::new_rand())
            .collect();
    }
}
