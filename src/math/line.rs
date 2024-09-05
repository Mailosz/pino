use std::{cmp, ops};

use super::{bounds::Bounds, point::{dot_product, Point}};


pub struct Line {
    a : Point,
    b : Point
}

impl Line {
    pub fn a(&self) -> Point {
        self.a
    }

    pub fn b(&self) -> Point {
        self.b
    }

    pub fn new(a : Point, b : Point) -> Line {
        Line{a:a, b:b}
    }

    /**
     * Returns whether the line is of 0 length, meaning that a = b
     */
    pub fn is_point(&self) -> bool {
        self.a == self.b
    }

    pub fn point_on(&self, t : f64) -> Point {
        self.a + (self.b - self.a) * t
    }

    pub fn direction(&self) -> f64 {
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

        Bounds::new_fast(l, t, r, b)
    }


}

    /**
     * Sign of the returned value defines if point is above/on the left the line (+) or below/on the right (-)
     */
    pub fn line_side(a : Point, b : Point, p : Point) -> f64 {
        (b.x - a.x)*(p.y - a.y) - (b.y - a.y)*(p.x - a.x)
    }

    /**
     * Gets t value of a point projected onto a line
     */
    pub fn get_point_on_line_projection(a : Point, b : Point, p : Point) -> f64 {
        let len = (b.x() - a.x()) * (b.x() - a.x()) + (b.y() - a.y()) * (b.y() - a.y()); 

        f64::max(0.0, f64::min(1.0, dot_product(&(p - a), &(b - a)) / len))
    }

    pub fn point_on_line(a : Point, b : Point, t : f64) -> Point {
        (a + t) * (b - a)
    }

    pub fn distance_to_line(a : Point, b : Point, p : Point) -> f64 {
        if (a == b) {
            a.distance(&p)
        } else {
            // length squared
            let t = get_point_on_line_projection(a, b, p);
             // Projection falls on the segment
            p.distance(&point_on_line(a, b, t))
        }
    }

    pub fn do_lines_intersect() -> bool {
        true
    }