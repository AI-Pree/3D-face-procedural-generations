// lib for wrapper around sdl2
use std::error::Error;

/// struct for the window screen size
#[derive(Copy, Clone, Debug)]
pub struct WindowDisplay {
    pub height: u32,
    pub width: u32,
}

/// implementaion for the windows screen size
impl WindowDisplay {

    /// creates a new windowsize instance
    pub fn new(width: u32, height: u32) -> Result<WindowDisplay, &'static str> {
        let width = width;
        let height = height;
        
        // windows size sud be less than size of 800
        // Needs to be applied later:
        //  - Change the scale restriction here
        //  - User can change the window size only during windowed mode
        //  - User can implement the full screen mode
        if width > 800 && height > 800 {
            return Err("Windows size is too big");
        }
       
        // return ok as the result generic when the window size is not too large
        Ok(WindowDisplay {
            width,
            height,
        })
    } 
    
    /// create a window screen based on the windowsize
    pub fn spawn_window(&self) -> Result<(sdl2::Sdl, sdl2::video::Window, sdl2::VideoSubsystem), Box<dyn Error>> {
       
        // creating sdl2 instance to interact with openGL  
        let sdl = sdl2::init().unwrap();
         
        // initialise the video subsystem
        let video_subsystem = sdl.video().unwrap(); 
        
        // creating a window instance to display here
        let window = video_subsystem
            .window("ProcGen", self.width, self.height) // returns result with WindowBuilder type
            .resizable()    // set the window to be resizable
            .opengl()       // sets the window to be usable with the openGL context
            .build()?;      // builds the window and throws the error in Result
     
        Ok((sdl, window, video_subsystem))
    }
}

pub mod draw;
