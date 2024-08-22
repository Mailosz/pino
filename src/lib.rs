use std::{collections::HashMap, primitive};

use base::log;
use once_cell::*;
use renderer::{draw, Gradient, Primitive, Renderer};
use sync::Lazy;
use wasm_bindgen::prelude::*;
use web_sys::{Event, WebGl2RenderingContext};
use crate::math::*;
extern crate console_error_panic_hook;

mod base;
mod math;
mod data;
mod renderer;


struct Context {
    id : String,
    renderer : Renderer,
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
        renderer : renderer,
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
pub fn add_primitive(canvas_id : &str, vertices : Vec<f32>, r:f32, g:f32, b:f32, a:f32) {
    let context = get_context(canvas_id);

    
    let primitive = Primitive{
        vertices : vertices,
        fill: renderer::Brush::COLOR(r, g, b, a)
    };

    context.renderer.add_primitive(primitive);
}

#[wasm_bindgen]
pub fn add_primitive_gradient(canvas_id : &str, vertices : Vec<f32>, x1:f32, y1:f32, r1:f32, g1:f32, b1:f32, a1:f32, x2:f32, y2:f32, r2:f32, g2:f32, b2:f32, a2:f32) {
    let context = get_context(canvas_id);

    
    let primitive = Primitive{
        vertices : vertices,
        fill: renderer::Brush::LINEAR_GRADIENT(Gradient{
            coords : vec![x1, y1, x2, y2],
            colors : vec![r1, g1, b1, a1, r2, g2, b2, a2],
        })
    };

    context.renderer.add_primitive(primitive);
}