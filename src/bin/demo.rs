extern crate gl;
extern crate lib;
extern crate sdl2;

use lib::types::buffer::ebo::EBO;
use lib::types::buffer::vao_builder::VAOBuilder;
use lib::types::buffer::vbo::VBO;
use lib::types::data::data_layout::DataLayout;
use lib::types::shader::shader::Shader;
use lib::types::shader::shader_program::ShaderProgram;
use lib::*;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::ffi::c_void;
use std::time::SystemTime;

fn main() {
    let vertex_shader_source = load_file("shaders/vertex_shader.glsl");
    let fragment_shader_source = load_file("shaders/fragment_shader.glsl");

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
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const c_void);
    //Set the viewport and color
    unsafe {
        gl::Viewport(0, 0, 900, 700);
    }

    //Create shader
    let vertex_shader = Shader::from_source(vertex_shader_source, gl::VERTEX_SHADER).unwrap();
    let fragment_shader = Shader::from_source(fragment_shader_source, gl::FRAGMENT_SHADER).unwrap();
    let shader_program = ShaderProgram::link([&vertex_shader, &fragment_shader].as_ref()).unwrap();
    let horizontal_offset = shader_program.uniform_from_str("horizontalOffset").unwrap();
    let vertical_offset = shader_program.uniform_from_str("verticalOffset").unwrap();

    //Create vertices
    let vertices: [f32; 18] = [
        // positions         // colors
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom le t
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    ];
    let data_layout = DataLayout::infer_from_f32slice(&vertices, &[3], gl::FALSE, 3);
    let indices: [u32; 3] = [0, 1, 2];

    let (vao, _vbo, ebo) =
        VAOBuilder::from_vbo(VBO::gen_buffer(), &vertices, gl::STATIC_DRAW, data_layout)
            .add_ebo(EBO::gen_buffer(), &indices, gl::STATIC_DRAW)
            .compile();
    let _ebo = ebo.unwrap();

    let mut wireframe = false;
    let now = SystemTime::now();
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
            shader_program.gl_use();
            shader_program.uniform1f(
                &horizontal_offset,
                now.elapsed().unwrap().as_secs_f32().sin() / 2.,
            );
            shader_program.uniform1f(
                &vertical_offset,
                (now.elapsed().unwrap().as_secs_f32() * 1.414).cos() / 2.,
            );
            vao.bind();
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
