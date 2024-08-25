use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader, WebGlUniformLocation};


const vertex_shader_source : &str = include_str!("vertex_shader.hlsl");

const fragemnt_shader_source : &str = include_str!("fragment_shader.hlsl");

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
    pub u_brush_type : Option<WebGlUniformLocation>,
    pub gradient_start : Option<WebGlUniformLocation>,
    pub gradient_end : Option<WebGlUniformLocation>,
    pub colors : Option<WebGlUniformLocation>,
    pub gradient_stops : Option<WebGlUniformLocation>,
    pub gradient_stops_count : Option<WebGlUniformLocation>,
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
        gradient_start : gl.get_uniform_location(&program, "gradient_start"),
        gradient_end : gl.get_uniform_location(&program, "gradient_end"),
        colors : gl.get_uniform_location(&program, "colors"),
        gradient_stops : gl.get_uniform_location(&program, "gradient_stops"),
        gradient_stops_count : gl.get_uniform_location(&program, "gradient_stops_count"),
    };

    (program, shader_info)
}