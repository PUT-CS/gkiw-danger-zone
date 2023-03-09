use glfw::{ffi::glfwSwapInterval, Context};

const SCR_WIDTH: u32 = 1920;
const SCR_HEIGHT: u32 = 1080;

mod cg {
    pub mod shader;
    pub mod camera;
}
mod macros;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, _events) = glfw
        .create_window(
            SCR_WIDTH,
            SCR_HEIGHT,
            "LearnOpenGL",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_framebuffer_size_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        glfwSwapInterval(0);
        gl::ClearColor(0.1, 0.4, 0.6, 1.0);
    }
    
    while !window.should_close() {

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        window.swap_buffers();
        glfw.poll_events();
    }
}
