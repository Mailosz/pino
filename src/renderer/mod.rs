use std::{borrow::Borrow, cell::RefCell, f32::consts::PI, mem, primitive, time::Instant};

use shaders::{ create_shader_program, ShaderInfo};
use wasm_bindgen::prelude::*;
use web_sys::{console::{self, time_end_with_label, time_with_label}, HtmlImageElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};
use crate::{base::*, bounds::Bounds, math::rect::Rect, matrix::Matrix3x3, point::Point, Orientation};

mod shaders;
pub mod tesselation;

pub struct Renderer {
    gl : WebGl2RenderingContext,
    program : WebGlProgram,
    primitives : Vec<Primitive>,
    shader_info : ShaderInfo,
    current_viewport : Rect,
    viewports : Vec<ViewportData>,
    main_viewport_index : usize,
    cache_viewport_index : usize,
    zoom_mode : ZoomMode,
    width : i32,
    height: i32
}

struct ViewportData {
    viewport: Rect,
    framebuffer: FramebufferData

}

impl ViewportData {
    pub fn empty() -> ViewportData {
        ViewportData{viewport: Rect::new(0.0, 0.0, 1.0, 1.0), framebuffer: FramebufferData{framebuffer: Option::None, texture : Option::None, width: 1, height: 1, drawn: false}}
    }
}

struct FramebufferData {
    framebuffer : Option<web_sys::WebGlFramebuffer>,
    texture : Option<web_sys::WebGlTexture>,
    width : i32,
    height: i32,
    drawn : bool,
}

enum ZoomMode {
    In, Out, None
}

impl Renderer {
    pub fn create(gl : WebGl2RenderingContext) -> Renderer{

        let (program, shader_info) = create_shader_program(&gl);
        gl.use_program(Some(&program));



        let renderer = Renderer{
            gl,
            program : program,
            primitives : vec![Primitive{parts : vec![Triangles{vertices:vec![10.0, 30.0, 170.0, 30.0, 100.0, 170.0], mode: TrianglesMode::Strip}], fill: Brush::Color(0.2, 0.7, 0.5, 1.0)}],
            shader_info : shader_info,
            current_viewport : Rect::new(0.0, 0.0, 1.0, 1.0),
            viewports : vec![ViewportData::empty(), ViewportData::empty()],
            main_viewport_index : 0,
            cache_viewport_index : 1,
            zoom_mode : ZoomMode::None,
            width: 1,
            height: 1
        };

        renderer.set_transform(Matrix3x3::identity());

        return renderer;
    }

    /**
     * Sets the current framebuffer for OpenGL
     */
    pub fn set_framebuffer(&self, framebuffer_data : &FramebufferData) {
        self.gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, framebuffer_data.framebuffer.as_ref());

