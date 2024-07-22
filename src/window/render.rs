#![allow(non_snake_case)]
use std::ffi::CString;
use std::os::raw::c_void;
use std::{mem, ptr, str};

use gl::types::*;

const VERTICES: [f32; 9] = [
    -0.5, -0.5, 0.0, // left
    0.5, -0.5, 0.0, // right
    0.0, 0.5, 0.0, // top
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
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

pub unsafe fn render_triangle() -> (u32, u32) {
    // Rendering logic for a triangle

    // Build and compile vertex shader
    let vertex_shader = build_compile_shader(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER);

    // fragment shader
    let fragment_shader = build_compile_shader(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER);

    // link shaders
    let shader_program = gl::CreateProgram();
    gl::AttachShader(shader_program, vertex_shader);
    gl::AttachShader(shader_program, fragment_shader);
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
    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    let (mut VBO, mut VAO) = (0, 0);
    gl::GenVertexArrays(1, &mut VAO);
    gl::GenBuffers(1, &mut VBO);
    gl::BindVertexArray(VAO);

    gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (VERTICES.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
        &VERTICES[0] as *const f32 as *const c_void,
        gl::STATIC_DRAW,
    );

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
