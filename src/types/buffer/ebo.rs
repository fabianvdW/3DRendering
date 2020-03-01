use gl::types::*;
use std::ffi::c_void;

pub type EBO = ElementBufferObject;
pub struct ElementBufferObject {
    pub id: GLuint,
}
impl EBO {
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.id) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) }
    }
    pub fn buffer_data(&self, indices: &[u32], draw_type: GLenum) {
        unsafe {
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                4 * indices.len() as isize,
                indices.as_ptr() as *const c_void,
                draw_type,
            )
        }
    }
    pub fn delete(self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}
impl Default for EBO {
    fn default() -> Self {
        let mut id = 0;
        unsafe { gl::GenBuffers(1, &mut id) }
        EBO { id }
    }
}
impl Drop for EBO {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }
    }
}