        self.gl.viewport(0,0, framebuffer_data.width, framebuffer_data.height);
    }

    /**
     * Sets the current framebuffer to canvas
     */
    pub fn reset_framebuffer(&self) {
        self.gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Option::None);

        self.gl.viewport(0,0, self.width, self.height);
    }

    pub fn create_image_brush(&self, image : HtmlImageElement ) -> ImageData {

        let texture: Option<web_sys::WebGlTexture> = self.gl.create_texture();
    
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

        let internal_format = WebGl2RenderingContext::RGBA;
        let tex = self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_html_image_element(
            WebGl2RenderingContext::TEXTURE_2D, 
            0, 
            internal_format as i32, // internalFormat
            image.width() as i32, 
            image.height() as i32, 
            0, // must be 0
            internal_format, // srcFormat
            WebGl2RenderingContext::UNSIGNED_BYTE, // srcType
            &image // copy pixels from image
            );

        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);

        return ImageData{
            viewport: Bounds::new_fast( 0.0,0.0, image.width() as f64, image.height() as f64),
            image : Some(image),
            texture : texture,
        };

    }

    pub fn create_texture_framebuffer(gl : &WebGl2RenderingContext, width : i32, height : i32) -> FramebufferData{

        let texture = gl.create_texture();

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

        let internal_format = WebGl2RenderingContext::RGBA;
        let tex = gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            WebGl2RenderingContext::TEXTURE_2D, 
            0, 
            internal_format as i32, // internalFormat
            width, 
            height, 
            0, // must be 0
            internal_format, // srcFormat
            WebGl2RenderingContext::UNSIGNED_BYTE, // srcType
            Option::None // data is null - we will render into it
            );

        let framebuffer = gl.create_framebuffer();
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, framebuffer.as_ref());
        //attach texture to framebuffer
        let level : i32 = 0; // Dont know what's this
        gl.framebuffer_texture_2d(WebGl2RenderingContext::FRAMEBUFFER, WebGl2RenderingContext::COLOR_ATTACHMENT0, WebGl2RenderingContext::TEXTURE_2D, texture.as_ref(), level);
        
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
        //
        return FramebufferData{framebuffer: framebuffer, texture : texture, width: width, height : height, drawn: false};
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

        const COORDS_PER_VERTEX : i32 = 2;
    
        self.set_brush(&primitive.fill);
        for strip in &primitive.parts {
            self.set_vertices(self.shader_info.a_pos, &strip.vertices, COORDS_PER_VERTEX as i32);
            match strip.mode {
                TrianglesMode::Fan => self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLE_FAN, 0, strip.vertices.len() as i32 / COORDS_PER_VERTEX ),
                TrianglesMode::Strip => self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLE_STRIP, 0, strip.vertices.len() as i32 / COORDS_PER_VERTEX ),
            }
            
        }
        //red lines
        for strip in &primitive.parts { 
            self.set_vertices(self.shader_info.a_pos, &strip.vertices, COORDS_PER_VERTEX as i32);
            self.set_brush(&Brush::Color(1.0, 0.0, 0.0, 1.0));
            self.gl.draw_arrays(WebGl2RenderingContext::LINE_LOOP, 0, strip.vertices.len() as i32 / COORDS_PER_VERTEX);
        }
    }

    /**
     * Draws the framebuffer contents to the screen
     */
    fn draw_framebuffer(&self, framebuffer_data : &FramebufferData) {

        let width = framebuffer_data.width as f32;
        let height = framebuffer_data.height as f32;

        let vertices : [f32; 8] = [width, height, 0.0, height, 0.0, 0.0, width, 0.0];

        // SET TEXTURE 
        self.set_texture_brush(
            framebuffer_data.texture.as_ref(), 
            &Bounds::new_fast(0.0, 0.0, width as f64, height as f64), 
            &Matrix3x3::identity(), 
            WebGl2RenderingContext::CLAMP_TO_EDGE, 
            WebGl2RenderingContext::CLAMP_TO_EDGE);       

        self.set_vertices(self.shader_info.a_pos, &vertices, 2 as i32);
        self.gl.draw_arrays(WebGl2RenderingContext::TRIANGLE_FAN, 0, 4 as i32);

        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, None);
    }

    /**
     * Resizes all the viewport buffers
     */
    pub fn resize_screen(&mut self, width : i32, height : i32) {

        self.width = width;
        self.height = height;

        // resize framebuffers
        for viewport in &mut self.viewports {
            Renderer::resize_viewport_framebuffer(&self.gl, viewport, width, height); 
        }

        // set resolution for a shader
        self.gl.uniform2f(self.shader_info.u_res.as_ref(), width as f32, height as f32);

        // set viewport for WebGL
        self.gl.viewport(0,0, width, height);
    }

    /**
     * Deletes the current framebuffer and creates new one for specified dimensions
     */
    fn resize_viewport_framebuffer(gl : &WebGl2RenderingContext, viewport_data : &mut ViewportData, width : i32, height : i32) {

        // resize framebuffer
        gl.delete_framebuffer(viewport_data.framebuffer.framebuffer.as_ref());
        gl.delete_texture(viewport_data.framebuffer.texture.as_ref());
        viewport_data.framebuffer = Renderer::create_texture_framebuffer(gl, width, height);
    }

    fn set_brush(&self, brush : &Brush) {
        match brush {
            Brush::Color(r, g, b, a) => {
                self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 1);
                self.gl.uniform4f(self.shader_info.u_color.as_ref(), f32::to_owned(r), f32::to_owned(g), f32::to_owned(b), f32::to_owned(a));
            },
            Brush::LinearGradient(gradient) => {
                self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 2);

                self.gl.uniform2f(self.shader_info.brush_start.as_ref(), gradient.x1, gradient.y1);
                self.gl.uniform2f(self.shader_info.brush_end.as_ref(), gradient.x2, gradient.y2);

                self.gl.uniform1i(self.shader_info.gradient_stops_count.as_ref(), gradient.stops.len() as i32);
                self.gl.uniform4fv_with_f32_array(self.shader_info.colors.as_ref(), &gradient.stops.iter().flat_map(|s| [s.r, s.g, s.b, s.a].into_iter()).collect::<Vec<f32>>());
                self.gl.uniform1fv_with_f32_array(self.shader_info.gradient_stops.as_ref(), &gradient.stops.iter().map(|s| s.position).collect::<Vec<f32>>());
            },
            Brush::RadialGradient(gradient) => {
                self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 3);

                self.gl.uniform2f(self.shader_info.brush_start.as_ref(), gradient.x1, gradient.y1);
                self.gl.uniform2f(self.shader_info.brush_end.as_ref(), gradient.x2, gradient.y2);

                self.gl.uniform1i(self.shader_info.gradient_stops_count.as_ref(), gradient.stops.len() as i32);
                self.gl.uniform4fv_with_f32_array(self.shader_info.colors.as_ref(), &gradient.stops.iter().flat_map(|s| [s.r, s.g, s.b, s.a].into_iter()).collect::<Vec<f32>>());
                self.gl.uniform1fv_with_f32_array(self.shader_info.gradient_stops.as_ref(), &gradient.stops.iter().map(|s| s.position).collect::<Vec<f32>>());
            },
            Brush::ConicGradient(gradient) => {
                self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 4);

                self.gl.uniform2f(self.shader_info.brush_start.as_ref(), gradient.x1, gradient.y1);
                self.gl.uniform2f(self.shader_info.brush_end.as_ref(), gradient.x2, gradient.y2);

                self.gl.uniform1i(self.shader_info.gradient_stops_count.as_ref(), gradient.stops.len() as i32);
                self.gl.uniform4fv_with_f32_array(self.shader_info.colors.as_ref(), &gradient.stops.iter().flat_map(|s| [s.r, s.g, s.b, s.a].into_iter()).collect::<Vec<f32>>());
                self.gl.uniform1fv_with_f32_array(self.shader_info.gradient_stops.as_ref(), &gradient.stops.iter().map(|s| s.position).collect::<Vec<f32>>());
            },
            Brush::ImageBrush(image_data) => {
                self.set_texture_brush(image_data.texture.as_ref(), &image_data.viewport, &Matrix3x3::identity(), WebGl2RenderingContext::MIRRORED_REPEAT, WebGl2RenderingContext::MIRRORED_REPEAT);             
            }
        };
    }
    
    fn set_texture_brush(&self, texture : Option<&web_sys::WebGlTexture>, source_bounds : &Bounds, texture_transform : &Matrix3x3, wrap_x : u32, wrap_y : u32) {
        self.gl.uniform1ui(self.shader_info.u_brush_type.as_ref(), 5);
        self.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture);
        // self.gl.active_texture(WebGl2RenderingContext::TEXTURE0);
        self.gl.uniform2f(self.shader_info.brush_start.as_ref(), source_bounds.l() as f32 , source_bounds.t() as f32);
        self.gl.uniform2f(self.shader_info.brush_end.as_ref(), source_bounds.r() as f32 , source_bounds.b() as f32);
        self.gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);

        self.gl.uniform_matrix3fv_with_f32_array(self.shader_info.texture_transform.as_ref(), false, &texture_transform.data());
        self.gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, wrap_x as i32);
        self.gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, wrap_y as i32);

    }

    pub fn set_transform(&self, matrix : Matrix3x3) {
        self.gl.uniform_matrix3fv_with_f32_array(self.shader_info.transform.as_ref(), false, &matrix.data())
    }

    pub fn add_primitive(&mut self, primitive : Primitive) {
        self.primitives.push(primitive);
    }

    /**
     * 
     */
    pub fn refresh_view(&mut self, target_viewport : &Rect) {

        if (self.viewports[self.main_viewport_index].viewport.contains_rect(target_viewport)) {
            // if the before viewport covers the 
            self.zoom_mode = ZoomMode::In;
            if (self.viewports[self.main_viewport_index].viewport.contains_rect(&self.current_viewport)) {
                mem::swap(&mut self.main_viewport_index, &mut self.cache_viewport_index);
            }
        } else {
            self.zoom_mode = ZoomMode::Out;
        }
        
        self.refresh_viewport_data(self.main_viewport_index, &target_viewport);

    }

    pub fn refresh_viewport_data(&mut self, viewport_index : usize , viewport : &Rect) {
        self.set_framebuffer(&self.viewports[viewport_index].framebuffer);

        let zoomX = self.width as f32 / viewport.w() as f32;
        let zoomY = self.height as f32 / viewport.h() as f32;
    
        self.set_transform(Matrix3x3::new(
            zoomY, 0.0, -viewport.y() as f32 * zoomY,
            0.0, zoomX, -viewport.x() as f32 * zoomX,
            0.0, 0.0, 1.0
        ));
        
        self.gl.clear_color(0.0, 0.0, 0.2, 1.0);
        self.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        
        for primitive in self.primitives.iter() {
            self.draw_primitive(primitive);
        }

        let viewport_data = &mut self.viewports[viewport_index];//.framebuffer.drawn = true;
        
        viewport_data.framebuffer.drawn = true;
        viewport_data.viewport = viewport.clone();
    }
    
    /**
     * Draws the framebuffer data to the screen, applying necessary transformations to fit the viewport
     */
    fn draw_viewport(&self, viewport_index: usize, viewport: &Rect) -> () {
        let viewport_data = &self.viewports[viewport_index];

        if (!viewport_data.framebuffer.drawn) {
            return;
        }

        let zoomX = viewport_data.viewport.w() / viewport.w();
        let zoomY = viewport_data.viewport.h() / viewport.h();
        let offsetX = (viewport_data.viewport.x() - viewport.x()) / (viewport.w() / self.width as f64);
        let offsetY = (viewport_data.viewport.y() - viewport.y()) / (viewport.h() / self.height as f64);

        // scrollX = this.viewport.x - (origin.x - this.viewport.x) * ((zoomX) - 1);


        self.set_transform(Matrix3x3::new(
            zoomY as f32, 0.0, offsetY as f32,
            0.0, zoomX as f32, offsetX as f32,
            0.0, 0.0, 1.0
        ));


        self.draw_framebuffer(&viewport_data.framebuffer);
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
    ImageBrush(ImageData)
}

