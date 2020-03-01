use gl::types::*;

pub type VAO = VertexArrayObject;
pub struct VertexArrayObject {
    pub id: GLuint,
}
impl VAO {
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
impl Default for VAO {
    fn default() -> Self {
        let mut id = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        VAO { id }
    }
}
impl Drop for VAO {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}
