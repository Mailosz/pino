use super::{point::Point, rect::Rect};

pub struct Bounds {
    l:f64,
    r:f64,
    t:f64,
    b:f64
}

impl Bounds {

    /**
     * Creates new Bounds
     * CAUTION: *r* must be > *l* and *b* must be > *t*
     */
    pub fn new(l:f64, t:f64, r:f64, b:f64) -> Bounds{
        Bounds{l:l, t:t, r:r, b:b}
    }

    pub fn new_safe(x1:f64,y1:f64,x2:f64,y2:f64) -> Bounds {
        if (x2 < x1) {
            if (y2 < y1) {
                Bounds{l:x2,t:y2,r:x1,b:y1}
            } else {
                Bounds{l:x2,t:y1,r:x1,b:y2}
            }
        } else {
            if (y2 < y1) {
                Bounds{l:x1,t:y2,r:x2,b:y1}
            } else {
                Bounds{l:x1,t:y1,r:x2,b:y2}
            }
        }
    }

    pub fn to_rect(&self) -> Rect {
        Rect::new(self.l ,self.t, self.r - self.l,  self.b - self.t)
    }

    pub fn containing_point(&self, p : &Point) -> Bounds {
        let mut l = self.l;
        let mut t = self.t;
        let mut r = self.r;
        let mut b = self.b;

        if (p.x < self.l) {
            l = p.x;
        } else if (p.x > self.r) {
            r = p.x;
        }

        if (p.y < self.t) {
            t = p.y;
        } else if (p.y > self.b) {
            b = p.y;
        }

        Bounds{l:l, t:t, r:r, b:b}
    }

    /**
     * Returns new Bounds extended if necessary to ensure that it contains both initial and passed bounds
     */
    pub fn containing_bounds(&self, bounds : &Bounds) -> Bounds {
        let mut l = self.l;
        let mut t = self.t;
        let mut r = self.r;
        let mut b = self.b;

        if (bounds.l < self.l) {
            l = bounds.l;
        }
        if (bounds.r > self.r) {
            r = bounds.r;
        }

        if (bounds.t < self.t) {
            t = bounds.t;
        }
        if (bounds.b > self.b) {
            b = bounds.b;
        }

        Bounds{l:l, t:t, r:r, b:b}
    }

    pub fn contains_point(&self, p : &Point) -> bool {
        p.x >= self.l && p.y >= self.t && p.x <= self.r && p.y <= self.b
    }

    pub fn contains_bounds(&self, bounds : &Bounds) -> bool {
        bounds.l >= self.l && bounds.t >= self.t && bounds.r <= self.r && bounds.b <= self.b
    }

    pub fn intersects_bounds(&self, bounds : &Bounds) -> bool {
        bounds.l >= self.l || bounds.t >= self.t || bounds.r <= self.r || bounds.b <= self.b
    }
}