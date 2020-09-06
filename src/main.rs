use procedural_generations::window;
use sdl2::event::Event;


/// main function for using sdl2 library wrapper
fn main() {
    // creating a new instance of the window
    let new_window = window::WindowDisplay::new(600, 600).unwrap();

    // keep the window alive 
    let (sdl, display_window, video_subsystem) = new_window.spawn_window().unwrap();
       
    // create opengl context
    let _gl_context = display_window.gl_create_context().unwrap();
  
    // creating an event loop
    let mut event_pump = sdl.event_pump().unwrap();
        
    // load OpenGl function pointer
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void); 
    
    unsafe {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0); 
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
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        display_window.gl_swap_window(); // swaps the buffer to display the current context in the buffer
    }
}
