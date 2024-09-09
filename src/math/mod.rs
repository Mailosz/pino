use std::fmt::Display;
use std::ops::{self, Bound};

use wasm_bindgen::prelude::*;
use wasm_bindgen::convert::ReturnWasmAbi;
use num::Float;


pub mod bounds;
pub mod rect;
pub mod line;
pub mod point;
pub mod curves;
pub mod matrix;


#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
    Clockwise, CounterClockwise, Colinear
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Clockwise => write!(f, "Clockwise"),
            Orientation::CounterClockwise => write!(f, "CounterClockwise"),
            Orientation::Colinear => write!(f, "Colinear"),
        }
    }
}