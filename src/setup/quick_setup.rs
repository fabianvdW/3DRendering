use crate::types::linalg::dimension::Dimension;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Scancode;
use sdl2::video::{GLContext, Window};
use sdl2::Sdl;
use std::ffi::c_void;

pub struct Demo {
    sdl: Sdl,
    window: Window,
    _gl_context: GLContext, // Make sure that current gl_context isn't dropped
}
pub fn initialize_demo(title: &str, dimensions: Dimension) -> Demo {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);
    let window = video_subsystem
        .window(title, dimensions.rows as u32, dimensions.columns as u32)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    window.gl_set_context_to_current().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const c_void);
    unsafe {
        gl::Viewport(0, 0, dimensions.rows as i32, dimensions.columns as i32);
    }
    Demo {
        sdl,
        window,
        _gl_context: gl_context,
    }
}
pub fn quick_demo<F, G>(demo: Demo, mut render_loop: F, event_soaker: G)
where
    F: FnMut(),
    G: Fn(Event),
{
    let sdl = demo.sdl;
    let window = demo.window;
    let mut event_pump = sdl.event_pump().unwrap();
    let mut wireframe = false;
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    scancode: Some(Scancode::Escape),
                    ..
                } => break 'main,
                Event::KeyDown {
                    scancode: Some(Scancode::Space),
                    ..
                } => {
                    wireframe = !wireframe;
                    unsafe {
                        gl::PolygonMode(
                            gl::FRONT_AND_BACK,
                            if wireframe { gl::LINE } else { gl::FILL },
                        );
                    }
                }
                _ => {
                    event_soaker(event.clone());
                    housekeeping(event);
                }
            }
        }
        render_loop();
        window.gl_swap_window();
    }
}

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
