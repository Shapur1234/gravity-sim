use std::default;
use std::fmt;
use vector2d::Vector2D;

// ----------------------------------------------------------------

pub trait Draw {
    fn draw(&self, frame_buffer: &mut FrameBuffer);
    fn draw_outline(&self, frame_buffer: &mut FrameBuffer);

    fn offset(&self, offset_by: Vector2D<f32>) -> Box<dyn Draw>;
    fn scale(&self, times: f32) -> Box<dyn Draw>;

    fn z_index(&self) -> u32;
    fn set_z_index(&mut self, val: u32);
}

// ----------------------------------------------------------------

#[derive(Debug, Default)]
pub struct SceneUserInput {
    pub move_up: bool,
    pub move_down: bool,
    pub move_right: bool,
    pub move_left: bool,

    pub zoom_in: bool,
    pub zoom_out: bool,

    pub reset_view: bool,

    pub mouse_screen_pos: Option<Vector2D<f32>>,
    pub mouse_scroll_wheel: Option<f32>,
}

pub struct Scene {
    contents: Vec<Box<dyn Draw>>,
    res: Vector2D<u32>,
    offset: Vector2D<f32>,
    scale: f32,
    min_max_scale: Option<Vector2D<f32>>,
    base_scale: f32,
}

#[allow(dead_code)]
impl Scene {
    // Constructor
    pub fn new(contents: Vec<Box<dyn Draw>>, res: Vector2D<u32>, min_max_scale: Option<Vector2D<f32>>) -> Scene {
        Scene {
            contents,
            res,
            offset: Vector2D::new(0.0, 0.0),
            scale: 1.0,
            min_max_scale,
            base_scale: (res.x as f32) / 500.0,
        }
    }

    // Immutable access
    pub fn contents(&self) -> &Vec<Box<dyn Draw>> {
        &self.contents
    }

    pub fn res(&self) -> &Vector2D<u32> {
        &self.res
    }

    pub fn offset(&self) -> &Vector2D<f32> {
        &self.offset
    }

    pub fn min_max_scale(&self) -> &Option<Vector2D<f32>> {
        &self.min_max_scale
    }

    // Mutable access
    pub fn contents_mut(&mut self) -> &mut Vec<Box<dyn Draw>> {
        &mut self.contents
    }

    // Setters
    pub fn get_scale(&self) -> f32 {
        self.base_scale * self.scale
    }

    pub fn set_scale(&mut self, val: f32) {
        match self.min_max_scale {
            Some(x) => self.scale = val.clamp(x.x, x.y),
            None => self.scale = val,
        }
    }

    pub fn set_offset(&mut self, val: Vector2D<f32>) {
        self.offset = val
    }

    pub fn set_min_max_scale(&mut self, val: Option<Vector2D<f32>>) {
        self.min_max_scale = val
    }

    // Methods
    pub fn change_scale(&mut self, amount: f32) {
        let scale_old = self.scale;
        self.set_scale(self.scale + amount);
        {
            let res = Vector2D::new(self.res.x as f32, self.res.y as f32);
            if self.scale - scale_old != 0.0 {
                self.offset -= Vector2D::new(
                    ((res.x / (self.base_scale * scale_old)) - (res.x / (self.get_scale()))) / 2.0,
                    ((res.y / (self.base_scale * scale_old)) - (res.y / (self.get_scale()))) / 2.0,
                );
            }
        }
    }

    pub fn focus_on(&mut self, p: Vector2D<f32>) {
        self.offset = (Vector2D::new(self.res.x as f32, self.res.y as f32) / self.get_scale() / 2.0) - p
    }

    pub fn zoom_on(&mut self, amount: f32, on: Vector2D<f32>) {
        unimplemented!()
    }

    pub fn handle_user_input(&mut self, input: SceneUserInput) {
        if input.move_up {
            self.set_offset(Vector2D::new(self.offset.x, self.offset.y + (5.0 / self.get_scale())));
        }
        if input.move_down {
            self.set_offset(Vector2D::new(self.offset.x, self.offset.y - (5.0 / self.get_scale())));
        }
        if input.move_left {
            self.set_offset(Vector2D::new(self.offset.x + (5.0 / self.get_scale()), self.offset.y));
        }
        if input.move_right {
            self.set_offset(Vector2D::new(self.offset.x - (5.0 / self.get_scale()), self.offset.y));
        }

        if input.zoom_in {
            self.change_scale(0.05)
        }
        if input.zoom_out {
            self.change_scale(-0.05)
        }

        if let Some(mouse_screen_pos) = input.mouse_screen_pos {
            if let Some(mouse_scroll_wheel) = input.mouse_scroll_wheel {
                self.change_scale(mouse_scroll_wheel)
            } else {
            }
        } else {
        }
        // TODO: MOUSE!!!
        if input.reset_view {
            self.offset = Vector2D::new(0.0, 0.0);
            self.scale = 1.0;
        }
    }

