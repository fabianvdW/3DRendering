use gl::types::*;
use std::ffi::c_void;

pub type VBO = VertexBufferObject;

pub struct VertexBufferObject {
    pub id: GLuint,
}
impl VertexBufferObject {
    pub fn gen_buffer() -> Self {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        VertexBufferObject { id }
    }
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) }
    }
    pub fn buffer_data(&self, vertices: &[f32], draw_type: GLenum) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                4 * vertices.len() as isize,
                vertices.as_ptr() as *const c_void,
                draw_type,
            )
        }
    }
    pub fn delete(self) {
        unsafe { gl::DeleteBuffers(1, self.id as *const GLuint) }
    }
}
impl Drop for VertexBufferObject {
    fn drop(&mut self) {
        println!("Dropping VBO");
        unsafe { gl::DeleteBuffers(1, self.id as *const GLuint) }
        println!("Done Dropping VBO");
    }
}
