#![allow(non_snake_case)]

use super::render::render_triangle;
use crate::glfw::Context;
use crate::{debugging::check_errors::gl_check_error, window::render::compile_triangle_shaders};
use glfw::fail_on_errors;

const VERTICES_1: [f32; 9] = [
    -0.9, -0.5, 0.0, // let
    -0.0, -0.5, 0.0, // right
    -0.45, 0.5, 0.0, // top
];

const VERTICES_2: [f32; 9] = [
    0.0, -0.5, 0.0, // let
    0.9, -0.5, 0.0, // right
    0.45, 0.5, 0.0, // top
];

pub unsafe fn rendering_loop(
    mut window: glfw::PWindow,
    mut glfw: glfw::Glfw,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    while !window.should_close() {
        process_events(&mut window, &events);
        crate::gl_check_error!();

        gl::ClearColor(0., 0., 0., 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        let shader_program = compile_triangle_shaders();
        let VAO = render_triangle(&VERTICES_1);
        gl::UseProgram(shader_program);
        gl::BindVertexArray(VAO);
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
        // Wireframe mode
        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        let VAO = render_triangle(&VERTICES_2);
        // gl::UseProgram(shader_program);
        gl::BindVertexArray(VAO);
        gl::DrawArrays(gl::TRIANGLES, 0, 6);
        window.swap_buffers();

        glfw.poll_events();
    }
}

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
}
