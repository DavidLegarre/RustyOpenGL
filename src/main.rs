extern crate gl;
extern crate glfw;
mod debugging;
mod window;

use window::window::{init_window, rendering_loop};

const WINDOW_TITLE: &str = "OpenGL Rendering";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let (glfw, window, events) = init_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE);
    unsafe {
        rendering_loop(window, glfw, events);
        glfw::ffi::glfwTerminate();
    }
}
