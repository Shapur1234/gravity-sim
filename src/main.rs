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
// Resizing support
// Zoom onto mouse

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let mut window = Window::new(
        "Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale_mode: ScaleMode::Stretch,
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        simulation.handle_user_input(SimulationInput {
            add_body: window.is_key_pressed(Key::Q, KeyRepeat::Yes),
            remove_body: window.is_key_pressed(Key::E, KeyRepeat::Yes),
            print_body: window.is_key_pressed(Key::R, KeyRepeat::Yes),
            selected_body: window.is_key_pressed(Key::V, KeyRepeat::No),
            up_speed: window.is_key_pressed(Key::NumPadPlus, KeyRepeat::Yes),
            down_speed: window.is_key_pressed(Key::NumPadMinus, KeyRepeat::Yes),
            reset_contents: window.is_key_pressed(Key::R, KeyRepeat::No),
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
        scene.handle_user_input(graphics::SceneUserInput {
            move_up: window.is_key_down(Key::Up) || window.is_key_down(Key::W),
            move_down: window.is_key_down(Key::Down) || window.is_key_down(Key::S),
            move_right: window.is_key_down(Key::Right) || window.is_key_down(Key::D),
            move_left: window.is_key_down(Key::Left) || window.is_key_down(Key::A),
            zoom_in: window.is_key_down(Key::M),
            zoom_out: window.is_key_down(Key::N),
            reset_view: window.is_key_pressed(Key::R, KeyRepeat::No),
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
        if let Some(selected_body) = *simulation.selected_body() {
            if let Some(body) = simulation.get_body(selected_body) {
                scene.focus_on(*body.pos())
            }
        }

        physics_on = physics_on ^ window.is_key_pressed(Key::Space, KeyRepeat::No);
        if physics_on {
            simulation.physics_tick();
        }

        *scene.contents_mut() = simulation.shapes();
        scene.sort_contents();
        window
            .update_with_buffer(&scene.to_frame_buffer().to_vec_u32(), WIDTH, HEIGHT)
            .unwrap();
    }
}
