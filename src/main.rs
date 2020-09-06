use procedural_generations::window;
/// main function for using sdl2 library wrapper
fn main() {
    // creating a new instance of the window
    let new_window = window::WindowDisplay::new(600, 600).unwrap();

    // keep the window alive 
    let display_window = new_window.spawn_window().unwrap();     
}
