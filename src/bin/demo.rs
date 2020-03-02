extern crate lib;
extern crate rand;

use lib::types::buffer::ebo::EBO;
use lib::types::buffer::vao_builder::VAOBuilder;
use lib::types::buffer::vbo::VBO;
use lib::types::data::data_layout::DataLayout;
use lib::types::linalg::dimension::Dimension;
use lib::types::linalg::matrix::Matrix;
use lib::types::shader::shader::Shader;
use lib::types::shader::shader_program::ShaderProgram;
use lib::types::shader::texture_builder::TextureBuilder;
use lib::*;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;
use std::ffi::c_void;
use std::time::{Instant, SystemTime};

fn main() {
    let mut a = Vec::with_capacity(1440 * 1440);
    let mut rng = rand::thread_rng();
    for _ in 0..(1440 * 1440) {
        a.push(rng.gen_range(0f32, 1f32));
    }
    let matrix = Matrix::from_data(a, Dimension::new(1440, 1440));
    let mut output = Matrix::from_closure(|_| 0. as f32, Dimension::new(1440, 1440));
    let now = Instant::now();
    output.buffered_mul(&matrix, &matrix);
    let millis = now.elapsed().as_millis() as f32 / 1000.;
    println!("Took: {}", millis);
    let vertex_shader_source = load_file("shaders/vertex_shader.glsl");
    let fragment_shader_source = load_file("shaders/fragment_shader.glsl");

    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

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
    let texture0 = shader_program.uniform_from_str("texture0").unwrap();
    let texture1 = shader_program.uniform_from_str("texture1").unwrap();
    let mix_p = shader_program.uniform_from_str("mix_p").unwrap();
    let mut mix_p_val = 0.2;

    //Create textures
    let container = TextureBuilder::default()
        .compile("textures/container.jpg")
        .unwrap();
    let smiley = TextureBuilder::default()
        .compile("textures/awesomeface.png")
        .unwrap();
    shader_program.gl_use();
    shader_program.uniform1i(&texture0, 0);
    shader_program.uniform1i(&texture1, 1);

    //Create vertices
    let vertices: [f32; 32] = [
        // positions          // colors           // texture coords
        0.5, 0.5, 0.0, 1.0, 0.0, 0.0, 1., 1., // top right
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1., 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, // bottom left
        -0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 0.0, 1., // top left
    ];
    let data_layout = DataLayout::infer_from_f32slice(&vertices, &[3, 6], gl::FALSE, 4);
    let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

    let (vao, _vbo, ebo) =
        VAOBuilder::from_vbo(VBO::default(), &vertices, gl::STATIC_DRAW, data_layout)
            .add_ebo(EBO::default(), &indices, gl::STATIC_DRAW)
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
                Event::KeyDown {
                    scancode: Some(Scancode::Up),
                    ..
                } => mix_p_val += 0.02,
                Event::KeyDown {
                    scancode: Some(Scancode::Down),
                    ..
                } => mix_p_val -= 0.02,
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
                (now.elapsed().unwrap().as_secs_f32() * 1.414).sin() / 2.,
            );
            shader_program.uniform1f(&mix_p, mix_p_val);
            container.bind(0);
            smiley.bind(1);
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
