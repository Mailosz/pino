use std::primitive;

use shaders::{create_fragment_shader, create_shader_program, create_vertex_shader, ShaderInfo};
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use crate::base::*;

mod shaders;

pub struct Renderer {
    gl : WebGl2RenderingContext,
    program : WebGlProgram,
    primitives : Vec<Primitive>,
    shader_info : ShaderInfo,
}

impl Renderer {
    pub fn create(gl : WebGl2RenderingContext) -> Renderer{

        let (program, shader_info) = create_shader_program(&gl);
        gl.use_program(Some(&program));

        Renderer{
            gl,
            program : program,
            primitives : vec![Primitive{vertices : vec![10.0, 30.0, 100.0, 170.0, 30.0, 100.0, 100.0, 170.0, 100.0], fill: Brush::COLOR(0.2, 0.7, 0.5, 1.0)}],
            shader_info : shader_info
        }
    }


    pub fn set_vertices(&self, attribute : u32, vertices : &[f32], size : i32) {

        let buffer = self.gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        self.gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
            
            self.gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
        
        let vao = self.gl
            .create_vertex_array()
            .ok_or("Could not create vertex array object")
            .unwrap();
        self.gl.bind_vertex_array(Some(&vao));


        let position_attribute_location = 
        self.gl.vertex_attrib_pointer_with_i32(
            attribute,
            size,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.gl.enable_vertex_attrib_array(attribute);

        self.gl.bind_vertex_array(Some(&vao));

    }

    fn draw_primitive(&self, primitive : &Primitive) {
    
        self.set_vertices(self.shader_info.a_pos, &primitive.vertices, 3);
        self.set_brush(&primitive.fill);
        self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, (primitive.vertices.len() / 3) as i32);
    }

    pub fn resize_viewport(&self, width : f32, height : f32) {

        let resolution_attribute_location = self.gl.uniform2f(self.shader_info.u_res.as_ref(), width, height);

        self.gl.viewport(0,0, width as i32, height as i32);

        // self.width = width;
        // self.height = height;
    }

    fn set_brush(&self, brush : &Brush) {
        match brush {
            Brush::COLOR(r, g, b, a) => {
                self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 1);
                self.gl.uniform4f(self.shader_info.u_color.as_ref(), f32::to_owned(r), f32::to_owned(g), f32::to_owned(b), f32::to_owned(a));
            },
            Brush::LINEAR_GRADIENT(gradient) => {
                self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 2);

            }
        };
    }

    pub fn add_primitive(&mut self, primitive : Primitive) {
        self.primitives.push(primitive);
    }

}



pub struct Primitive {
    pub vertices : Vec<f32>,
    pub fill : Brush
}

pub struct Gradient {
    pub coords : Vec<f32>,
    pub colors : Vec<f32>
}

pub enum Brush {
    COLOR(f32, f32, f32, f32),
    LINEAR_GRADIENT(Gradient),
}


pub fn draw(renderer: &Renderer) {

    renderer.gl.clear_color(0.0, 0.0, 0.2, 1.0);
    renderer.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    for primitive in renderer.primitives.iter() {

        renderer.draw_primitive(primitive);
    }
}



