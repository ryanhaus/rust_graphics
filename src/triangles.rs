#[derive(Clone, Copy, Debug)]
pub struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
pub struct Triangle2D {
    a: Point2D,
    b: Point2D,
    c: Point2D,
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
}
