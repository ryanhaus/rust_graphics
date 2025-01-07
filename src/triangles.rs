use std::{cmp, ops::Range};

#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn translated_by(&self, offset: Point2D) -> Self {
        Point2D::new(self.x + offset.x, self.y + offset.y)
    }
}

#[derive(Clone, Copy, Debug)]
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
    fn edge_function(a: Point2D, b: Point2D, c: Point2D) -> f64 {
        (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
    }

    pub fn signed_area(&self) -> f64 {
        2.0 * Triangle2D::edge_function(self.a, self.b, self.c)
    }

    // see https://jtsorlinis.github.io/rendering-tutorial/
    // checks whether or not a point is inside the triangle
    pub fn contains_point(&self, p: Point2D) -> bool {
        let (weight_a, weight_b, weight_c) = self.get_weights_at(p);

        let area = 0.5 * Triangle2D::edge_function(self.a, self.b, self.c);
        weight_a >= 0.0 && weight_b >= 0.0 && weight_c >= 0.0
    }

    // gets the 'weights' of each point (a,b,c) at a given point
    pub fn get_weights_at(&self, p: Point2D) -> (f64, f64, f64) {
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
    pub fn get_bounding_box(&self) -> (Range<f64>, Range<f64>) {
        let min_x = f64::min(1.0, f64::min(f64::min(self.a.x, self.b.x), self.c.x));
        let max_x = f64::max(0.0, f64::max(f64::max(self.a.x, self.b.x), self.c.x));
        let min_y = f64::min(1.0, f64::min(f64::min(self.a.y, self.b.y), self.c.y));
        let max_y = f64::max(0.0, f64::max(f64::max(self.a.y, self.b.y), self.c.y));

        (min_x..max_x, min_y..max_y)
    }

    // returns two Ranges indicating the 'bounding box' of the triangle in pixels
    pub fn get_bounding_box_px(&self, width: u32, height: u32) -> (Range<u32>, Range<u32>) {
        let (x_range, y_range) = self.get_bounding_box();

        let min_x = (x_range.start * width as f64).floor() as u32;
        let max_x = (x_range.end * width as f64).ceil() as u32;
        let min_y = (y_range.start * height as f64).floor() as u32;
        let max_y = (y_range.end * height as f64).ceil() as u32;
        
        (min_x..max_x, min_y..max_y)
    }

    // paints the triangle into a PaintBuffer object
    pub fn paint_to_buffer(&self, buffer: &mut PaintBuffer, paint_value: u32) {
        // don't even bother with back-facing triangles
        if self.signed_area() <= 0.0 {
            return;
        }

        // get bounding box of triangle in this buffer
        let (range_x, range_y) = self.get_bounding_box_px(buffer.width, buffer.height);

        // paint all points in the triangle
        for y in range_y {
            for x in range_x.clone() {
                let index = (x + y * buffer.width) as usize;
                let x = (x as f64) / (buffer.width as f64);
                let y = (y as f64) / (buffer.height as f64);
                let p = Point2D::new(x, y);

                buffer.pixel_buffer[index] = if self.contains_point(p) { paint_value } else { 0x000000 };
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
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

    pub fn normalized(&self) -> Self {
        let magnitude = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();

        Point3D::new(self.x / magnitude, self.y / magnitude, self.z / magnitude)
    }

    pub fn dot(&self, p: Point3D) -> f64 {
        self.x * p.x + self.y * p.y + self.z * p.z
    }

    pub fn get_translating_point(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }

    pub fn rotated_xz(&self, rotation: f64) -> Self {
        let magnitude = (self.x.powf(2.0) + self.z.powf(2.0)).sqrt();
        let theta = self.z.atan2(self.x) + rotation;

        Point3D::new(
            magnitude * theta.cos(),
            self.y,
            magnitude * theta.sin(),
        )
    }
}

#[derive(Clone, Copy, Debug)]
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

    pub fn paint_to_buffer<ColorF: Fn(f64, f64, f64) -> u32>(&self, buffer: &mut PaintBuffer, scene: Scene, color_f: ColorF) {
        let Scene(camera, light) = scene;
        let mut translated_triangle = self.translated_by(camera.position.get_translating_point());
        translated_triangle.a.y *= -1.0;
        translated_triangle.b.y *= -1.0;
        translated_triangle.c.y *= -1.0;
        let projected_triangle = translated_triangle.project_to_2d();
        let projected_triangle = projected_triangle.translated_by(Point2D::new(0.5, 0.5));

        // don't even bother with back-facing triangles
        if projected_triangle.signed_area() <= 0.0 {
            return;
        }

        let (range_x, range_y) = projected_triangle.get_bounding_box_px(buffer.width, buffer.height);

        for y in range_y {
            for x in range_x.clone() {
                let index = (x + y * buffer.width) as usize;

                if index >= buffer.pixel_buffer.len() {
                    continue;
                }

                let x = (x as f64) / (buffer.width as f64);
                let y = (y as f64) / (buffer.height as f64);
                let p = Point2D::new(x, y);

                if projected_triangle.contains_point(p) {
                    let (weight_a, weight_b, weight_c) = projected_triangle.get_weights_at(p);
                    let z_val = self.a.z * weight_a + self.b.z * weight_b + self.c.z * weight_c;

                    if z_val < buffer.z_buffer[index] {
                        buffer.z_buffer[index] = z_val;
                        buffer.pixel_buffer[index] = color_f(weight_a, weight_b, weight_c);
                    }
                }
            }
        }
    }

    pub fn rotated_xz(&self, rotation: f64) -> Self {
        Self {
            a: self.a.rotated_xz(rotation),
            b: self.b.rotated_xz(rotation),
            c: self.c.rotated_xz(rotation),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ColorTriangle {
    pub color: u32,
    pub tri: Triangle3D,
    pub normal_tri: Triangle3D,
}

impl ColorTriangle {
    pub fn new(color: u32, tri: Triangle3D, normal_tri: Triangle3D) -> Self {
        ColorTriangle { color, tri, normal_tri }
    }

    pub fn paint_to_buffer(&self, buffer: &mut PaintBuffer, scene: Scene) {
        let Scene(camera, light) = scene;

        let light_dir_a = Point3D::new(
            -self.tri.a.x + light.position.x,
            -self.tri.a.y + light.position.y,
            -self.tri.a.z + light.position.z,
        ).normalized();

        let light_dir_b = Point3D::new(
            -self.tri.b.x + light.position.x,
            -self.tri.b.y + light.position.y,
            -self.tri.b.z + light.position.z,
        ).normalized();

        let light_dir_c = Point3D::new(
            -self.tri.c.x + light.position.x,
            -self.tri.c.y + light.position.y,
            -self.tri.c.z + light.position.z,
        ).normalized();
        
        let diff_brightness_a = light_dir_a.x * self.normal_tri.a.x + light_dir_a.y * self.normal_tri.a.y + light_dir_a.z * self.normal_tri.a.z;
        let diff_brightness_b = light_dir_b.x * self.normal_tri.b.x + light_dir_b.y * self.normal_tri.b.y + light_dir_b.z * self.normal_tri.b.z;
        let diff_brightness_c = light_dir_c.x * self.normal_tri.c.x + light_dir_c.y * self.normal_tri.c.y + light_dir_c.z * self.normal_tri.c.z;

        let halfway_dir_a = light_dir_a.translated_by(camera.view_dir).normalized();
        let halfway_dir_b = light_dir_b.translated_by(camera.view_dir).normalized();
        let halfway_dir_c = light_dir_c.translated_by(camera.view_dir).normalized();

        let spec_constant = 4.0;
        let spec_brightness_a = f64::max(self.normal_tri.a.dot(halfway_dir_a), 0.0).powf(spec_constant);
        let spec_brightness_b = f64::max(self.normal_tri.b.dot(halfway_dir_b), 0.0).powf(spec_constant);
        let spec_brightness_c = f64::max(self.normal_tri.c.dot(halfway_dir_c), 0.0).powf(spec_constant);

        self.tri.paint_to_buffer(buffer, scene, |weight_a, weight_b, weight_c| {

            let mut brightness = 0.15; // ambient
            brightness += diff_brightness_a * weight_a + diff_brightness_b * weight_b + diff_brightness_c * weight_c; // diffuse
            brightness += spec_brightness_a * weight_a + spec_brightness_b * weight_b + spec_brightness_c * weight_c; // specular
            brightness = f64::clamp(brightness, 0.0, 1.0);

            let brightness_r = brightness * light.color.0;
            let brightness_g = brightness * light.color.1;
            let brightness_b = brightness * light.color.2;

            let r = (255.0 * brightness_r) as u32;
            let g = (255.0 * brightness_g) as u32;
            let b = (255.0 * brightness_b) as u32;

            (r << 16) | (g << 8) | b
        });
    }

    pub fn translated_by(&self, offset: Point3D) -> Self {
        Self {
            tri: self.tri.translated_by(offset),
            normal_tri: self.normal_tri,
            color: self.color,
        }
    }
}

pub struct PaintBuffer {
    pub width: u32,
    pub height: u32,
    pub z_buffer: Vec<f64>,
    pub pixel_buffer: Vec<u32>,
}

impl PaintBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let buffer_size = (width * height) as usize;

        Self {
            width,
            height,
            z_buffer: vec![f64::MAX; buffer_size],
            pixel_buffer: vec![0; buffer_size],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub position: Point3D,
    pub view_dir: Point3D,
}

impl Camera {
    pub fn new(position: Point3D, view_dir: Point3D) -> Self {
        Self { position, view_dir }
    }

}

#[derive(Clone, Copy, Debug)]
pub struct Light {
    pub position: Point3D,
    pub color: (f64, f64, f64),
}

impl Light {
    pub fn new(position: Point3D, color: (f64, f64, f64)) -> Self {
        Self { position, color }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Scene(Camera, Light);

impl Scene {
    pub fn new(camera: Camera, light: Light) -> Self {
        Self(camera, light)
    }
}

pub struct Object3D {
    pub position: Point3D,
    pub rotation: f64,
    pub triangles: Vec<ColorTriangle>,
}

impl Object3D {
    pub fn new(triangles: Vec<ColorTriangle>) -> Self {
        Self {
            position: Point3D::new(0.0, 0.0, 0.0),
            rotation: 0.0,
            triangles
        }
    }

    pub fn paint_to_buffer(&self, buffer: &mut PaintBuffer, scene: Scene) {
        for tri in &self.triangles {
            let mut tri = tri.clone();
            tri.tri = tri.tri.rotated_xz(self.rotation);
            tri.normal_tri = tri.normal_tri.rotated_xz(self.rotation);
            tri.tri = tri.tri.translated_by(self.position.get_translating_point());
            tri.paint_to_buffer(buffer, scene);
        }
    }
}
