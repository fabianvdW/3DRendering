extern crate lib;

use lib::load_file;
use lib::setup::quick_setup::initialize_demo;
use lib::types::buffer::ebo::EBO;
use lib::types::buffer::vao::VAO;
use lib::types::buffer::vao_builder::VAOBuilder;
use lib::types::buffer::vbo::VBO;
use lib::types::data::data_layout::DataLayout;
use lib::types::linalg::dimension::Dimension;
use lib::types::linalg::matrix::Matrix;
use lib::types::shader::shader::Shader;
use lib::types::shader::shader_program::ShaderProgram;
use std::time::SystemTime;

pub const SIERPINSKI_DEPTH: usize = 10;
pub fn main() {
    let demo = initialize_demo("Sierpinski Cpu", Dimension::new(900, 700));
    let vertex_shader = Shader::from_source(
        load_file("shaders/sierpinski_cpu_vertex.glsl"),
        gl::VERTEX_SHADER,
    )
    .unwrap();
    let fragment_shader = Shader::from_source(
        load_file("shaders/sierpinski_cpu_fragment.glsl"),
        gl::FRAGMENT_SHADER,
    )
    .unwrap();
    let shader_program = ShaderProgram::link(&[&vertex_shader, &fragment_shader]).unwrap();

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    sierpinski_triangle(
        &mut vertices,
        &mut indices,
        SIERPINSKI_DEPTH,
        Matrix::from_data(vec![-1., -1.0, 0., 1.], Dimension::new(4, 1)),
        Matrix::from_data(vec![1.0, -1.0, 0., 1.], Dimension::new(4, 1)),
        Matrix::from_data(vec![0., 1.0, 0., 1.], Dimension::new(4, 1)),
        0.,
    );
    let data_layout =
        DataLayout::infer_from_f32slice(&vertices, &[3], gl::FALSE, vertices.len() / 6);

    let (vao, vbo, _ebo): (VAO, VBO, Option<EBO>) =
        VAOBuilder::from_vbo(VBO::default(), &vertices, gl::DYNAMIC_DRAW, data_layout)
            .add_ebo(EBO::default(), &indices, gl::DYNAMIC_DRAW)
            .compile();
    let now = SystemTime::now();
    let mut curr_time = SystemTime::now();
    lib::setup::quick_setup::quick_demo(
        demo,
        || {
            let elapsed_time = now.elapsed().unwrap().as_secs_f32();
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
                vertices.clear();
                indices.clear();
                sierpinski_triangle(
                    &mut vertices,
                    &mut indices,
                    SIERPINSKI_DEPTH,
                    Matrix::from_data(vec![-1., -1.0, 0., 1.], Dimension::new(4, 1)),
                    Matrix::from_data(vec![1.0, -1.0, 0., 1.], Dimension::new(4, 1)),
                    Matrix::from_data(vec![0., 1.0, 0., 1.], Dimension::new(4, 1)),
                    elapsed_time,
                );
                vbo.bind();
                vbo.buffer_sub_data(&vertices);
                shader_program.gl_use();
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
    color_signal: f32,
) {
    if depth == 0 {
        return;
    }
    let mut new_bottom = (&bottom_left + &bottom_right) * 0.5;
    let mut new_left = (&bottom_left + &top) * 0.5;
    let mut new_right = (&top + &bottom_right) * 0.5;
    sierpinski_triangle(
        vertices,
        indices,
        depth - 1,
        bottom_left.clone(),
        new_bottom.clone(),
        new_left.clone(),
        color_signal,
    );
    sierpinski_triangle(
        vertices,
        indices,
        depth - 1,
        new_left.clone(),
        new_right.clone(),
        top,
        color_signal,
    );
    sierpinski_triangle(
        vertices,
        indices,
        depth - 1,
        new_bottom.clone(),
        bottom_right.clone(),
        new_right.clone(),
        color_signal,
    );
    if new_bottom.data[0] != 0. {
        let mid_point = (&new_bottom + &((&new_left + &new_right) * 0.5)) * 0.5;
        let matrix = Matrix::<f32>::identity4()
            .translate4(mid_point.data[0], mid_point.data[1], mid_point.data[2])
            .rotate4(0., 1., 0., color_signal)
            .translate4(-mid_point.data[0], -mid_point.data[1], -mid_point.data[2]); //Only need to align x-axes for y-only-rotation
        new_bottom = &matrix * &new_bottom;
        new_right = &matrix * &new_right;
        new_left = &matrix * &new_left;
    }
    let len = indices.len() as u32;
    vertices.extend_from_slice(&new_right.data[0..3]);
    let mult = 5.;
    vertices.push((mult * (mult * new_right.data[0] + color_signal)).sin() / 2. + 0.5);
    vertices.push((mult * (mult * new_right.data[1] + color_signal)).cos() / 2. + 0.5);
    vertices.push((1.41 * (new_right.data[0] + color_signal)).sin() / 2. + 0.5);
    vertices.extend_from_slice(&new_bottom.data[0..3]);
    vertices.push((new_bottom.data[0] + color_signal).sin() / 2. + 0.5);
    vertices.push((new_bottom.data[1] + color_signal).cos() / 2. + 0.5);
    vertices.push((1.41 * (new_bottom.data[0] + color_signal)).sin() / 2. + 0.5);
    vertices.extend_from_slice(&new_left.data[0..3]);
    vertices.push((new_left.data[0] + color_signal).sin() / 2. + 0.5);
    vertices.push((new_left.data[1] + color_signal).cos() / 2. + 0.5);
    vertices.push((1.41 * (new_left.data[0] + color_signal)).sin() / 2. + 0.5);
    indices.push(len);
    indices.push(len + 1);
    indices.push(len + 2);
}
