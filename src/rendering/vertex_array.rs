use super::{DataType, Buffer};
use gl::types::*;
use std::mem::uninitialized;
use std::ptr::null;

pub struct VertexArray {
    id: u32,
}

impl VertexArray {
    pub fn new<'a>() -> VertexArrayBuilder<'a> {
        VertexArrayBuilder {
            buffers: vec![],
            location: 0,
            components: 0,
            data_type: DataType::Float,
            normalize: false,
            stride: 0,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct VertexArrayBuilder<'a> {
    buffers: Vec<&'a Buffer>,
    location: GLuint,
    components: GLint,
    data_type: DataType,
    normalize: bool,
    stride: GLsizei,
}

impl<'a> VertexArrayBuilder<'a> {
    pub fn buffer(mut self, buffer: &'a Buffer) -> Self {
        self.buffers.push(buffer);
        self
    }

    pub fn location(mut self, location: GLuint) -> Self {
        self.location = location;
        self
    }

    // Must be fewer than 5
    pub fn components(mut self, components: GLint) -> Self {
        self.components = components;
        self
    }

    pub fn data_type(mut self, data_type: DataType) -> Self {
        self.data_type = data_type;
        self
    }

    pub fn normalize(mut self, normalize: bool) -> Self {
        self.normalize = normalize;
        self
    }

    pub fn build(self) -> VertexArray {
        let id = unsafe {
            let mut id: u32 = uninitialized();
            gl::GenVertexArrays(1, &mut id);
            gl::BindVertexArray(id);
            id
        };
        for buffer in self.buffers {
            buffer.bind();
        }
        unsafe {
            gl::VertexAttribPointer(
                self.location,
                self.components,
                self.data_type as GLenum,
                self.normalize as u8,
                0,      // Stride
                null(), // offset
            );
            gl::EnableVertexAttribArray(self.location);
        }
        VertexArray::unbind();
        VertexArray { id }
    }
}
