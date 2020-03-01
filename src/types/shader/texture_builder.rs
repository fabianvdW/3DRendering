use crate::types::shader::texture::Texture;
use gl::types::*;
use image::{GenericImageView, ImageError};

pub struct TextureBuilder {
    kind: GLenum,
    texture_wrapping_s: GLint,
    texture_wrapping_t: GLint,
    texture_wrapping_r: GLint,
    border_color: Option<[f32; 4]>,
    texture_min_filter: GLint,
    texture_mag_filter: GLint,
    generate_mipmaps: bool,
}
impl TextureBuilder {
    pub fn kind(mut self, kind: GLenum) -> Self {
        debug_assert!([gl::TEXTURE_2D].contains(&kind));
        self.kind = kind;
        self
    }
    pub fn texture_wrapping(mut self, value: GLuint) -> Self {
        debug_assert!(value != gl::CLAMP_TO_BORDER);
        debug_assert!([gl::REPEAT, gl::MIRRORED_REPEAT, gl::CLAMP_TO_EDGE].contains(&value));
        self.texture_wrapping_s = value as GLint;
        self.texture_wrapping_t = value as GLint;
        self.texture_wrapping_r = value as GLint;
        self
    }
    pub fn texture_wrapping_s(mut self, value: GLuint) -> Self {
        debug_assert!(value != gl::CLAMP_TO_BORDER);
        debug_assert!([gl::REPEAT, gl::MIRRORED_REPEAT, gl::CLAMP_TO_EDGE].contains(&value));
        self.texture_wrapping_s = value as GLint;
        self
    }
    pub fn texture_wrapping_t(mut self, value: GLuint) -> Self {
        debug_assert!(value != gl::CLAMP_TO_BORDER);
        debug_assert!([gl::REPEAT, gl::MIRRORED_REPEAT, gl::CLAMP_TO_EDGE].contains(&value));
        self.texture_wrapping_t = value as GLint;
        self
    }
    pub fn texture_wrapping_r(mut self, value: GLuint) -> Self {
        debug_assert!(value != gl::CLAMP_TO_BORDER);
        debug_assert!([gl::REPEAT, gl::MIRRORED_REPEAT, gl::CLAMP_TO_EDGE].contains(&value));
        self.texture_wrapping_r = value as GLint;
        self
    }
    pub fn clamp_to_border(mut self, border_color: [f32; 4]) -> Self {
        self.texture_wrapping_s = gl::CLAMP_TO_BORDER as GLint;
        self.texture_wrapping_t = gl::CLAMP_TO_BORDER as GLint;
        self.texture_wrapping_r = gl::CLAMP_TO_BORDER as GLint;
        self.border_color = Some(border_color);
        self
    }
    pub fn texture_min_filter(mut self, value: GLuint) -> Self {
        debug_assert!([
            gl::NEAREST,
            gl::LINEAR,
            gl::NEAREST_MIPMAP_NEAREST,
            gl::LINEAR_MIPMAP_NEAREST,
            gl::NEAREST_MIPMAP_LINEAR,
            gl::LINEAR_MIPMAP_LINEAR
        ]
        .contains(&value));
        self.texture_min_filter = value as GLint;
        self
    }
    pub fn texture_mag_filter(mut self, value: GLuint) -> Self {
        debug_assert!([gl::NEAREST, gl::LINEAR].contains(&value));
        self.texture_mag_filter = value as GLint;
        self
    }
    pub fn generate_mipmaps(mut self, value: bool) -> Self {
        self.generate_mipmaps = value;
        self
    }

    pub fn compile(self, path: &str) -> Result<Texture, ImageError> {
        //1. Load the image
        let img = image::open(path)?.rotate180();
        let (img_w, img_h) = img.dimensions();
        let r_split = path.rsplit(".").collect::<Vec<&str>>();
        let file_ending = r_split.first().unwrap();
        let (data, format): (Vec<u8>, GLenum) = if file_ending.contains("png") {
            (img.into_rgba().into_vec(), gl::RGBA)
        } else if file_ending.contains("jpg") || file_ending.contains("jpeg") {
            (img.into_rgb().into_vec(), gl::RGB)
        } else {
            unimplemented!("File ending is not implemented");
        };

        let texture = Texture::default();
        texture.bind(0);
        unsafe {
            gl::TexParameteri(self.kind, gl::TEXTURE_WRAP_S, self.texture_wrapping_s);
            if self.texture_wrapping_s == gl::CLAMP_TO_BORDER as GLint {
                debug_assert!(self.border_color.is_some());
                gl::TexParameterfv(
                    self.kind,
                    gl::TEXTURE_BORDER_COLOR,
                    self.border_color.unwrap().as_ptr(),
                );
            }
            gl::TexParameteri(self.kind, gl::TEXTURE_WRAP_T, self.texture_wrapping_t);
            if self.texture_wrapping_t == gl::CLAMP_TO_BORDER as GLint {
                debug_assert!(self.border_color.is_some());
                gl::TexParameterfv(
                    self.kind,
                    gl::TEXTURE_BORDER_COLOR,
                    self.border_color.unwrap().as_ptr(),
                );
            }
            gl::TexParameteri(self.kind, gl::TEXTURE_WRAP_R, self.texture_wrapping_r);
            if self.texture_wrapping_r == gl::CLAMP_TO_BORDER as GLint {
                debug_assert!(self.border_color.is_some());
                gl::TexParameterfv(
                    self.kind,
                    gl::TEXTURE_BORDER_COLOR,
                    self.border_color.unwrap().as_ptr(),
                );
            }
            gl::TexParameteri(self.kind, gl::TEXTURE_MIN_FILTER, self.texture_min_filter);
            gl::TexParameteri(self.kind, gl::TEXTURE_MAG_FILTER, self.texture_mag_filter);
            if self.kind == gl::TEXTURE_2D {
                texture.tex_image2d(img_w, img_h, &data, format);
            } else {
                unimplemented!("Currently, only 2D textures can be built");
            }
            if self.generate_mipmaps {
                gl::GenerateMipmap(self.kind);
            }
        }
        Ok(texture)
    }
}
impl Default for TextureBuilder {
    fn default() -> Self {
        TextureBuilder {
            kind: gl::TEXTURE_2D,
            texture_wrapping_s: gl::REPEAT as GLint,
            texture_wrapping_t: gl::REPEAT as GLint,
            texture_wrapping_r: gl::REPEAT as GLint,
            border_color: None,
            texture_min_filter: gl::NEAREST_MIPMAP_LINEAR as GLint,
            texture_mag_filter: gl::LINEAR as GLint,
            generate_mipmaps: true,
        }
    }
}
