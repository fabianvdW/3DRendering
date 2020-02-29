extern crate gl;
extern crate lib;
extern crate sdl2;

use lib::*;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::ffi::{c_void, CString};

fn main() {
    let vertex_shader_source = CString::new(load_file("shaders/vertex_shader.glsl")).unwrap();
    let fragment_shader_source = CString::new(load_file("shaders/fragment_shader.glsl")).unwrap();

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
    }

    //Create shader
    let vertex_shader = shader_from_source(&vertex_shader_source, gl::VERTEX_SHADER).unwrap();
    let fragment_shader = shader_from_source(&fragment_shader_source, gl::FRAGMENT_SHADER).unwrap();
    let shader_program = link_shaders(&[vertex_shader, fragment_shader]).unwrap();

    //Create vertices
    let vertices: [f32; 12] = [
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
    ];
    let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            4 * vertices.len() as isize,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            4 * indices.len() as isize,
            indices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindVertexArray(0);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
    }
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
                    housekeeping(event);
                }
            }
        }
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::TRIANGLES,
                indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        window.gl_swap_window();
    }
}
