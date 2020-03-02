use crate::types::linalg::dimension::Dimension;
use crate::types::linalg::matrix::Matrix;
use crate::types::shader::shader::Shader;
use crate::types::shader::uniform::Uniform;
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

    pub fn uniform_from_str(&self, s: &str) -> Result<Uniform, String> {
        let cstr = CString::new(s).unwrap();
        let id = unsafe { gl::GetUniformLocation(self.id, cstr.as_ptr()) };
        if id == -1 {
            Err("Uniform not found!".to_owned())
        } else {
            Ok(Uniform { id })
        }
    }
    pub fn uniform1i(&self, uniform: &Uniform, i1: i32) {
        //TODO Design decision: Make sure shader program is active? Requires internal "active" field and mutability
        unsafe { gl::Uniform1i(uniform.id, i1) }
    }
    pub fn uniform1f(&self, uniform: &Uniform, f1: f32) {
        //TODO Design decision: Make sure shader program is active? Requires internal "active" field and mutability.
        unsafe { gl::Uniform1f(uniform.id, f1) }
    }
    pub fn uniform4f(&self, uniform: &Uniform, f1: f32, f2: f32, f3: f32, f4: f32) {
        //TODO Design decision: Make sure shader program is active? Requires internal "active" field and mutability.
        unsafe { gl::Uniform4f(uniform.id, f1, f2, f3, f4) }
    }
    pub fn uniform4fv(&self, uniform: &Uniform, mat: &Matrix<f32>) {
        //TODO Design decision: Make sure shader program is active? Requires internal "active" field and mutability.
        debug_assert!(mat.dimension == Dimension::new(4, 4));
        unsafe { gl::Uniform4fv(uniform.id, 1, mat.as_ptr()) }
    }
}
impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) }
    }
}
