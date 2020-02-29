use crate::types::shader::Shader;
use gl::types::*;
use std::ffi::CString;

pub struct ShaderProgram {
    pub id: GLuint,
}
impl ShaderProgram {
    pub fn link(shaders: &[&Shader]) -> Result<Self, String> {
        unsafe {
            let id = gl::CreateProgram();
            for shader in shaders.iter() {
                shader.attach(id);
            }
            gl::LinkProgram(id);
            for shader in shaders.iter() {
                shader.detach(id);
            }
            let mut success = 1;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut len = 0;
                gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
                let buffer = vec![0u8; len as usize];
                gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), buffer.as_ptr() as *mut i8);
                Err(CString::from_vec_unchecked(buffer).into_string().unwrap())
            } else {
                Ok(ShaderProgram { id })
            }
        }
    }

    pub fn gl_use(&self) {
        unsafe { gl::UseProgram(self.id) }
    }

    pub fn delete(self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}
impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}