#[derive(Clone)]
pub struct ImageData {
    image : Option<HtmlImageElement>,
    texture : Option<web_sys::WebGlTexture>,
    viewport : Bounds,
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

/**
 * Causes the target viewport dimensions to be set, and redraws it
 */
pub fn refresh_viewport(renderer: &mut Renderer, viewport : Rect ) {

    renderer.refresh_view(&viewport);
    
}

/**
 * Renders everything to the screen
 */
pub fn render_screen(renderer: &mut Renderer, viewport : Rect ) {

    renderer.reset_framebuffer();
    renderer.gl.clear_color(0.6, 0.7, 0.8, 1.0);
    renderer.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    renderer.current_viewport = viewport.clone();
    
    match renderer.zoom_mode {
        ZoomMode::In => {
            renderer.draw_viewport(renderer.cache_viewport_index, &viewport);
            renderer.draw_viewport(renderer.main_viewport_index, &viewport);
        },
        ZoomMode::Out => {
            renderer.draw_viewport(renderer.main_viewport_index, &viewport);
            renderer.draw_viewport(renderer.cache_viewport_index, &viewport);
        },
        ZoomMode::None => {
            renderer.draw_viewport(renderer.main_viewport_index, &viewport);
        }
    }



}

pub fn draw(renderer: &mut Renderer) {

    time_with_label("Render time");
    
    renderer.reset_framebuffer();
    renderer.gl.clear_color(0.6, 0.7, 0.8, 1.0);
    renderer.gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);



    // for primitive in renderer.primitives.iter() {

    //     renderer.draw_primitive(primitive);
    // }

    // renderer.gl.uniform1ui(renderer.shader_info.u_brush_type.as_ref(), 5);
    // // renderer.gl.active_texture(WebGl2RenderingContext::TEXTURE0);
    // renderer.gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, renderer.main_framebuffer.texture.as_ref());
    // renderer.gl.generate_mipmap(WebGl2RenderingContext::TEXTURE_2D);
    // renderer.gl.uniform_matrix3fv_with_f32_array(renderer.shader_info.texture_transform.as_ref(), false, &Matrix3x3::identity().data());
    // renderer.gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::MIRRORED_REPEAT as i32);
    // renderer.gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::MIRRORED_REPEAT as i32);

    // let vertices = [100.0, 100.0, 2000.0, 100.0, 2000.0, 2000.0, 100.0, 2000.0];
    // renderer.set_vertices(renderer.shader_info.a_pos, &vertices, 2);
    // renderer.gl.draw_arrays(WebGl2RenderingContext::TRIANGLE_FAN, 0, vertices.len() as i32 / 2 );

    time_end_with_label("Render time");
}




