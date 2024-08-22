use std::collections::HashMap;

use base::log;
use once_cell::*;
use renderer::{draw, Renderer};
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

    unsafe {
        let option = CONTEXTS.get(canvas_id);
        match option {
            Some(context) => {
                draw(&context.renderer);
            },
            None => ()
        }
    }


    Ok(())
}