extern crate glfw;
extern crate gl;

use glfw::{Action, Context, Key};

static SCREEN_WIDTH: u32 = 800;
static SCREEN_HEIGHT: u32 = 800;
static TITLE: &str = "LD50 Game";

#[cfg(target_os = "macos")]
unsafe fn set_window_hints() {
    use glfw::ffi::glfwWindowHint;

    let GLFW_OPENGL_FORWARD_COMPAT = 0x00022006;
    let GLFW_CONTEXT_VERSION_MAJOR = 0x00022002;
    let GLFW_CONTEXT_VERSION_MINOR = 0x00022003;
    let GLFW_OPENGL_PROFILE = 0x00022008;
    let GLFW_OPENGL_CORE_PROFILE = 0x00032001;

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 3);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 3);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
    glfwWindowHint(GLFW_OPENGL_FORWARD_COMPAT, 1);
    glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);
}

#[cfg(target_os = "windows")]
unsafe fn set_window_hints() {
    glfw::WindowHint::ContextVersion(3, 3);
    glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core);
}

fn main() {
    unsafe {
        set_window_hints();
    }

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to initialize glfw");
    
    // create window
    let (mut window, events) = glfw.create_window(SCREEN_WIDTH, SCREEN_HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window!");
    window.make_current();
    window.set_key_polling(true);

    // bind opengl to window
    gl::load_with(|symb| window.get_proc_address(symb));

    ldjam50::engine::eng_work();

    #[allow(unused_labels)]
    'main_loop: while !window.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.1, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                window.set_should_close(true);
            }
        }

        window.swap_buffers();
    }
}