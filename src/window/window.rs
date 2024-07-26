#![allow(non_snake_case)]

use std::ffi::CString;

use super::render::{
    get_triangle_array, PINK_FRAGMENT_SHADER, VERTEX_SHADER_SOURCE,
};
use crate::glfw::Context;
use crate::{debugging::check_errors::gl_check_error, window::render::compile_triangle_shaders};
use glfw::fail_on_errors;

const VERTICES_1: [f32; 9] = [
    0.5, -0.5, 0.0, // bottom right
    -0.5, -0.5, 0.0, // bottom let
    0.0, 0.5, 0.0, // top
];

pub fn init_window(
    width: u32,
    height: u32,
    title: &str,
) -> (
    glfw::Glfw,
    glfw::PWindow,
    glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    let mut glfw = init_glfw();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Load all opengl function constants
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // Set the framebuffer size callback
    // This makes it so whenever the window gets an event to change the size it resizes the viewport
    window.set_framebuffer_size_callback(|window, width, height| unsafe {
        let _window = window;
        gl::Viewport(0, 0, width, height);
    });

    (glfw, window, events)
}
fn init_glfw() -> glfw::Glfw {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 5));

    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    glfw
}

pub unsafe fn rendering_loop(
    mut window: glfw::PWindow,
    mut glfw: glfw::Glfw,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    // Wireframe mode
    // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

    while !window.should_close() {
        process_events(&mut window, &events);
        clear_screen();

        let shaderProgram = compile_triangle_shaders(VERTEX_SHADER_SOURCE, PINK_FRAGMENT_SHADER);
        let timeValue = glfw.get_time();
        let greenValue: f32 = ((timeValue.sin() / 2.0) + 0.5) as f32;
        let redValue: f32 = ((timeValue.cos() / 2.0) + 0.5) as f32;
        let blueValue: f32 = ((timeValue.sin() / 1.0) + 0.5) as f32;
        let uniformName = CString::new("ourColor").unwrap();
        let vertexColorLocation = gl::GetUniformLocation(shaderProgram, uniformName.as_ptr());
        gl::UseProgram(shaderProgram);
        gl::Uniform4f(vertexColorLocation, redValue, greenValue, blueValue, 1.0);
        draw_triangle(&VERTICES_1);

        window.swap_buffers();

        glfw.poll_events();
    }
}

fn process_events(
    window: &mut glfw::PWindow,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    for (_, event) in glfw::flush_messages(events) {
        println!("{:?}", event);
        match event {
            glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
    unsafe {
        crate::gl_check_error!();
    }
}

unsafe fn clear_screen() {
    gl::ClearColor(0., 0., 0., 1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);
}

unsafe fn draw_triangle(vertices: &[f32]) {
    let VAO = get_triangle_array(vertices);
    gl::BindVertexArray(VAO);
    gl::DrawArrays(gl::TRIANGLES, 0, 3);
}
