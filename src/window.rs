/// lib for wrapper around sdl2

extern crate sdl2; // importing all the modules from sdl2
use std::error::Error; // for the dynamic error

/// struct for the window screen size
pub struct Window {
    pub height: i32,
    pub width: i32,
    pub sdl: sdl2::Sdl,
}

/// implementaion for the windows screen size
impl Window {

    /// creates a new windowsize instance
    pub fn new(width: i32, height: i32) -> Result<Window, &'static str> {
        let width = width;
        let height = height;
        
        // windows size sud be between size of 800
        // Needs be appllied later:
        //  - Change the scale restriction here
        //  - User can change the window size only during windowed mode
        //  - User can implement the full screen mode
        if width > 800 && height > 800 {
            return Err("Windows size is too big");
        }

        // creating sdl2 instance to interact with openGL  
        let sdl = sdl2::init().unwrap();
        
        // return ok as the result generic when the window size is not too large
        Ok(Window {
            width,
            height,
            sdl
        })
    } 
    
    // create a window screen based on the windowsize
    pub fn spawn_window(&self)-> Result<(), Box<dyn Error>> {
        
        // creating sdl2 instance to interact with openGL  
        let _sdl = sdl2::init()?; // return error to the client side, so user can handle it depending on the error

        // return sdl object to  ok, and making sure the function ran without any error
        Ok(())
        
    }
}
