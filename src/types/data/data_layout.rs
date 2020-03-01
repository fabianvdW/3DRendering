use crate::types::data::data_specification::DataSpecification;
use gl::types::*;
use std::ffi::c_void;

#[derive(Clone)]
pub struct DataLayout {
    specs: Vec<DataSpecification>,
}
impl DataLayout {
    pub fn push(mut self, spec: DataSpecification) -> Self {
        self.specs.push(spec);
        self
    }
    pub fn infer_from_f32slice(
        slice: &[f32],
        cutoffs: &[usize],
        normalize: GLboolean,
        data_points: usize,
    ) -> Self {
        debug_assert!(slice.len() % data_points == 0);
        let cols = slice.len() / data_points;
        let mut layout = DataLayout::default();
        let mut last_cutoff = 0;
        for c in cutoffs.iter() {
            let components = (c - last_cutoff) as u32;
            layout = layout.push(DataSpecification {
                stride: components * 4,
                components: components as GLint,
                normalize,
            });
            last_cutoff = *c;
        }
        let components = (cols - last_cutoff) as u32;
        layout = layout.push(DataSpecification {
            stride: components * 4,
            components: components as GLint,
            normalize,
        });
        layout
    }
    pub fn vertex_attrib_pointer(&self) {
        let sum_stride = self.specs.iter().fold(0 as GLuint, |c, s| c + s.stride);
        unsafe {
            let mut current_offset = 0 as GLuint;
            for (i, spec) in self.specs.iter().enumerate() {
                gl::VertexAttribPointer(
                    i as GLuint,
                    spec.components,
                    gl::FLOAT,
                    spec.normalize,
                    sum_stride as GLint,
                    current_offset as *const c_void,
                );
                gl::EnableVertexAttribArray(i as GLuint);
                current_offset += spec.stride;
            }
        }
    }
}
impl Default for DataLayout {
    fn default() -> Self {
        DataLayout { specs: vec![] }
    }
}
