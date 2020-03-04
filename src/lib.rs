extern crate gl;
extern crate image;
extern crate sdl2;

pub mod setup;
pub mod types;

pub fn load_file(path: &str) -> String {
    let contents = std::fs::read_to_string(path);
    match contents {
        Ok(s) => s,
        Err(e) => panic!("{}", e.to_string()),
    }
}
