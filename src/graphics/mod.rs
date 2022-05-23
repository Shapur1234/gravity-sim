use std::fmt;
use vector2d::Vector2D;

// ----------------------------------------------------------------

pub trait Draw {
    fn draw(&self, frame_buffer: &mut FrameBuffer);
    fn draw_outline(&self, frame_buffer: &mut FrameBuffer);
    fn offset(&self, offset_by: Vector2D<f64>) -> Box<dyn Draw>;
    fn scale(&self, times: f64) -> Box<dyn Draw>;
}

// ----------------------------------------------------------------

#[derive()]
pub struct Scene {
    contents: Vec<Box<dyn Draw>>,
    res: Vector2D<u64>,
    offset: Vector2D<f64>,
    scale: f64,
    min_max_scale: Option<Vector2D<f64>>,
    base_scale: f64,
}

#[allow(dead_code)]
impl Scene {
    // Constructor
    pub fn new(
        contents: Vec<Box<dyn Draw>>,
        res: Vector2D<u64>,
        scale: f64,
        min_max_scale: Option<Vector2D<f64>>,
    ) -> Scene {
        Scene {
            contents,
            res,
            offset: Vector2D::new(0.0, 0.0),
            scale,
            min_max_scale,
            base_scale: (res.x as f64) / 500.0,
        }
    }

    // Immutable access
    pub fn contents(&self) -> &Vec<Box<dyn Draw>> {
        &self.contents
    }

    pub fn res(&self) -> &Vector2D<u64> {
        &self.res
    }

    pub fn offset(&self) -> &Vector2D<f64> {
        &self.offset
    }

    pub fn scale(&self) -> &f64 {
        &self.scale
    }

    pub fn min_max_scale(&self) -> &Option<Vector2D<f64>> {
        &self.min_max_scale
    }

    // Mutable access
    pub fn contents_mut(&mut self) -> &mut Vec<Box<dyn Draw>> {
        &mut self.contents
    }

    pub fn offset_mut(&mut self) -> &mut Vector2D<f64> {
        &mut self.offset
    }

    pub fn min_max_scale_mut(&mut self) -> &mut Option<Vector2D<f64>> {
        &mut self.min_max_scale
    }

    // Setters
    pub fn set_scale(&mut self, val: f64) {
        match self.min_max_scale {
            Some(x) => self.scale = val.clamp(x.x, x.y),
            None => self.scale = val,
        }
    }

    // Methods
    pub fn change_scale(&mut self, amount: f64) {
        // println!(
        //     "{:?}, {:?}, {:?}",
        //     self.res,
        //     self.base_scale,
        //     Vector2D::new(
        //         (self.res.x as f64) * self.base_scale,
        //         (self.res.y as f64) * self.base_scale
        //     )
        // );

        let scale_old = self.scale;
        self.set_scale(self.scale + amount);
        {
            let base_scale = self.base_scale;
            let scale = self.scale;
            let res = Vector2D::new(self.res.x as f64, self.res.y as f64);
            if self.scale - scale_old != 0.0 {
                self.offset -= Vector2D::new(
                    (res.x / (base_scale * scale_old)) - (res.x / (base_scale * scale)),
                    (res.y / (base_scale * scale_old)) - (res.y / (base_scale * scale)),
                );
            }
        }
    }

    pub fn draw(&self, frame_buffer: &mut FrameBuffer) {
        self.contents.iter().for_each(|shape| {
            shape
                .offset(self.offset)
                .scale(self.base_scale * self.scale)
                .draw(frame_buffer)
        });
    }

    pub fn to_frame_buffer(&self) -> FrameBuffer {
        let mut output = FrameBuffer::new(self.res);
        self.draw(&mut output);

        output
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
    size: Vector2D<u64>,
}

#[allow(dead_code)]
impl FrameBuffer {
    // Constructor
    pub fn new(size: Vector2D<u64>) -> FrameBuffer {
        FrameBuffer {
            buffer: vec![Color::new(0, 0, 0); (size.x * size.y) as usize],
            size,
        }
    }

    // Immutable access
    pub fn buffer(&self) -> &Vec<Color> {
        &self.buffer
    }

    pub fn size(&self) -> &Vector2D<u64> {
        &self.size
    }

    // Methods
    pub fn contains_point(&self, point: Vector2D<f64>) -> bool {
        point.x >= 0.0 && point.x < (self.size.x as f64) && point.y >= 0.0 && point.y < (self.size.y as f64)
    }

