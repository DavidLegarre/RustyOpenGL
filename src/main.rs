extern crate gl;
extern crate glfw;
mod debugging;
mod window;
use rand::{thread_rng, Rng};

use window::window::{init_window, rendering_loop};

const WINDOW_TITLE: &str = "OpenGL Rendering";
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn generate_random_vertices(n: usize) -> Vec<f32> {
    let mut rng = thread_rng();
    let mut vertices = Vec::with_capacity(n * 9);
    for _ in 0..n {
        for _ in 0..3 {
            vertices.push(rng.gen_range(-1.0..1.0));
            vertices.push(rng.gen_range(-1.0..1.0));
            vertices.push(0.0); // z-coordinate
        }
    }
    vertices
}


fn main() {
    let (glfw, window, events) = init_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE);
    unsafe {
        rendering_loop(window, glfw, events);
        glfw::ffi::glfwTerminate();
    }
}
