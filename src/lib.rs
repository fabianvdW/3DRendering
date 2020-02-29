use gl::types::*;
use sdl2::event::{Event, WindowEvent};
use std::ffi::{CStr, CString};

pub fn housekeeping(event: Event) {
    if let Event::Window { win_event, .. } = event {
        match win_event {
            WindowEvent::Resized(new_x, new_y) | WindowEvent::SizeChanged(new_x, new_y) => {
                unsafe { gl::Viewport(0, 0, new_x, new_y) };
            }
            _ => {}
        }
    }
}
pub fn load_file(path: &str) -> String {
    let contents = std::fs::read_to_string(path);
    match contents {
        Ok(s) => s,
        Err(e) => panic!("{}", e.to_string()),
    }
}
pub fn link_shaders(shaders: &[GLuint]) -> Result<GLuint, String> {
    unsafe {
        let id = gl::CreateProgram();
        for shader in shaders {
            gl::AttachShader(id, *shader);
        }
        gl::LinkProgram(id);
        let mut success = 1;
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
        if success == 0 {
            let mut len = 0;
            gl::GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let buffer = vec![0u8; len as usize];
            gl::GetProgramInfoLog(id, len, std::ptr::null_mut(), buffer.as_ptr() as *mut i8);
            Err(CString::from_vec_unchecked(buffer).into_string().unwrap())
        } else {
            for shader in shaders {
                gl::DeleteShader(*shader);
            }
            Ok(id)
        }
    }
}
pub fn shader_from_source(source: &CStr, kind: GLenum) -> Result<GLuint, String> {
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
            Ok(id)
        }
    }
}
