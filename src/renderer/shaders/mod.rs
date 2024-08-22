use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};


const vertex_shader_source : &str = r##"#version 300 es
        #pragma debug(on) 
            // an attribute will receive data from a buffer
            in vec2 a_pos;
            uniform vec2 u_res;

            //out vec4 out_pos;

            // void normaizuj(inout vvv, in res) {

            // }


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

const fragemnt_shader_source : &str = r##"#version 300 es
            // fragment shaders don't have a default precision so we need
            // to pick one. mediump is a good default
            precision highp float;

            uniform uint u_brush_type;
            uniform vec4 u_color;

            out vec4 out_color;

            void main() {
                // gl_FragColor is a special variable a fragment shader
                // is responsible for setting
                if (u_brush_type == uint(1)) { // solid color
                    out_color = u_color;
                } else {
                    out_color = vec4(1.0, 0.2, 0.2, 1.0);
                }
            }
        "##;

pub fn create_vertex_shader(gl : &WebGl2RenderingContext) -> WebGlShader {
    compile_shader(
        gl,
        WebGl2RenderingContext::VERTEX_SHADER,
        vertex_shader_source,
    ).unwrap()
}

pub fn create_fragment_shader(gl : &WebGl2RenderingContext) -> WebGlShader {
    compile_shader(
        gl,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        fragemnt_shader_source
    ).unwrap()
}


fn compile_shader(
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


fn link_program(
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

pub struct ShaderInfo {
    pub u_color : Option<WebGlUniformLocation>,
    pub u_res : Option<WebGlUniformLocation>,
    pub a_pos : u32,
    pub u_brush_type : Option<WebGlUniformLocation>
}

pub fn create_shader_program(gl : &WebGl2RenderingContext) -> (WebGlProgram, ShaderInfo) {

    let vert_shader = create_vertex_shader(gl);
    let frag_shader = create_fragment_shader(gl);

    let program = link_program(gl, &vert_shader, &frag_shader).unwrap();

    let shader_info = ShaderInfo{
        u_color : gl.get_uniform_location(&program, "u_color"),
        u_res : gl.get_uniform_location(&program, "u_res"),
        a_pos : gl.get_attrib_location(&program, "a_pos") as u32,
        u_brush_type : gl.get_uniform_location(&program, "u_brush_type"),
    };

    (program, shader_info)
}