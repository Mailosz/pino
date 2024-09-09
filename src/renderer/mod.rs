use std::{borrow::Borrow, f32::consts::PI, primitive, time::Instant};

use shaders::{ create_shader_program, ShaderInfo};
use wasm_bindgen::prelude::*;
use web_sys::{console::{time_end_with_label, time_with_label}, WebGl2RenderingContext, WebGlProgram, WebGlShader};
use crate::{base::*, matrix::Matrix3x3, point::Point, Orientation};

mod shaders;
pub mod tesselation;

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

        let renderer = Renderer{
            gl,
            program : program,
            primitives : vec![Primitive{parts : vec![Triangles{vertices:vec![10.0, 30.0, 170.0, 30.0, 100.0, 170.0], mode: TrianglesMode::Strip}], fill: Brush::Color(0.2, 0.7, 0.5, 1.0)}],
            shader_info : shader_info
        };

        renderer.set_transform(Matrix3x3::identity());

        return renderer;
    }

    pub fn set_vertices(&self, attribute : u32, vertices : &[f32], coords_per_vertex : i32) {

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
            coords_per_vertex,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        self.gl.enable_vertex_attrib_array(attribute);

        self.gl.bind_vertex_array(Some(&vao));

    }

    fn draw_primitive(&self, primitive : &Primitive) {

        const coords_per_vertex : i32 = 2;
    
        self.set_brush(&primitive.fill);
        for strip in &primitive.parts {
            self.set_vertices(self.shader_info.a_pos, &strip.vertices, coords_per_vertex as i32);
            match strip.mode {
                TrianglesMode::Fan =>self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLE_FAN, 0, strip.vertices.len() as i32 / coords_per_vertex ),
                TrianglesMode::Strip =>self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, strip.vertices.len() as i32 / coords_per_vertex ),
            }
            
        }
        //red lines
        for strip in &primitive.parts { 
            self.set_vertices(self.shader_info.a_pos, &strip.vertices, coords_per_vertex as i32);
            self.set_brush(&Brush::Color(1.0, 0.0, 0.0, 1.0));
            self.gl.draw_arrays(WebGl2RenderingContext::LINE_LOOP, 0, strip.vertices.len() as i32 / coords_per_vertex);
        }
    }

    pub fn resize_viewport(&self, width : f32, height : f32) {

        let resolution_attribute_location = self.gl.uniform2f(self.shader_info.u_res.as_ref(), width, height);

        self.gl.viewport(0,0, width as i32, height as i32);
    }

    fn set_brush(&self, brush : &Brush) {
        match brush {
            Brush::Color(r, g, b, a) => {
                self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 1);
                self.gl.uniform4f(self.shader_info.u_color.as_ref(), f32::to_owned(r), f32::to_owned(g), f32::to_owned(b), f32::to_owned(a));
            },
            Brush::LinearGradient(gradient) => {
                self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 2);

                self.gl.uniform2f(self.shader_info.gradient_start.as_ref(), gradient.x1, gradient.y1);
                self.gl.uniform2f(self.shader_info.gradient_end.as_ref(), gradient.x2, gradient.y2);

                self.gl.uniform1i(self.shader_info.gradient_stops_count.as_ref(), gradient.stops.len() as i32);
                self.gl.uniform4fv_with_f32_array(self.shader_info.colors.as_ref(), &gradient.stops.iter().flat_map(|s| [s.r, s.g, s.b, s.a].into_iter()).collect::<Vec<f32>>());
                self.gl.uniform1fv_with_f32_array(self.shader_info.gradient_stops.as_ref(), &gradient.stops.iter().map(|s| s.position).collect::<Vec<f32>>());
            },
            Brush::RadialGradient(gradient) => {
                self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 3);

                self.gl.uniform2f(self.shader_info.gradient_start.as_ref(), gradient.x1, gradient.y1);
                self.gl.uniform2f(self.shader_info.gradient_end.as_ref(), gradient.x2, gradient.y2);

                self.gl.uniform1i(self.shader_info.gradient_stops_count.as_ref(), gradient.stops.len() as i32);
                self.gl.uniform4fv_with_f32_array(self.shader_info.colors.as_ref(), &gradient.stops.iter().flat_map(|s| [s.r, s.g, s.b, s.a].into_iter()).collect::<Vec<f32>>());
                self.gl.uniform1fv_with_f32_array(self.shader_info.gradient_stops.as_ref(), &gradient.stops.iter().map(|s| s.position).collect::<Vec<f32>>());
            },
            Brush::ConicGradient(gradient) => {
                self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 4);

                self.gl.uniform2f(self.shader_info.gradient_start.as_ref(), gradient.x1, gradient.y1);
                self.gl.uniform2f(self.shader_info.gradient_end.as_ref(), gradient.x2, gradient.y2);

                self.gl.uniform1i(self.shader_info.gradient_stops_count.as_ref(), gradient.stops.len() as i32);
                self.gl.uniform4fv_with_f32_array(self.shader_info.colors.as_ref(), &gradient.stops.iter().flat_map(|s| [s.r, s.g, s.b, s.a].into_iter()).collect::<Vec<f32>>());
                self.gl.uniform1fv_with_f32_array(self.shader_info.gradient_stops.as_ref(), &gradient.stops.iter().map(|s| s.position).collect::<Vec<f32>>());
            }
        };
    }

    pub fn set_transform(&self, matrix : Matrix3x3) {
        self.gl.uniform_matrix3fv_with_f32_array(self.shader_info.transform.as_ref(), false, &matrix.data())
    }

    pub fn add_primitive(&mut self, primitive : Primitive) {
        self.primitives.push(primitive);
    }

}



pub struct Primitive {
    pub parts : Vec<Triangles>,
    pub fill : Brush
}

pub struct Triangles {
    pub vertices : Vec<f32>,
    pub mode : TrianglesMode,
}

pub enum TrianglesMode {
    Strip, Fan
}

#[derive(Clone)]
pub struct Gradient {
    pub x1 : f32,
    pub y1 : f32,
    pub x2 : f32,
    pub y2 : f32,
    pub stops : Vec<GradientStop>,
}

#[derive(Clone)]
pub struct GradientStop {
    pub position : f32,
    pub r : f32,
    pub g : f32,
    pub b : f32,
    pub a : f32,
}

#[derive(Clone)]
pub enum Brush {
    Color(f32, f32, f32, f32),
    LinearGradient(Gradient),
    RadialGradient(Gradient),
    ConicGradient(Gradient),
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct P {
    x: f32,
    y: f32,
}


impl P {
    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn new(x :f32, y : f32) -> P {
        P{x: x, y: y }
    }
}

pub struct Polygon {
    pub orientation : Orientation,
    pub points : Vec<P>
}


pub fn draw(renderer: &Renderer) {

    time_with_label("Render time");

    renderer.gl.clear_color(0.0, 0.0, 0.2, 1.0);
    renderer.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    for primitive in renderer.primitives.iter() {

        renderer.draw_primitive(primitive);
    }

    time_end_with_label("Render time");
}




