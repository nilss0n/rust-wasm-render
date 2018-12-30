use crate::gl::*;


pub fn compile_shader_program(gl: &GL, vertex_src: &str, fragment_src: &str) -> Result<WebGlProgram, String> {
    let vert_shader = compile_shader(gl, GL::VERTEX_SHADER, vertex_src)?;

    let frag_shader = compile_shader(gl, GL::FRAGMENT_SHADER, fragment_src)?;
    let program = link_program(gl, [vert_shader, frag_shader].iter())?;

    Ok(program)
}

pub fn compile_shader(context: &GL, shader_type: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, GL::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown error creating shader".into()))
    }
}

pub fn link_program<'a, T: IntoIterator<Item = &'a WebGlShader>>(context: &GL, shaders: T) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    for shader in shaders {
        context.attach_shader(&program, shader)
    }

    context.link_program(&program);

    if context
        .get_program_parameter(&program, GL::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(
            context
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown error creating program object".into())
        )
    }
}