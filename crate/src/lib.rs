use js_sys::WebAssembly;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod shader;
mod gl;
mod geometry;

use crate::gl::*;

const vertex_shader_source : &'static str = include_str!("./shaders/fullscreen_triangle_vertex.glsl");
const fragment_shader_source : &'static str = include_str!("./shaders/fragment.glsl");

const STANDARD_VERTEX : &'static str = include_str!("./shaders/standard_vertex.glsl");
const STANDARD_FRAGMENT : &'static str = include_str!("./shaders/standard_fragment.glsl");



#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas.get_context("webgl2")?.unwrap().dyn_into::<GL>()?;
    let gl = &context;

    let program = shader::compile_shader_program(gl, vertex_shader_source, fragment_shader_source)?;
    let program2 = shader::compile_shader_program(gl, STANDARD_VERTEX, STANDARD_FRAGMENT)?;

    context.use_program(Some(&program));

    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(GL::COLOR_BUFFER_BIT);
    let tri = geometry::create_fullscreen_triangle(gl)?;
    let quad = geometry::create_unit_quad(gl)?;

    tri.draw(gl);

    context.use_program(Some(&program2));

    quad.draw(gl);

    Ok(())
}
