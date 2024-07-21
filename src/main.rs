extern crate gl;
extern crate glfw;

use gl::types::*;

use glfw::{fail_on_errors, Action, Context, Key};

const WINDOW_TITLE: &str = "Hello, Window!";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let vertices: Vec<f32> = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let (mut glfw, mut window, events) = init_window();

    while !window.should_close() {
        process_events(&mut glfw, &mut window, &events);

        let mut vbo: GLuint = 0;
        unsafe {
            check_gl_error(); // Check before GenBuffers
            gl::GenBuffers(1, &mut vbo);
            check_gl_error(); // Check after GenBuffers

            check_gl_error(); // Check before BindBuffer
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            check_gl_error(); // Check after BindBuffer

            check_gl_error(); // Check before BufferData
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
            check_gl_error(); // Check after BufferData

            check_gl_error(); // Check before unbinding buffer
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            check_gl_error(); // Check after unbinding buffer
        }
    }
}

fn init_glfw() -> glfw::Glfw {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    glfw
}

fn init_window() -> (
    glfw::Glfw,
    glfw::PWindow,
    glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    let mut glfw = init_glfw();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
            WINDOW_TITLE,
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // Set the framebuffer size callback
    window.set_framebuffer_size_callback(|window, width, height| unsafe {
        let _window = window;
        gl::Viewport(0, 0, width, height);
    });

    (glfw, window, events)
}

fn process_events(
    glfw: &mut glfw::Glfw,
    window: &mut glfw::PWindow,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    glfw.poll_events();
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}

fn check_gl_error() {
    let mut error_code: GLenum;
    unsafe {
        loop {
            error_code = gl::GetError();
            if error_code == gl::NO_ERROR {
                break;
            }

            let error_str = match error_code {
                gl::INVALID_ENUM => "GL_INVALID_ENUM",
                gl::INVALID_VALUE => "GL_INVALID_VALUE",
                gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
                gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
                gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
                gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
                gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
                _ => "Unknown error",
            };

            println!("OpenGL Error: {} ({})", error_str, error_code);
        }
    }
}