    pub fn set_pixel(&mut self, point: Vector2D<f64>, color: Color) {
        if self.contains_point(point) {
            let width = self.size.x as u64; // Dunno why I have to do that
            self.buffer[(((point.y as u64) * width) + (point.x as u64)) as usize] = color;
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
    r: u8,
    g: u8,
    b: u8,
}

#[allow(dead_code)]
impl Color {
    // Constructor
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    // Immutable access
    pub fn r(&self) -> &u8 {
        &self.r
    }

    pub fn g(&self) -> &u8 {
        &self.g
    }

    pub fn b(&self) -> &u8 {
        &self.b
    }

    // Mutable access
    pub fn r_mut(&mut self) -> &mut u8 {
        &mut self.r
    }

    pub fn g_mut(&mut self) -> &mut u8 {
        &mut self.g
    }

    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.b
    }

    // Methods
    pub fn to_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pos_1: Vector2D<f64>,
    pos_2: Vector2D<f64>,
    color: Color,
}

#[allow(dead_code)]
impl Line {
    // Constructor
    pub fn new(pos_1: Vector2D<f64>, pos_2: Vector2D<f64>, color: Color) -> Line {
        Line { pos_1, pos_2, color }
    }

    // Immutable access
    pub fn pos_1(&self) -> &Vector2D<f64> {
        &self.pos_1
    }

