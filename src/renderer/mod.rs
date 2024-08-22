use std::primitive;

use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use crate::base::*;


pub struct Renderer {
    gl : WebGl2RenderingContext,
    program : WebGlProgram,
    width : f32,
    height : f32
}

impl Renderer {
    pub fn create(gl : WebGl2RenderingContext) -> Renderer{
        let vertex_shader_source = r##"#version 300 es
            // an attribute will receive data from a buffer
            in vec2 a_pos;
            uniform vec2 u_res;

            //out vec4 out_pos;

            // all shaders have a main function
            void main() {

                // convert the position from pixels to 0.0 to 1.0
                vec2 zeroToOne = a_pos / u_res;
            
                // convert from 0->1 to 0->2
                vec2 zeroToTwo = zeroToOne * 2.0;
            
                // convert from 0->2 to -1->+1 (clip space)
                vec2 clipSpace = zeroToTwo - 1.0;
            
                gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
            }
        "##;


        let fragemnt_shader_source = r##"#version 300 es
            // fragment shaders don't have a default precision so we need
            // to pick one. mediump is a good default
            precision highp float;

            uniform vec4 u_color;

            out vec4 out_color;

            void main() {
                // gl_FragColor is a special variable a fragment shader
                // is responsible for setting
                out_color = u_color;
            }
        "##;
    
        let vert_shader = compile_shader(
            &gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            vertex_shader_source,
        ).unwrap();
    
        let frag_shader = compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            fragemnt_shader_source
        ).unwrap();

        let program = link_program(&gl, &vert_shader, &frag_shader).unwrap();

        gl.use_program(Some(&program));

        let renderer = Renderer{
            gl,
            program : program,
            width : 2.0,
            height: 2.0
        };
    
        renderer
    }


    pub fn set_vertices(&self, vertices : &[f32]) {

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


        let position_attribute_location = self.gl.get_attrib_location(&self.program, "a_pos");
        self.gl.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.gl.enable_vertex_attrib_array(position_attribute_location as u32);

        self.gl.bind_vertex_array(Some(&vao));

    }

    fn draw_primitive(&self, primitive : &Primitive) {
    
        self.set_vertices(&primitive.vertices);
        self.set_brush_solid_color(0.2, 0.7, 0.5, 1.0);
        self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, (primitive.vertices.len() / 3) as i32);
    }

    pub fn resize_viewport(&self, width : f32, height : f32) {

        let u_res = self.gl.get_uniform_location(&self.program, "u_res");

        let resolution_attribute_location = self.gl.uniform2f(u_res.as_ref(), width, height);

        self.gl.viewport(0,0, width as i32, height as i32);

        // self.width = width;
        // self.height = height;
    }

    fn set_brush_solid_color(&self, r : f32, g : f32, b : f32, a : f32) {
        let u_color = self.gl.get_uniform_location(&self.program, "u_color");
        
        let color_attribute_location = self.gl.uniform4f(u_color.as_ref(), r, g, b, a);
    }

}




pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

struct Primitive {
    vertices : Vec<f32>,
}

pub fn draw(renderer: &Renderer) {

    log("Droooo");
    renderer.gl.clear_color(0.0, 0.0, 0.2, 1.0);
    renderer.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    
    let primitive = Primitive{
        vertices : vec![10.0, 30.0, 100.0, 170.0, 30.0, 100.0, 100.0, 170.0, 100.0],
    };

    renderer.draw_primitive(&primitive);
}



