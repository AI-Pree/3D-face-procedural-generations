/// lib for wrapper around sdl2

use sdl2; // importing all the modules from sdl2

/// struct for the window screen size
pub struct Window {
    pub height: u32,
    pub width: u32,
    pub sdl: sdl2::Sdl,
}

/// implementaion for the windows screen size
impl Window {

    /// creates a new windowsize instance
    pub fn new(width: u32, height: u32) -> Result<Window, &'static str> {
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
    pub fn spawn_window(&self) -> Result<sdl2::video::Window, sdl2::video::WindowBuildError> {
        // initialise the video subsystem
        let video_subsystem = (self.sdl).video().unwrap();
             
        // creating a window instance to display here
        let window = video_subsystem
            .window("ProcGen", self.width, self.height) // returns result with WindowBuilder type
            .resizable()    // set the window to be resizable
            .opengl()       // sets the window to be usable with the openGL context
            .build()        // builds the window
            .unwrap();

        Ok(window)          // return result with Ok of window type in sdl2 video
    }
}
