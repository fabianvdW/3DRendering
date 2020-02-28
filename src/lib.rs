use sdl2::event::EventType::Window;
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

pub fn shader_from_source(
    source: &CStr,
    kind: gl::types::GLenum,
) -> Result<gl::types::GLuint, String> {
    unsafe {
        let id = gl::CreateShader(kind);
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
        let mut success: gl::types::GLint = 1;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
            buffer.extend([b' '].iter().cycle().take(len as usize));
            let error: CString = CString::from_vec_unchecked(buffer);
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
            Err(error.to_string_lossy().into_owned())
        } else {
            Ok(id)
        }
    }
}
