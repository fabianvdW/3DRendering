use gl::types::*;
use std::os::raw::c_void;

pub struct Texture {
    pub id: GLuint,
    pub kind: GLenum,
}
impl Texture {
    pub fn from_kind(kind: GLenum) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Texture { id, kind }
    }
    pub fn bind(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + unit);
            gl::BindTexture(self.kind, self.id);
        }
    }
    pub fn delete(self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
    pub fn tex_image2d(&self, width: GLuint, height: GLuint, data: &[u8], typ: GLenum) {
        //TODO Design decision: Enforce bind before? Requires inner mutability
        debug_assert!(self.kind == gl::TEXTURE_2D);
        unsafe {
            gl::TexImage2D(
                self.kind,
                0,
                gl::RGB as GLint,
                width as GLint,
                height as GLint,
                0,
                typ,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const c_void,
            )
        }
    }
    pub fn generate_mipmap(&self) {
        unsafe { gl::GenerateMipmap(self.kind) }
    }
}
impl Default for Texture {
    fn default() -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        Texture {
            id,
            kind: gl::TEXTURE_2D,
        }
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
