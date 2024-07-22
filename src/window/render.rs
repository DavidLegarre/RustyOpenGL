#![allow(non_snake_case)]
use std::ffi::CString;
use std::os::raw::c_void;
use std::{mem, ptr, str};

use gl::types::*;

const VERTICES: [f32; 12] = [
    0.5, 0.5, 0.0, // top right
    0.5, -0.5, 0.0, // bottom right
    -0.5, -0.5, 0.0, // bottom left
    -0.5, 0.5, 0.0, // top left
];

const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.2f, 0.5f, 1.0f);
    }
"#;

const INDICES: [i32; 6] = [
    0, 1, 3, // first triangle
    1, 2, 3, // second triangle
];
pub unsafe fn render_triangle(vertices: &[f32; 9]) -> (u32, u32) {
    // Rendering logic for a triangle

    // Build and compile vertex shader
    let vertex_shader = build_compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);

    // fragment shader
    let fragment_shader = build_compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);

    // link shaders
    let shaders = [vertex_shader, fragment_shader];
    let shader_program = build_shader_program(&shaders);

    // Add EBO

    // let (mut VBO, mut VAO, mut EBO) = (0, 0, 0);
    let (mut VBO, mut VAO) = (0, 0);
    gl::GenVertexArrays(1, &mut VAO);
    gl::GenBuffers(1, &mut VBO);
    // gl::GenBuffers(1, &mut EBO);

    // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
    gl::BindVertexArray(VAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &vertices[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW,
    );
    // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, EBO);
    // gl::BufferData(
    //     gl::ELEMENT_ARRAY_BUFFER,
    //     (INDICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
    //     &INDICES[0] as *const i32 as *const c_void,
    //     gl::STATIC_DRAW,
    // );

    gl::VertexAttribPointer(
        0,
        3,
        gl::FLOAT,
        gl::FALSE,
        3 * mem::size_of::<GLfloat>() as GLsizei,
        ptr::null(),
    );
    gl::EnableVertexAttribArray(0);

    gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    gl::BindVertexArray(0);

    (shader_program, VAO)
}

unsafe fn build_compile_shader(shader_source: &str, shader_type: GLenum) -> u32 {
    let shader = gl::CreateShader(shader_type);
    let c_str_src = CString::new(shader_source.as_bytes()).unwrap();
    gl::ShaderSource(shader, 1, &c_str_src.as_ptr(), ptr::null());
    gl::CompileShader(shader);
    check_shader_compile_error(shader);

    shader
}

unsafe fn check_shader_compile_error(shader: u32) {
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    info_log.set_len(512 - 1); // subtract 1 to skip the trailing null character
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetShaderInfoLog(
            shader,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
            "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
            str::from_utf8(&info_log).unwrap()
        );
    }
}

unsafe fn build_shader_program(shaders: &[u32]) -> u32 {
    let shader_program = gl::CreateProgram();
    for shader in shaders {
        gl::AttachShader(shader_program, *shader);
    }
    gl::LinkProgram(shader_program);
    // check for linking errors
    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);
    gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
    if success != gl::TRUE as GLint {
        gl::GetProgramInfoLog(
            shader_program,
            512,
            ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        println!(
            "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
            str::from_utf8(&info_log).unwrap()
        );
    }

    for shader in shaders {
        gl::DeleteShader(*shader);
    }

    shader_program
}
