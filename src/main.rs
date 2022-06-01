mod graphics;
mod simulation;

use simulation::*;

use minifb::{Key, KeyRepeat, ScaleMode, Window, WindowOptions};
use vector2d::Vector2D;

const WIDTH: usize = 1260;
const HEIGHT: usize = 720;
const NUM_OF_BODIES: usize = 10;

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
    );

    let mut focused_body: Option<usize> = None;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        move_scene(&mut scene, &window);
        keyboard_input(&mut scene, &mut simulation, &mut physics_on, &mut focused_body, &window);

        match focused_body {
            Some(v) => match simulation.get_body(v) {
                Some(v) => scene.set_offset(scene.world_to_screen_coords(Vector2D::new(v.pos().x, v.pos().y))),
                None => {}
            },
            None => {}
        }

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

fn move_scene(scene: &mut graphics::Scene, window: &Window) {
    if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
        scene.set_offset(Vector2D::new(
            scene.offset().x,
            scene.offset().y + (5.0 / scene.get_scale()),
        ));
    }
    if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
        scene.set_offset(Vector2D::new(
            scene.offset().x,
            scene.offset().y - (5.0 / scene.get_scale()),
        ));
    }
    if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
        scene.set_offset(Vector2D::new(
            scene.offset().x + (5.0 / scene.get_scale()),
            scene.offset().y,
        ));
    }
    if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
        scene.set_offset(Vector2D::new(
            scene.offset().x - (5.0 / scene.get_scale()),
            scene.offset().y,
        ));
    }

    if window.is_key_down(Key::M) {
        scene.change_scale(0.05)
    }
    if window.is_key_down(Key::N) {
        scene.change_scale(-0.05)
    }

    match window.get_scroll_wheel() {
        Some(v) => scene.change_scale(v.1 / 50.0),
        None => {}
    }
}

fn keyboard_input(
    scene: &mut graphics::Scene,
    simulation: &mut Simulation,
    physics_on: &mut bool,
    focused_body: &mut Option<usize>,
    window: &Window,
) {
    if window.is_key_pressed(Key::Space, KeyRepeat::No) {
        *physics_on = !*physics_on;
    }

    if window.is_key_pressed(Key::Q, KeyRepeat::No) {
        match window.get_mouse_pos(minifb::MouseMode::Discard) {
            Some(v) => {
                let mut new_physics_body = PhysicsBody::new_rand();
                new_physics_body.set_pos(scene.screen_to_world_coords(Vector2D::new(v.0, v.1)));

                simulation.add_body(new_physics_body);
            }
            None => {}
        }
    }

    if window.is_key_pressed(Key::B, KeyRepeat::No) {
        match window.get_mouse_pos(minifb::MouseMode::Discard) {
            Some(v) => {
                let found = simulation.get_bodies_on_point(scene.screen_to_world_coords(Vector2D::new(v.0, v.1)));

                if found.len() > 0 {
                    println!("Bodies under point:");
                    found.into_iter().for_each(|x| println!("{x}"));
                    println!();
                }
            }
            None => {}
        }
    }

    if window.is_key_pressed(Key::V, KeyRepeat::No) {
        match window.get_mouse_pos(minifb::MouseMode::Discard) {
            Some(v) => {
                match simulation.get_body_on_point_index(scene.screen_to_world_coords(Vector2D::new(v.0, v.1))) {
                    Some(v) => *focused_body = Some(v),
                    None => *focused_body = None,
                }
            }
            None => {}
        }
    }

    if window.is_key_pressed(Key::R, KeyRepeat::No) {
        scene.set_offset(Vector2D::new(0.0, 0.0));
        scene.set_scale(1.0);
        *simulation.bodies_mut() = (0..NUM_OF_BODIES)
            .into_iter()
            .map(|_| PhysicsBody::new_rand())
            .collect();
    }
}
