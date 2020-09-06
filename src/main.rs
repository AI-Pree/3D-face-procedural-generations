use procedural_generations::window;
use sdl2::event::Event;


/// main function for using sdl2 library wrapper
fn main() {
    // dimension of the window
    let height: u32 = 600;
    let width: u32  = 600;

    let new_window                             = window::WindowDisplay::new(height, width).unwrap(); // creating new instance of the window
    let (sdl, display_window, video_subsystem) = new_window.spawn_window().unwrap(); // generate core elements for interacting with openGl
    let _gl_context                            = display_window.gl_create_context().unwrap(); // create openGl context
    let mut event_pump                         = sdl.event_pump().unwrap(); // creating a event loop
        
    // load OpenGl function pointer
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void); 
    
    unsafe {
        gl::Viewport(0, 0, height as i32, width as i32); // set viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0); // color to display after the buffer is cleared
    }
 
    // using loop to keep window alive throughout the session
    'running: loop {
        for event in event_pump.poll_iter() {
            // handle use input
            match event {
                Event::Quit {..} => break 'running,
                _ => {},
            }
        }
        // clears the  color buffer in the framebuffer and uses the color in gl::ClearColor
        // when gl::COLOR_BUFFER_BIT bit is called
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        display_window.gl_swap_window(); // swaps the buffer to display the current context in the framebuffer
    }
}
