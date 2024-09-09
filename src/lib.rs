#![allow(warnings)]
use std::{collections::HashMap, f32::consts::PI, primitive};

use base::log;
use data::{Document};
use js_sys::Math::atan2;
use matrix::Matrix3x3;
use num::iter;
use once_cell::*;
use renderer::{draw, tesselation::{normalize_polygon, tesselate_polygon}, Brush, Gradient, GradientStop, Polygon, Primitive, Renderer, Triangles, TrianglesMode, P};
use sync::Lazy;
use wasm_bindgen::prelude::*;
use web_sys::{console::{time_end_with_label, time_with_label}, Event, WebGl2RenderingContext};
use crate::math::*;
use std::fmt::Write; // for write!
extern crate console_error_panic_hook;

mod base;
mod math;
mod data;
mod renderer;


struct Context {
    id : String,
    brush : Brush,
    renderer : Renderer,
    document : Document,
    canvas_element : web_sys::HtmlCanvasElement
}

static mut CONTEXTS : Lazy<HashMap<String, Context>> = Lazy::new(||{
    HashMap::new()
});


#[wasm_bindgen]
pub fn initialize(canvas_id : &str) -> Result<(), JsValue> {

    console_error_panic_hook::set_once();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(canvas_id).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;
    
    let renderer = Renderer::create(context);

    let context = Context{
        id : canvas_id.to_string(),
        brush : renderer::Brush::Color(1.0, 1.0, 1.0, 1.0),
        renderer : renderer,
        document : Document::new(),
        canvas_element : canvas
    };

    unsafe {
        CONTEXTS.insert(canvas_id.to_string(), context);
    }


    Ok(())
}

#[wasm_bindgen]
pub fn resize(canvas_id : &str, width : u32, height : u32) -> Result<(), JsValue> {

    unsafe {
        let option = CONTEXTS.get(canvas_id);

        match option {
            Some(context) => {
                context.renderer.resize_viewport(width as f32, height as f32);

                context.canvas_element.set_width(width);
                context.canvas_element.set_height(height);
            },
            None => ()
        }

    }


    Ok(())
}

#[wasm_bindgen]
pub fn redraw(canvas_id : &str) -> Result<(), JsValue> {

    draw(&get_context(canvas_id).renderer);

    Ok(())
}

fn get_context(canvas_id : &str) -> &mut Context {
    let option: Option<&mut Context>;
    unsafe {
        option = CONTEXTS.get_mut(canvas_id);
    }
    match option {
        Some(context) => {
            context
        },
        None => {panic!("No context with name {}", canvas_id)}
    }
}


#[wasm_bindgen]
pub struct JsPoint {
    pub x : f64,
    pub y : f64
}


#[wasm_bindgen]
pub fn set_solid_color_brush(canvas_id : &str, r:f32, g:f32, b:f32, a:f32) {

    let brush = renderer::Brush::Color(r, g, b, a);

    change_brush(canvas_id, brush);
    
}

pub fn change_brush(canvas_id : &str, brush : Brush) {
    let context = get_context(canvas_id);

    context.brush = brush;
}




#[wasm_bindgen]
pub fn set_linear_gradient(canvas_id : &str, x1:f32, y1:f32, x2:f32, y2:f32, stops:Vec<f32>) {
    let context = get_context(canvas_id);

    let gradient_stops: Vec<GradientStop> = get_gradient_stops(stops);
    
    let brush = renderer::Brush::LinearGradient(Gradient{
        x1 : x1,
        y1 : y1,
        x2 : x2,
        y2 : y2,
        stops : gradient_stops
    });


    change_brush(canvas_id, brush)
}

pub fn get_gradient_stops(stops:Vec<f32>) -> Vec<GradientStop>  {
    let mut gradient_stops = Vec::new();
    for i in (0..stops.len()).skip(4).step_by(5) {
        gradient_stops.push(GradientStop{
            position : stops[i-4],
            r : stops[i-3],
            g : stops[i-2],
            b : stops[i-1],
            a : stops[i-0],
        });
    }
    return gradient_stops;
}

#[wasm_bindgen]
pub fn set_radial_gradient(canvas_id : &str, x1:f32, y1:f32, x2:f32, y2:f32, stops:Vec<f32>) {
    let context = get_context(canvas_id);

    let gradient_stops: Vec<GradientStop> = get_gradient_stops(stops);
    
    let brush = renderer::Brush::RadialGradient(Gradient{
        x1 : x1,
        y1 : y1,
        x2 : x2,
        y2 : y2,
        stops : gradient_stops
    });


    change_brush(canvas_id, brush)
}

#[wasm_bindgen]
pub fn set_conic_gradient(canvas_id : &str, x1:f32, y1:f32, x2:f32, y2:f32, stops:Vec<f32>) {
    let context = get_context(canvas_id);

    let gradient_stops: Vec<GradientStop> = get_gradient_stops(stops);
    
    let brush = renderer::Brush::ConicGradient(Gradient{
        x1 : x1,
        y1 : y1,
        x2 : x2,
        y2 : y2,
        stops : gradient_stops
    });


    change_brush(canvas_id, brush)
}



#[wasm_bindgen]
pub fn add_polygon(canvas_id : &str, orientation : &str, points : Vec<f32>) {
    let context: &mut Context = get_context(canvas_id);

    let pua = points.iter().step_by(2).zip(points.iter().skip(1).step_by(2)).map(|(a,b)| P::new(*a, *b));

    let o;
    if (orientation == "clockwise") {
        o = Orientation::Clockwise
    } else if (orientation == "counter-clockwise") {
        o = Orientation::CounterClockwise
    } else {
        o = Orientation::Colinear;
        panic!("Wrong orientation");
    }

    let mut polygon = Polygon{points: pua.collect(), orientation : o};

    time_with_label("Normalization time");
    normalize_polygon(&mut polygon);
    time_end_with_label("Normalization time");

    // print polygon points
    // let mut str = String::new();
    // for p in &polygon.points {
    //      write!(&mut str, "({},{}) ",p.x(), p.y());
    // }
   
    // log(&str);

    time_with_label("Tesselation time");
    let strips = tesselate_polygon(&polygon);
    time_end_with_label("Tesselation time");

    let primitive = Primitive{parts : strips, fill : context.brush.clone()};

    context.renderer.add_primitive(primitive);
}


#[wasm_bindgen]
pub fn set_transform(canvas_id : &str, c11 : f32, c12 : f32, c13 : f32, c21 : f32, c22 : f32, c23 : f32, c31 : f32, c32 : f32, c33 : f32) {
    let context: &mut Context = get_context(canvas_id);

    let transform_matrix = Matrix3x3::new(c11, c12, c13, c21, c22, c23, c31, c32, c33);

    context.renderer.set_transform(transform_matrix);
    
}