use crate::types::ebo::EBO;
use crate::types::vao::VAO;
use crate::types::vbo::VBO;
use gl::types::*;

pub type VAOBuilder<'a> = VertexArrayObjectBuilder<'a>;

pub struct VertexArrayObjectBuilder<'a> {
    pub vbo: VBO,
    pub vbo_data: &'a [f32],
    pub vbo_draw_type: GLenum,
    pub ebo: Option<EBO>,
    pub ebo_data: Option<&'a [u32]>,
    pub ebo_draw_type: Option<GLenum>,
}
impl<'a> VertexArrayObjectBuilder<'a> {
    pub fn from_vbo(vbo: VBO, vbo_data: &'a [f32], vbo_draw_type: GLenum) -> Self {
        VertexArrayObjectBuilder {
            vbo,
            vbo_data,
            vbo_draw_type,
            ebo: None,
            ebo_data: None,
            ebo_draw_type: None,
        }
    }
    pub fn add_ebo(mut self, ebo: EBO, ebo_data: &'a [u32], ebo_draw_type: GLenum) -> Self {
        self.ebo = Some(ebo);
        self.ebo_data = Some(ebo_data);
        self.ebo_draw_type = Some(ebo_draw_type);
        self
    }
    pub fn compile(self) -> (VAO, VBO, Option<EBO>) {
        let vao = VAO::gen_buffer();
        vao.bind();

        self.vbo.bind();
        self.vbo.buffer_data(self.vbo_data, self.vbo_draw_type);

        if self.ebo.is_some() {
            let ebo = self.ebo.as_ref().unwrap();
            ebo.bind();
            ebo.buffer_data(self.ebo_data.unwrap(), self.ebo_draw_type.unwrap());
        }
        unsafe {
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * 4, std::ptr::null());
            gl::EnableVertexAttribArray(0);
        }
        vao.unbind();
        self.vbo.unbind();
        if self.ebo.is_some() {
            self.ebo.as_ref().unwrap().unbind();
        }
        (vao, self.vbo, self.ebo)
    }
}