    pub fn sort_contents(&mut self) {
        self.contents.sort_by_key(|x| x.z_index())
    }

    pub fn draw(&self, frame_buffer: &mut FrameBuffer) {
        self.contents
            .iter()
            .for_each(|shape| shape.offset(self.offset).scale(self.get_scale()).draw(frame_buffer));
    }

    pub fn to_frame_buffer(&self) -> FrameBuffer {
        let mut output = FrameBuffer::new(self.res);
        self.draw(&mut output);

        output
    }

    pub fn world_to_screen_coords(&self, pos: Vector2D<f32>) -> Vector2D<f32> {
        unimplemented!();
    }

    pub fn screen_to_world_coords(&self, pos: Vector2D<f32>) -> Vector2D<f32> {
        (pos / self.get_scale()) - self.offset
    }
}

impl fmt::Debug for Scene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Scale: {:?}, Offset: {:?}, Res: {:?}",
            self.scale, self.offset, self.res,
        )
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct FrameBuffer {
    buffer: Vec<Color>,
    size: Vector2D<u32>,
}

#[allow(dead_code)]
impl FrameBuffer {
    // Constructor
    pub fn new(size: Vector2D<u32>) -> FrameBuffer {
        FrameBuffer {
            buffer: vec![Color::new(0, 0, 0); (size.x * size.y) as usize],
            size,
        }
    }

    // Immutable access
    pub fn buffer(&self) -> &Vec<Color> {
        &self.buffer
    }

    pub fn size(&self) -> &Vector2D<u32> {
        &self.size
    }

    // Methods
    pub fn contains_point(&self, p: Vector2D<f32>) -> bool {
        p.x >= 0.0 && p.x < (self.size.x as f32) && p.y >= 0.0 && p.y < (self.size.y as f32)
    }

    pub fn set_pixel(&mut self, p: Vector2D<f32>, color: Color) {
        if self.contains_point(p) {
            let width = self.size.x as u32; // Dunno why I have to do that
            self.buffer[(((p.y as u32) * width) + (p.x as u32)) as usize] = color;
        }
    }

    pub fn draw(&mut self, object: &impl Draw) {
        object.draw(self);
    }

    pub fn draw_outline(&mut self, object: &impl Draw) {
        object.draw_outline(self);
    }

    // Casting
    pub fn to_vec_u8(&self, transparency: bool) -> Vec<u8> {
        let output_length = self.buffer.len();
        let mut output: Vec<u8> = Vec::with_capacity(output_length * (if transparency { 4 } else { 3 }));

        for i in 0..(output_length * (if transparency { 4 } else { 3 })) {
            let current_color = self.buffer[i / (if transparency { 4 } else { 3 })];

            output.push(current_color.r);
            output.push(current_color.g);
            output.push(current_color.b);
            if transparency {
                output.push(255);
            }
        }
        output
    }

