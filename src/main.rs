mod graphics;

use graphics::*;

use minifb::{Key, ScaleMode, Window, WindowOptions};
use vector2d::Vector2D;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new(
        "Noise Test - Press ESC to exit",
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

    let mut scene = Scene::new(
        vec![
            Box::new(Rect::new(
                Vector2D::new(30.0, 30.0),
                Vector2D::new(50.0, 50.0),
                Color::new(0, 255, 255),
            )),
            Box::new(Rect::new(
                Vector2D::new(80.0, 30.0),
                Vector2D::new(50.0, 50.0),
                Color::new(255, 255, 255),
            )),
            Box::new(Rect::new(
                Vector2D::new(20.0, 90.0),
                Vector2D::new(50.0, 50.0),
                Color::new(0, 0, 255),
            )),
            Box::new(Rect::new(
                Vector2D::new(80.0, 90.0),
                Vector2D::new(50.0, 50.0),
                Color::new(0, 255, 0),
            )),
            // Box::new(Line::new(
            //     Vector2D::new(10.0, 5.0),
            //     Vector2D::new(40.0, 120.0),
            //     Color::new(255, 255, 255),
            // )),
            Box::new(Circle::new(Vector2D::new(100.0, 100.0), 20.0, Color::new(255, 0, 0))),
        ],
        Vector2D::new(WIDTH as u32, HEIGHT as u32),
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        keyboard_input(&mut scene, &window);

        window
            .update_with_buffer(&scene.to_framebuffer().to_vec_u32(), WIDTH, HEIGHT)
            .unwrap();
    }
}

fn keyboard_input(scene: &mut Scene, window: &Window) {
    if window.is_key_down(Key::Down) {
        scene.offset_mut().y -= 5.0 / *scene.scale();
        // scene.offset_mut().y += 5.0;
        println!("{scene:?}");
    }
    if window.is_key_down(Key::Up) {
        scene.offset_mut().y += 5.0 / *scene.scale();
        // scene.offset_mut().y -= 5.0;
        println!("{scene:?}");
    }
    if window.is_key_down(Key::Right) {
        scene.offset_mut().x -= 5.0 / *scene.scale();
        // scene.offset_mut().x += 5.0;
        println!("{scene:?}");
    }
    if window.is_key_down(Key::Left) {
        scene.offset_mut().x += 5.0 / *scene.scale();
        // scene.offset_mut().x -= 5.0;
        println!("{scene:?}");
    }

    if window.is_key_down(Key::M) {
        scene.change_scale(0.05)
    }
    if window.is_key_down(Key::N) {
        scene.change_scale(-0.05)
    }

    if window.is_key_down(Key::R) {
        *scene.offset_mut() = Vector2D::new(0.0, 0.0);
        scene.set_scale(1.0)
    }
}
