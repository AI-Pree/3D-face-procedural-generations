/// module for the window 
pub mod window;

/// Unit testing for the window module
/// all the unit test for the window module
#[cfg(test)]
mod tests {
    use super::*;

    /// testing if the error is returned when input window size larger than 800
    #[test]
    fn should_not_create_large_window_size() {
        let display_window = window::WindowDisplay::new(900, 900).is_err(); //creates the new instance of the display window
        assert!(display_window);
    }
    
    /// testing when the window size if in the given margin
    #[test]
    fn should_create_right_size_window() {
        assert!(!(window::WindowDisplay::new(700, 700).is_err()));
    }

}