    pub fn to_vec_u32(&self) -> Vec<u32> {
        self.buffer.iter().map(|x| x.to_u32()).collect()
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
impl Color {
    // Constructor
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    // Methods
    pub fn to_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    pub fn bg_string(self) -> String {
        format!("\x1b[38;2;{:?};{:?};{:?}m", self.r, self.g, self.b)
    }

    pub fn default_color() -> String {
        "\x1b[0m".to_string()
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pos_1: Vector2D<f32>,
    pos_2: Vector2D<f32>,
    color: Color,
    z_index: u32,
}

#[allow(dead_code)]
impl Line {
    // Constructor
    pub fn new(pos_1: Vector2D<f32>, pos_2: Vector2D<f32>, z_index: u32, color: Color) -> Line {
        Line {
            pos_1,
            pos_2,
            color,
            z_index,
        }
    }

    // Immutable access
    pub fn pos_1(&self) -> &Vector2D<f32> {
        &self.pos_1
    }

    pub fn pos_2(&self) -> &Vector2D<f32> {
        &self.pos_2
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    // Mutable access
    pub fn set_pos_1(&mut self, val: Vector2D<f32>) {
        self.pos_1 = val
    }

    pub fn set_pos_2(&mut self, val: Vector2D<f32>) {
        self.pos_2 = val
    }

    pub fn set_color(&mut self, val: Color) {
        self.color = val
    }
}

impl Draw for Line {
    fn draw(&self, frame_buffer: &mut FrameBuffer) {
        let (mut x0, mut y0) = (self.pos_1.x as i32, self.pos_1.y as i32);
        let (x1, y1) = (self.pos_2.x as i32, self.pos_2.y as i32);
        let (dx, dy) = ((x1 - x0).abs(), -(y1 - y0).abs());
        let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
        let mut error = dx + dy;

        loop {
            frame_buffer.set_pixel(Vector2D::new(x0 as f32, y0 as f32), self.color);

            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * error;
            if e2 >= dy {
                if x0 == x1 {
                    break;
                }
                error += dy;
                x0 += sx
            }
            if e2 <= dx {
                if y0 == y1 {
                    break;
                }
                error += dx;
                y0 += sy
            }
        }
    }

    fn draw_outline(&self, frame_buffer: &mut FrameBuffer) {
        frame_buffer.draw(self);
    }

    fn offset(&self, offset_by: Vector2D<f32>) -> Box<dyn Draw> {
        Box::new(Line::new(
            self.pos_1 + offset_by,
            self.pos_2 + offset_by,
            self.z_index,
            self.color,
        ))
    }

    fn scale(&self, times: f32) -> Box<dyn Draw> {
        Box::new(Line::new(
            Vector2D::new(self.pos_1.x * times, self.pos_1.y * times),
            Vector2D::new(self.pos_2.x * times, self.pos_2.y * times),
            self.z_index,
            self.color,
        ))
    }

    fn z_index(&self) -> u32 {
        self.z_index
    }

    fn set_z_index(&mut self, val: u32) {
        self.z_index = val
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pos: Vector2D<f32>,
    size: Vector2D<f32>,
    color: Color,
    z_index: u32,
}

#[allow(dead_code)]
impl Rect {
    // Constructor
    pub fn new(pos: Vector2D<f32>, size: Vector2D<f32>, z_index: u32, color: Color) -> Rect {
        Rect {
            pos,
            size: Vector2D::new(size.x.abs(), size.y.abs()),
            z_index,
            color,
        }
    }

    // Immutable access
    pub fn pos(&self) -> &Vector2D<f32> {
        &self.pos
    }

    pub fn size(&self) -> &Vector2D<f32> {
        &self.size
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    // Setters
    pub fn set_size(&mut self, val: Vector2D<f32>) {
        self.size = Vector2D::new(val.x.abs(), val.y.abs())
    }

    pub fn set_pos(&mut self, val: Vector2D<f32>) {
        self.pos = val
    }

    pub fn set_color(&mut self, val: Color) {
        self.color = val
    }
}

impl Draw for Rect {
    fn draw(&self, frame_buffer: &mut FrameBuffer) {
        // TODO: Check if on screen
        for y in 0..(self.size.y as usize) {
            for x in 0..(self.size.x as usize) {
                frame_buffer.set_pixel(
                    Vector2D::new((x as f32) + self.pos.x, (y as f32) + self.pos.y),
                    self.color,
                )
            }
        }
    }

    fn draw_outline(&self, frame_buffer: &mut FrameBuffer) {
        frame_buffer.draw(&Line::new(
            self.pos,
            Vector2D::new(self.pos.x + self.size.x, self.pos.y),
            self.z_index,
            self.color,
        ));
        frame_buffer.draw(&Line::new(
            self.pos,
            Vector2D::new(self.pos.x, self.pos.y + self.size.y),
            self.z_index,
            self.color,
        ));
        frame_buffer.draw(&Line::new(
            Vector2D::new(self.pos.x + self.size.x, self.pos.y),
            self.pos + self.size,
            self.z_index,
            self.color,
        ));
        frame_buffer.draw(&Line::new(
            Vector2D::new(self.pos.x, self.pos.y + self.size.y),
            self.pos + self.size,
            self.z_index,
            self.color,
        ));
    }

    fn offset(&self, offset_by: Vector2D<f32>) -> Box<dyn Draw> {
        Box::new(Rect::new(self.pos + offset_by, self.size, self.z_index, self.color))
    }

    fn scale(&self, times: f32) -> Box<dyn Draw> {
        let new_size = Vector2D::new(self.size.x * times, self.size.x * times);
        Box::new(Rect::new(
            Vector2D::new(self.pos.x * times, self.pos.y * times),
            new_size,
            self.z_index,
            self.color,
        ))
    }

    fn z_index(&self) -> u32 {
        self.z_index
    }

    fn set_z_index(&mut self, val: u32) {
        self.z_index = val
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pos: Vector2D<f32>,
    radius: f32,
    color: Color,
    z_index: u32,
}

#[allow(dead_code)]
impl Circle {
    // Constructor
    pub fn new(pos: Vector2D<f32>, radius: f32, z_index: u32, color: Color) -> Circle {
        Circle {
            pos,
            radius: radius.abs(),
            z_index,
            color,
        }
    }

    // Immutable access
    pub fn pos(&self) -> &Vector2D<f32> {
        &self.pos
    }

    pub fn radius(&self) -> &f32 {
        &self.radius
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    // Setters
    pub fn set_radius(&mut self, val: f32) {
        self.radius = val.abs()
    }

    pub fn set_pos(&mut self, val: Vector2D<f32>) {
        self.pos = val
    }

    pub fn color_mut(&mut self, val: Color) {
        self.color = val
    }
}

impl Draw for Circle {
    fn draw(&self, frame_buffer: &mut FrameBuffer) {
        if self.pos.x + self.radius >= 0.0
            && self.pos.x - self.radius <= frame_buffer.size().x as f32
            && self.pos.y + self.radius >= 0.0
            && self.pos.y - self.radius <= frame_buffer.size().y as f32
        {
            for y in -(self.radius as isize)..(self.radius as isize) {
                for x in -(self.radius as isize)..(self.radius as isize) {
                    if (x.pow(2) + y.pow(2)) <= (self.radius as isize).pow(2) {
                        frame_buffer.set_pixel(
                            Vector2D::new(self.pos.x + (x as f32), self.pos.y + (y as f32)),
                            self.color,
                        )
                    }
                }
            }
        }
    }

    fn draw_outline(&self, frame_buffer: &mut FrameBuffer) {
        fn draw_circle(c: Vector2D<i32>, p: Vector2D<i32>, color: Color, frame_buffer: &mut FrameBuffer) {
            frame_buffer.set_pixel(Vector2D::new((c.x + p.x) as f32, (c.y + p.y) as f32), color);
            frame_buffer.set_pixel(Vector2D::new((c.x - p.x) as f32, (c.y + p.y) as f32), color);
            frame_buffer.set_pixel(Vector2D::new((c.x + p.x) as f32, (c.y - p.y) as f32), color);
            frame_buffer.set_pixel(Vector2D::new((c.x - p.x) as f32, (c.y - p.y) as f32), color);
            frame_buffer.set_pixel(Vector2D::new((c.x + p.y) as f32, (c.y + p.x) as f32), color);
            frame_buffer.set_pixel(Vector2D::new((c.x - p.y) as f32, (c.y + p.x) as f32), color);
            frame_buffer.set_pixel(Vector2D::new((c.x + p.y) as f32, (c.y - p.x) as f32), color);
            frame_buffer.set_pixel(Vector2D::new((c.x - p.y) as f32, (c.y - p.x) as f32), color);
        }

        if self.radius < 2.0 {
            frame_buffer.set_pixel(self.pos, self.color)
        } else {
            let center_pos: Vector2D<i32> = Vector2D::new(self.pos.x as i32, self.pos().y as i32);
            let mut p: Vector2D<i32> = Vector2D::new(0, self.radius as i32);
            let mut d: i32 = 3 - 2 * (self.radius as i32);

            draw_circle(center_pos, p, self.color, frame_buffer);
            while p.y >= p.x {
                p.x += 1;
                if d > 0 {
                    p.y -= 1;
                    d += 4 * (p.x - p.y) + 10;
                } else {
                    d += 4 * p.x + 6
                }
                draw_circle(center_pos, p, self.color, frame_buffer);
            }
        }
    }

    fn offset(&self, offset_by: Vector2D<f32>) -> Box<dyn Draw> {
        Box::new(Circle::new(self.pos + offset_by, self.radius, self.z_index, self.color))
    }

    fn scale(&self, times: f32) -> Box<dyn Draw> {
        Box::new(Circle::new(
            Vector2D::new(self.pos.x * times, self.pos.y * times),
            self.radius * times,
            self.z_index,
            self.color,
        ))
    }

    fn z_index(&self) -> u32 {
        self.z_index
    }

    fn set_z_index(&mut self, val: u32) {
        self.z_index = val
    }
}
