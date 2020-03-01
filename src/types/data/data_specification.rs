use gl::types::*;
#[derive(Clone)]
pub struct DataSpecification {
    pub stride: GLuint,
    pub components: GLint,
    pub normalize: GLboolean,
}