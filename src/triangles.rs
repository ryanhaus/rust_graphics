use std::{cmp, ops::Range};

#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Triangle2D {
    pub a: Point2D,
    pub b: Point2D,
    pub c: Point2D,
}

impl Triangle2D {
    pub fn new(a: Point2D, b: Point2D, c: Point2D) -> Self {
        Self { a, b, c }
    }

    // see https://jtsorlinis.github.io/rendering-tutorial/
    // if the edge function value is positive, the triangle vertices are
    // clockwise. otherwise, they are counterclockwise
    fn edge_function(a: Point2D, b: Point2D, c: Point2D) -> f32 {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }

    // see https://jtsorlinis.github.io/rendering-tutorial/
    // checks whether or not a point is inside the triangle
    pub fn contains_point(&self, p: Point2D) -> bool {
        let abp = Triangle2D::edge_function(self.a, self.b, p);
        let bcp = Triangle2D::edge_function(self.b, self.c, p);
        let cap = Triangle2D::edge_function(self.c, self.a, p);

        abp.is_sign_positive() && bcp.is_sign_positive() && cap.is_sign_positive()
    }

    // returns two Ranges indicating the 'bounding box' of the triangle
    pub fn get_bounding_box(&self) -> (Range<f32>, Range<f32>) {
        let min_x = f32::min(1.0, f32::min(f32::min(self.a.x, self.b.x), self.c.x));
        let max_x = f32::max(0.0, f32::max(f32::max(self.a.x, self.b.x), self.c.x));
        let min_y = f32::min(0.0, f32::min(f32::min(self.a.y, self.b.y), self.c.y));
        let max_y = f32::max(0.0, f32::max(f32::max(self.a.y, self.b.y), self.c.y));

        (min_x..max_x, min_y..max_y)
    }

    // returns two Ranges indicating the 'bounding box' of the triangle in pixels
    pub fn get_bounding_box_px(&self, width: u32, height: u32) -> (Range<u32>, Range<u32>) {
        let (x_range, y_range) = self.get_bounding_box();

        let min_x = (x_range.start * (width as f32)) as u32;
        let max_x = (x_range.end * (width as f32)) as u32;
        let min_y = (y_range.start * (height as f32)) as u32;
        let max_y = (y_range.end * (height as f32)) as u32;
        
        (min_x..max_x, min_y..max_y)
    }

    // paints the triangle into a PaintBuffer object
    pub fn paint_to_buffer(&self, buffer: &mut PaintBuffer, paint_value: u32) {
        // get bounding box of triangle in this buffer
        let (range_x, range_y) = self.get_bounding_box_px(buffer.width, buffer.height);

        // paint all points in the triangle
        for y in range_y {
            for x in range_x.clone() {
                let index = (x + y * buffer.width) as usize;
                let x = (x as f32) / (buffer.width as f32);
                let y = (y as f32) / (buffer.height as f32);
                let p = Point2D::new(x, y);

                buffer.pixel_buffer[index] = if self.contains_point(p) { paint_value } else { 0x000000 };
            }
        }
    }
}

pub struct PaintBuffer {
    pub width: u32,
    pub height: u32,
    pub pixel_buffer: Vec<u32>,
}

impl PaintBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer_size = (width * height) as usize;

        Self {
            width,
            height,
            pixel_buffer: vec![0; buffer_size],
        }
    }
}
