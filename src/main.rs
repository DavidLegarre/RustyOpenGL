extern crate gl;
extern crate glfw;

use glfw::{fail_on_errors, Action, Context, Key};

const WINDOW_TITLE: &str = "Hello, Window!";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    //let vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let (mut glfw, mut window, events) = init_window();

    while !window.should_close() {
        process_events(&mut glfw, &mut window, &events);
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
