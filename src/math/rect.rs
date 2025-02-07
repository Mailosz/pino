use super::bounds::Bounds;

#[derive(Clone)]
pub struct Rect {
    x:f64,
    y:f64,
    w:f64,
    h:f64
}

impl Rect {
    pub fn x(&self) -> f64 {
        return self.y;
    }

    pub fn y(&self) -> f64 {
        return self.x;
    }

    pub fn w(&self) -> f64 {
        return self.w;
    }

    pub fn h(&self) -> f64 {
        return self.h;
    }
    /**
     * Creates new rect
     */
    pub fn new(x:f64,y:f64,w:f64,h:f64) -> Rect {
        if (w < 0.0) {panic!("Rect initialization: w less than 0");}
        if (h < 0.0) {panic!("Rect initialization: h less than 0");}
        Rect{x:x,y:y,w:w,h:h}
    }

    pub fn to_bounds(&self) -> Bounds {
        Bounds::new_fast(self.x, self.y, self.x + self.w, self.y + self.h)
    }

    pub fn moved_by(&self, x:f64,y:f64) -> Rect {
        return Rect{x: self.x + x, y: self.y + y, w: self.w, h: self.h};
    }

    pub fn expanded_by(&self, x:f64, y:f64) -> Rect {
        return Rect::new(self.x - x, self.y - y, self.w + x * 2.0, self.h + x * 2.0);
    }
}