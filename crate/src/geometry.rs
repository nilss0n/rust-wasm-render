use crate::gl::*;
use js_sys::WebAssembly;

// TODO: Move logging helpers to a more suitable module.
use web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

use wasm_bindgen::JsCast;

pub struct Geometry {
    id: WebGlVertexArrayObject,
    count: i32,
    buffer: Vec<f32>,
}

impl Geometry {

    pub fn draw(self, gl: &GL) {
        gl.bind_vertex_array(Some(&self.id));
        gl.draw_arrays(
            GL::TRIANGLES,
            0,
            self.count
        );
    }
}

pub fn create_fullscreen_triangle(gl: &GL) -> Result<Geometry, String>  {
    let vertices =  vec![-1.0, -1.0, 0.0, 3.0, -1.0, 0.0, -1.0, 3.0, 0.0];
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .map_err(|_| "Failed to allocate buffer.")?
        .buffer();

    let vertices_location = vertices.as_ptr() as u32 / 4;
    let vert_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(vertices_location, vertices_location + vertices.len() as u32);


    //console::log_1(&format!("HEJ: {} {}", vertices_location, vertices.len()).into());
    let array = gl.create_vertex_array().ok_or("failed to create vertex_array")?;
    gl.bind_vertex_array(Some(&array));
    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        GL::ARRAY_BUFFER,
        &vert_array,
        GL::STATIC_DRAW,
    );

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);

    log!("{:?} {} {}", &buffer, vertices_location, vertices.len());

    Ok(Geometry { id: array, count: (vertices.len() / 3) as i32, buffer: vertices })
}


pub fn create_unit_quad(gl: &GL) -> Result<Geometry, String>  {
    let vertices = vec![
        -1.0, -1.0, 0.0, 1.0, -1.0, 0.0, -1.0, 1.0, 0.0,
        1.0, -1.0, 0.0, 1.0, 1.0, 0.0, -1.0, 1.0, 0.0,
    ];
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<WebAssembly::Memory>()
        .map_err(|_| "Failed to allocate buffer.")?
        .buffer();


    let vertices_location = vertices.as_ptr() as u32 / 4;
    let vert_array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(vertices_location, vertices_location + vertices.len() as u32);


    let array = gl.create_vertex_array().ok_or("failed to create vertex_array")?;
    gl.bind_vertex_array(Some(&array));

    let buffer = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&buffer));
    gl.buffer_data_with_array_buffer_view(
        GL::ARRAY_BUFFER,
        &vert_array,
        GL::STATIC_DRAW,
    );

    gl.enable_vertex_attrib_array(0);
    gl.vertex_attrib_pointer_with_i32(0, 3, GL::FLOAT, false, 0, 0);


    log!("{:?} {} {}", &buffer, vertices_location, vertices.len());
    Ok(Geometry { id: array, count: (vertices.len() / 3) as i32, buffer: vertices  })
}