use std::ops::Bound;

use wasm_bindgen::convert::ReturnWasmAbi;


pub struct Point{
    pub x:f64,
    pub y:f64
}

impl Point {
    fn x(&self) -> f64 {
        self.x
    }

    fn new(x:f64, y:f64) -> Point {
        Point{x:x,y:y}
    }

}



pub struct Bounds {
    l:f64,
    r:f64,
    t:f64,
    b:f64
}

impl Bounds {

    fn new(x1:f64,y1:f64,x2:f64,y2:f64) -> Bounds {
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

    fn to_rect(&self) -> Rect {
        Rect{x:self.l,y:self.t,w:self.r - self.l, h: self.b - self.t}
    }
}

pub struct Rect {
    x:f64,
    y:f64,
    w:f64,
    h:f64
}

impl Rect {
    /**
     * Creates new rect
     */
    fn new(x:f64,y:f64,w:f64,h:f64) -> Rect {
        if (w < 0.0) {panic!("Rect initialization: w less than 0");}
        if (h < 0.0) {panic!("Rect initialization: h less than 0");}
        Rect{x:x,y:y,w:w,h:h}
    }

    fn to_bounds(&self) -> Bounds {
        Bounds{l:self.x,t:self.y,r:self.x + self.w, b: self.y + self.h}
    }
}