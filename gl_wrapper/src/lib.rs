//! Procedural Generation
//!
//! A library for generating procedural terrains models.
//! It uses marching cube algorithm to determine the vertex points
//! of the models

// modules for the lib
pub mod window;
pub mod render_gl;

// Unit testing for the window module
// all the unit test for the window module
#[cfg(test)]
mod tests {
    use super::*;
   
    /// testing when the window size is in the given margin
    #[test]
    fn should_create_right_size_window() {
        let display_window = window::WindowDisplay::new(700, 700).is_err();
        assert!(!display_window);
    } 

    /// testing if the error is returned when input window size larger than 800
    #[test]
    fn should_not_create_large_window_size() {
        let display_window = window::WindowDisplay::new(900, 900).is_err(); //creates the new instance of the display window
        assert!(display_window);
    }
    
    /// testing if the new window is spawned
    #[test]
    #[ignore]
    fn new_window_has_spawned() {
        let display_window = window::WindowDisplay::new(400, 400).unwrap(); //creates the new instance of the display window
        assert!(!display_window.spawn_window().is_err());
    }
   
}
