extern crate gl;
extern crate lib;
extern crate sdl2;

use lib::*;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("3D Rendering", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    window.gl_set_context_to_current().unwrap();

    //Load OpenGL functions
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    //Set the viewport and color
    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT)
    }

    let vertices = [(-0.5, -0.5, 0.), (0.5, -0.5, 0.), (0., 0.5, 0.)];

    let mut vbo = 0;
    unsafe {
        gl::GenBuffers(2, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    }
    println!("{}", vbo);
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => break 'main,
                _ => {
                    housekeeping(event);
                }
            }
        }

        window.gl_swap_window();
    }
}
