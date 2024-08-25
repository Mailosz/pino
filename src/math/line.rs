use super::{bounds::Bounds, point::Point};

pub struct Line {
    a : Point,
    b : Point
}

impl Line {
    pub fn a(&self) {
        self.a
    }

    pub fn b(&self) {
        self.b
    }

    pub fn new(a : Point, b : Point) -> Line {
        Line{a:a, b:b}
    }

    pub fn point_on(&self, t : f64) {
        self.a + (self.b - this.a) * t
    }

    pub fn direction(&self) {
        self.a.direction_to(self.b)
    }

    pub fn get_bounds(&self) -> Bounds {
        let l;
        let r;
        if (self.b.x > self.a.x) {
            l = self.a.x;
            r = self.b.x;
        } else {
            l = self.b.x;
            r = self.a.x;
        }

        let t;
        let b;
        if (self.b.y > self.a.y) {
            t = self.a.y;
            b = self.b.y;
        } else {
            t = self.b.y;
            b = self.a.y;
        }

        Bounds::new(l, t, r, b)
    }
}