use gl::types::*;
use std::ffi::c_void;

pub type VBO = VertexBufferObject;

pub struct VertexBufferObject {
    pub id: GLuint,
}
impl VBO {
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, self.id) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) }
    }
    pub fn buffer_data(&self, vertices: &[f32], draw_type: GLenum) {
        debug_assert!([gl::STATIC_DRAW, gl::DYNAMIC_DRAW].contains(&draw_type));
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
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}
impl Default for VBO {
    fn default() -> Self {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        VBO { id }
    }
}
impl Drop for VBO {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}
