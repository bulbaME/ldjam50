use super::*;

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

pub fn init<'a>() -> (Engine<'a>, EventHandler) {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)
        .expect("Failed to initialize glfw");

   unsafe {
        set_window_hints();
    }

    let (mut window, events) = glfw.create_window(WINDOW_WIDTH, WINDOW_HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    window.make_current();
    window.set_key_polling(true);
    
    // Load OpenGL functions
    gl::load_with(|symb| window.get_proc_address(symb));

    // Return Engine and EventHandler
    (
        engine::init(window), 
        event_handler::init(events, glfw)
    )
}