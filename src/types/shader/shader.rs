use gl::types::*;
use std::ffi::CString;

pub struct Shader {
    pub id: GLuint,
    pub kind: GLenum,
}
impl Shader {
    pub fn from_source(source: String, kind: GLenum) -> Result<Self, String> {
        debug_assert!([gl::VERTEX_SHADER, gl::FRAGMENT_SHADER].contains(&kind));
        let source = CString::new(source).unwrap();
        unsafe {
            let id = gl::CreateShader(kind);
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
            let mut success: gl::types::GLint = 1;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut len: gl::types::GLint = 0;
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let buffer = vec![0u8; len as usize];
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), buffer.as_ptr() as *mut i8);
                Err(CString::from_vec_unchecked(buffer).into_string().unwrap())
            } else {
                Ok(Shader { id, kind })
            }
        }
    }
    pub fn attach(&self, id: GLuint) {
        unsafe { gl::AttachShader(id, self.id) }
    }
    pub fn detach(&self, id: GLuint) {
        unsafe { gl::DetachShader(id, self.id) }
    }
    pub fn delete(self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) }
    }
}
