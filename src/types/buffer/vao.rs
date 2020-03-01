use gl::types::*;

pub type VAO = VertexArrayObject;
pub struct VertexArrayObject {
    pub id: GLuint,
}
impl VertexArrayObject {
    pub fn gen_buffer() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VAO { id }
    }
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) }
    }
    pub fn delete(self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) }
    }
}
impl Drop for VertexArrayObject {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
