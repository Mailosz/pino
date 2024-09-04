use super::{bounds::Bounds, point::Point};

struct QuadraticCurve {
    a : Point,
    cp : Point,
    b : Point
}

impl QuadraticCurve {

    /**
     * Returns x coord of the curve at specific t
     */
    pub fn get_x(&self, t : f64) -> f64 {
        self.a.x * f64::powi(1.0 - t, 2) + 2.0 * (1.0 - t) * t * self.cp.x + f64::powi(t,2) * self.b.x
    }

    /**
     * Returns y coord of the curve at specific t
     */
    pub fn get_y(&self, t : f64) -> f64 {
        self.a.y * f64::powi(1.0 - t, 2) + 2.0 * (1.0 - t) * t * self.cp.y + f64::powi(t,2) * self.b.y
    }

    /**
     * Get point on a curve
     * @param {Number} t position on a curve [0,1]
     */
    fn get_p(&self, t : f64) -> Point {
        Point::new(self.get_x(t), self.get_y(t))
    }

    fn get_bounds(&self) -> Bounds {
        // starting points
        let mut l;
        let mut r;
        if (self.b.x > self.a.x) {
            l = self.a.x;
            r = self.b.x;
        } else {
            l = self.b.x;
            r = self.a.x;
        }

        let mut t;
        let mut b;
        if (self.b.y > self.a.y) {
            t = self.a.y;
            b = self.b.y;
        } else {
            t = self.b.y;
            b = self.a.y;
        }

        // points on curve


        let x1 = (self.cp.x - self.a.x);
        let x2 = (self.b.x - self.cp.x);
        let tx = -x1/(x2 - x1);
        if (!f64::is_nan(tx) && tx > 0.0 && tx < 1.0) {
            let x = self.get_x(tx);
            if (x < l) {
                l = x;
            } else if (x > r) {
                r = x;
            }
        }

        let y1 = (self.cp.y - self.a.y);
        let y2 = (self.b.y - self.cp.y);
        let ty = -y1 / (y2 - y1);
        if (!f64::is_nan(ty) && ty > 0.0 && ty < 1.0) {
            let y = self.get_y(ty);
            if (y < t) {
                t = y;
            } else if (y > b) {
                b = y;
            }
        }


        Bounds::new_fast(l, t, r, b)
    }
}



struct BezierCurve {
    a : Point,
    c1 : Point,
    c2 : Point,
    b : Point
}