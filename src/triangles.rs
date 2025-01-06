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

    pub fn translated_by(&self, offset: Point2D) -> Self {
        Point2D::new(self.x + offset.x, self.y + offset.y)
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

    pub fn translated_by(&self, offset: Point2D) -> Self {
        Self {
            a: self.a.translated_by(offset),
            b: self.b.translated_by(offset),
            c: self.c.translated_by(offset),
        }
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

    // gets the 'weights' of each point (a,b,c) at a given point
    pub fn get_weights_at(&self, p: Point2D) -> (f32, f32, f32) {
        let abc = Triangle2D::edge_function(self.a, self.b, self.c);
        let abp = Triangle2D::edge_function(self.a, self.b, p);
        let bcp = Triangle2D::edge_function(self.b, self.c, p);
        let cap = Triangle2D::edge_function(self.c, self.a, p);

        let weight_a = bcp / abc;
        let weight_b = cap / abc;
        let weight_c = abp / abc;

        (weight_a, weight_b, weight_c)
    }

    // returns two Ranges indicating the 'bounding box' of the triangle
    pub fn get_bounding_box(&self) -> (Range<f32>, Range<f32>) {
        let min_x = f32::min(1.0, f32::min(f32::min(self.a.x, self.b.x), self.c.x));
        let max_x = f32::max(0.0, f32::max(f32::max(self.a.x, self.b.x), self.c.x));
        let min_y = f32::min(1.0, f32::min(f32::min(self.a.y, self.b.y), self.c.y));
        let max_y = f32::max(0.0, f32::max(f32::max(self.a.y, self.b.y), self.c.y));

        (min_x..max_x, min_y..max_y)
    }

    // returns two Ranges indicating the 'bounding box' of the triangle in pixels
    pub fn get_bounding_box_px(&self, width: u32, height: u32) -> (Range<u32>, Range<u32>) {
        let a_x_px = (self.a.x * (width as f32)) as u32;
        let a_y_px = (self.a.y * (height as f32)) as u32;
        let b_x_px = (self.b.x * (width as f32)) as u32;
        let b_y_px = (self.b.y * (height as f32)) as u32;
        let c_x_px = (self.c.x * (width as f32)) as u32;
        let c_y_px = (self.c.y * (height as f32)) as u32;
        
        let min_x = cmp::min(width - 1, cmp::min(a_x_px, cmp::min(b_x_px, c_x_px)));
        let max_x = cmp::max(0, cmp::max(a_x_px, cmp::max(b_x_px, c_x_px)));
        let min_y = cmp::min(height - 1, cmp::min(a_y_px, cmp::min(b_y_px, c_y_px)));
        let max_y = cmp::max(0, cmp::max(a_y_px, cmp::max(b_y_px, c_y_px)));
        
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

#[derive(Clone, Copy, Debug)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn translated_by(&self, offset: Point3D) -> Self {
        Point3D::new(self.x + offset.x, self.y + offset.y, self.z + offset.z)
    }

    pub fn project_to_2d(&self) -> Point2D {
        // TODO: use FOV
        Point2D::new(
            self.x / self.z,
            self.y / self.z,
        )
    }
}

#[derive(Debug)]
pub struct Triangle3D {
    pub a: Point3D,
    pub b: Point3D,
    pub c: Point3D,
}

impl Triangle3D {
    pub fn new(a: Point3D, b: Point3D, c: Point3D) -> Self {
        Triangle3D { a, b, c }
    }

    pub fn translated_by(&self, offset: Point3D) -> Self {
        Self {
            a: self.a.translated_by(offset),
            b: self.b.translated_by(offset),
            c: self.c.translated_by(offset),
        }
    }

    pub fn project_to_2d(&self) -> Triangle2D {
        Triangle2D::new(
            self.a.project_to_2d(),
            self.b.project_to_2d(),
            self.c.project_to_2d(),
        )
    }

    pub fn paint_to_buffer(&self, buffer: &mut PaintBuffer, camera: Camera, paint_value: u32) {
        let translated_triangle = self.translated_by(camera.get_translating_point());
        let projected_triangle = translated_triangle.project_to_2d();
        let projected_triangle = projected_triangle.translated_by(Point2D::new(0.5, 0.5));
        let (range_x, range_y) = projected_triangle.get_bounding_box_px(buffer.width, buffer.height);


        for y in range_y {
            for x in range_x.clone() {
                let index = (x + y * buffer.width) as usize;

                let x = (x as f32) / (buffer.width as f32);
                let y = (y as f32) / (buffer.height as f32);
                let p = Point2D::new(x, y);

                if projected_triangle.contains_point(p) {
                    let (weight_a, weight_b, weight_c) = projected_triangle.get_weights_at(p);
                    let z_val = self.a.z * weight_a + self.b.z * weight_b + self.c.z * weight_c;

                    if z_val < buffer.z_buffer[index] {
                        buffer.z_buffer[index] = z_val;
                        buffer.pixel_buffer[index] = paint_value;
                    }
                }
            }
        }
    }
}

pub struct PaintBuffer {
    pub width: u32,
    pub height: u32,
    pub z_buffer: Vec<f32>,
    pub pixel_buffer: Vec<u32>,
}

impl PaintBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer_size = (width * height) as usize;

        Self {
            width,
            height,
            z_buffer: vec![f32::MAX; buffer_size],
            pixel_buffer: vec![0; buffer_size],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    position: Point3D,
    // TODO: make orientation of camera as well as FOV etc
}

impl Camera {
    pub fn new(position: Point3D) -> Self {
        Self { position }
    }

    pub fn get_translating_point(&self) -> Point3D {
        Point3D::new(-self.position.x, -self.position.y, -self.position.z)
    }
}
