use lib::load_file;
use lib::setup::quick_setup::{initialize_demo, quick_demo};
use lib::types::buffer::ebo::EBO;
use lib::types::buffer::vao_builder::VAOBuilder;
use lib::types::buffer::vbo::VBO;
use lib::types::data::data_layout::DataLayout;
use lib::types::linalg::dimension::Dimension;
use lib::types::linalg::matrix::Matrix;
use lib::types::shader::shader::Shader;
use lib::types::shader::shader_program::ShaderProgram;
use std::time::SystemTime;

pub const SIERPINSKI_DEPTH: usize = 9;

pub fn main() {
    let demo = initialize_demo("Sierpinski GPU", Dimension::new(900, 700));
    let vertex_shader = Shader::from_source(
        load_file("./shaders/sierpinski_gpu_vertex.glsl"),
        gl::VERTEX_SHADER,
    )
    .unwrap();
    let fragment_shader = Shader::from_source(
        load_file("./shaders/sierpinski_gpu_fragment.glsl"),
        gl::FRAGMENT_SHADER,
    )
    .unwrap();
    let shader_program = ShaderProgram::link(&[&vertex_shader, &fragment_shader]).unwrap();
    let iterations = shader_program.uniform_from_str("iterations").unwrap();
    let rotate = shader_program.uniform_from_str("rotationMatrix").unwrap();
    let noise = shader_program.uniform_from_str("noise").unwrap();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    sierpinski_triangle(
        &mut vertices,
        &mut indices,
        SIERPINSKI_DEPTH,
        Matrix::<f32>::from_data(vec![-1.0, -1.0], Dimension::new(2, 1)),
        Matrix::<f32>::from_data(vec![1.0, -1.0], Dimension::new(2, 1)),
        Matrix::<f32>::from_data(vec![0., 1.0], Dimension::new(2, 1)),
    );
    let data_layout =
        DataLayout::infer_from_f32slice(&vertices, &[], gl::FALSE, vertices.len() / 2);
    let (vao, _vbo, _ebo) =
        VAOBuilder::from_vbo(VBO::default(), &vertices, gl::STATIC_DRAW, data_layout)
            .add_ebo(EBO::default(), &indices, gl::STATIC_DRAW)
            .compile();

    let now = SystemTime::now();
    let mut curr_time = SystemTime::now();
    quick_demo(
        demo,
        || {
            let elapsed_time = now.elapsed().unwrap().as_secs_f32();
            let rotation_matrix = Matrix::<f32>::identity4().rotate4(0.0, 1.0, 0.0, elapsed_time);
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                shader_program.gl_use();
                shader_program.uniform_matrix4fv(&rotate, &rotation_matrix);
                shader_program.uniform1f(&noise, elapsed_time);
                shader_program.uniform1i(&iterations, SIERPINSKI_DEPTH as i32);
                vao.bind();
                gl::DrawElements(
                    gl::TRIANGLES,
                    indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }
            println!(
                "Current frame took: {}",
                curr_time.elapsed().unwrap().as_millis()
            );
            curr_time = SystemTime::now();
        },
        |_| {},
    );
}

pub fn sierpinski_triangle(
    vertices: &mut Vec<f32>,
    indices: &mut Vec<u32>,
    depth: usize,
    bottom_left: Matrix<f32>,
    bottom_right: Matrix<f32>,
    top: Matrix<f32>,
) {
    if depth == 0 {
        return;
    }
    let new_bottom = (&bottom_left + &bottom_right) * 0.5;
    let new_left = (&bottom_left + &top) * 0.5;
    let new_right = (&top + &bottom_right) * 0.5;
    sierpinski_triangle(
        vertices,
        indices,
        depth - 1,
        bottom_left,
        new_bottom.clone(),
        new_left.clone(),
    );
    sierpinski_triangle(
        vertices,
        indices,
        depth - 1,
        new_left.clone(),
        new_right.clone(),
        top,
    );
    sierpinski_triangle(
        vertices,
        indices,
        depth - 1,
        new_bottom.clone(),
        bottom_right,
        new_right.clone(),
    );
    let len = indices.len() as u32;
    vertices.extend_from_slice(&new_right.data[0..2]);
    vertices.extend_from_slice(&new_bottom.data[0..2]);
    vertices.extend_from_slice(&new_left.data[0..2]);
    indices.push(len);
    indices.push(len + 1);
    indices.push(len + 2);
}
