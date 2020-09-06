use super::*;
use gl;
/// Uses SDL to create OpenGL context
pub fn opengl_context(window: &sdl2::video::Window) {
    //create opengl context instance
    let _gl_context = window.gl_create_context().unwrap_or_else(|error| {
        panic!("Error creating gl context: {:?}", error);
    });
}
