use sdl2::event::{Event, WindowEvent};

pub mod types;

pub fn housekeeping(event: Event) {
    if let Event::Window { win_event, .. } = event {
        match win_event {
            WindowEvent::Resized(new_x, new_y) | WindowEvent::SizeChanged(new_x, new_y) => {
                unsafe { gl::Viewport(0, 0, new_x, new_y) };
            }
            _ => {}
        }
    }
}
pub fn load_file(path: &str) -> String {
    let contents = std::fs::read_to_string(path);
    match contents {
        Ok(s) => s,
        Err(e) => panic!("{}", e.to_string()),
    }
}