    pub fn pos_2(&self) -> &Vector2D<f64> {
        &self.pos_2
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    // Mutable access
    pub fn pos_1_mut(&mut self) -> &mut Vector2D<f64> {
        &mut self.pos_1
    }

    pub fn pos_2_mut(&mut self) -> &mut Vector2D<f64> {
        &mut self.pos_2
    }

    pub fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }
}

impl Draw for Line {
    fn draw(&self, frame_buffer: &mut FrameBuffer) {
        let (mut x0, mut y0) = (self.pos_1.x as i64, self.pos_1.y as i64);
        let (x1, y1) = (self.pos_2.x as i64, self.pos_2.y as i64);
        let (dx, dy) = ((x1 - x0).abs(), -(y1 - y0).abs());
        let (sx, sy) = (if x0 < x1 { 1 } else { -1 }, if y0 < y1 { 1 } else { -1 });
        let mut error = dx + dy;

        loop {
            frame_buffer.set_pixel(Vector2D::new(x0 as f64, y0 as f64), self.color);

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

    fn offset(&self, offset_by: Vector2D<f64>) -> Box<dyn Draw> {
        Box::new(Line::new(self.pos_1 + offset_by, self.pos_2 + offset_by, self.color))
    }

    fn scale(&self, times: f64) -> Box<dyn Draw> {
        Box::new(Line::new(
            Vector2D::new(self.pos_1.x * times, self.pos_1.y * times),
            Vector2D::new(self.pos_2.x * times, self.pos_2.y * times),
            self.color,
        ))
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Rect {
    pos: Vector2D<f64>,
    size: Vector2D<f64>,
    color: Color,
}

#[allow(dead_code)]
impl Rect {
    // Constructor
    pub fn new(pos: Vector2D<f64>, size: Vector2D<f64>, color: Color) -> Rect {
        Rect {
            pos,
            size: Vector2D::new(size.x.abs(), size.y.abs()),
            color,
        }
    }

    // Immutable access
    pub fn pos(&self) -> &Vector2D<f64> {
        &self.pos
    }

    pub fn size(&self) -> &Vector2D<f64> {
        &self.size
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    // Mutable access
    pub fn pos_mut(&mut self) -> &mut Vector2D<f64> {
        &mut self.pos
    }

    pub fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }

    // Setters
    pub fn set_size(&mut self, val: Vector2D<f64>) {
        self.size = Vector2D::new(val.x.abs(), val.y.abs())
    }
}

impl Draw for Rect {
    fn draw(&self, frame_buffer: &mut FrameBuffer) {
        // TODO: Check if on screen
        for y in 0..(self.size.y as usize) {
            for x in 0..(self.size.x as usize) {
                frame_buffer.set_pixel(
                    Vector2D::new((x as f64) + self.pos.x, (y as f64) + self.pos.y),
                    self.color,
                )
            }
        }
    }

    fn draw_outline(&self, frame_buffer: &mut FrameBuffer) {
        frame_buffer.draw(&Line::new(
            self.pos,
            Vector2D::new(self.pos.x + self.size.x, self.pos.y),
            self.color,
        ));
        frame_buffer.draw(&Line::new(
            self.pos,
            Vector2D::new(self.pos.x, self.pos.y + self.size.y),
            self.color,
        ));
        frame_buffer.draw(&Line::new(
            Vector2D::new(self.pos.x + self.size.x, self.pos.y),
            self.pos + self.size,
            self.color,
        ));
        frame_buffer.draw(&Line::new(
            Vector2D::new(self.pos.x, self.pos.y + self.size.y),
            self.pos + self.size,
            self.color,
        ));
    }

    fn offset(&self, offset_by: Vector2D<f64>) -> Box<dyn Draw> {
        Box::new(Rect::new(self.pos + offset_by, self.size, self.color))
    }

    fn scale(&self, times: f64) -> Box<dyn Draw> {
        let new_size = Vector2D::new(self.size.x * times, self.size.x * times);
        Box::new(Rect::new(
            Vector2D::new(self.pos.x * times, self.pos.y * times),
            new_size,
            self.color,
        ))
    }
}

// ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct Circle {
    pos: Vector2D<f64>,
    radius: f64,
    color: Color,
}

#[allow(dead_code)]
impl Circle {
    // Constructor
    pub fn new(pos: Vector2D<f64>, radius: f64, color: Color) -> Circle {
        Circle {
            pos,
            radius: radius.abs(),
            color,
        }
    }

    // Immutable access
    pub fn pos(&self) -> &Vector2D<f64> {
        &self.pos
    }

    pub fn radius(&self) -> &f64 {
        &self.radius
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    // Mutable access
    pub fn pos_mut(&mut self) -> &mut Vector2D<f64> {
        &mut self.pos
    }

    pub fn color_mut(&mut self) -> &mut Color {
        &mut self.color
    }

    // Setters
    pub fn set_radius(&mut self, val: f64) {
        self.radius = val.abs()
    }
}

impl Draw for Circle {
    fn draw(&self, frame_buffer: &mut FrameBuffer) {
        if self.pos.x + self.radius >= 0.0
            && self.pos.x - self.radius <= frame_buffer.size().x as f64
            && self.pos.y + self.radius >= 0.0
            && self.pos.y - self.radius <= frame_buffer.size().y as f64
        {
            for y in -(self.radius as isize)..(self.radius as isize) {
                for x in -(self.radius as isize)..(self.radius as isize) {
                    if (x.pow(2) + y.pow(2)) <= (self.radius as isize).pow(2) {
                        frame_buffer.set_pixel(
                            Vector2D::new(self.pos.x + (x as f64), self.pos.y + (y as f64)),
                            self.color,
                        )
                    }
                }
            }
        }
    }

    fn draw_outline(&self, frame_buffer: &mut FrameBuffer) {
        fn draw_circle(c: Vector2D<i64>, p: Vector2D<i64>, color: Color, frame_buffer: &mut FrameBuffer) {
            frame_buffer.set_pixel(Vector2D::new((c.x + p.x) as f64, (c.y + p.y) as f64), color);
            frame_buffer.set_pixel(Vector2D::new((c.x - p.x) as f64, (c.y + p.y) as f64), color);
            frame_buffer.set_pixel(Vector2D::new((c.x + p.x) as f64, (c.y - p.y) as f64), color);
            frame_buffer.set_pixel(Vector2D::new((c.x - p.x) as f64, (c.y - p.y) as f64), color);
            frame_buffer.set_pixel(Vector2D::new((c.x + p.y) as f64, (c.y + p.x) as f64), color);
            frame_buffer.set_pixel(Vector2D::new((c.x - p.y) as f64, (c.y + p.x) as f64), color);
            frame_buffer.set_pixel(Vector2D::new((c.x + p.y) as f64, (c.y - p.x) as f64), color);
            frame_buffer.set_pixel(Vector2D::new((c.x - p.y) as f64, (c.y - p.x) as f64), color);
        }

        if self.radius < 2.0 {
            frame_buffer.set_pixel(self.pos, self.color)
        } else {
            let center_pos: Vector2D<i64> = Vector2D::new(self.pos.x as i64, self.pos().y as i64);
            let mut p: Vector2D<i64> = Vector2D::new(0, self.radius as i64);
            let mut d: i64 = 3 - 2 * (self.radius as i64);

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

    fn offset(&self, offset_by: Vector2D<f64>) -> Box<dyn Draw> {
        Box::new(Circle::new(self.pos + offset_by, self.radius, self.color))
    }

    fn scale(&self, times: f64) -> Box<dyn Draw> {
        Box::new(Circle::new(
            Vector2D::new(self.pos.x * times, self.pos.y * times),
            self.radius * times,
            self.color,
        ))
    }
}
