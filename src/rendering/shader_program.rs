use super::Shader;
use std::mem::uninitialized;

pub struct ShaderProgram {
    pub id: u32,
}

impl ShaderProgram {
    pub fn new<'a>() -> ShaderProgramBuilder<'a> {
        ShaderProgramBuilder { shaders: vec![] }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

pub struct ShaderProgramBuilder<'a> {
    shaders: Vec<&'a Shader>,
}

impl<'a> ShaderProgramBuilder<'a> {
    pub fn with(mut self, shader: &'a Shader) -> Self {
        self.shaders.push(shader);
        self
    }

    pub fn build(&self) -> Result<ShaderProgram, ()> {
        let id = unsafe { gl::CreateProgram() };
        for shader in self.shaders.iter() {
            unsafe {
                gl::AttachShader(id, shader.id);
            }
        }
        let success = unsafe {
            let mut success = uninitialized();
            gl::LinkProgram(id);
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            success
        };
        match success {
            1 => Ok(ShaderProgram { id }),
            _ => Err(()),
        }
    }
}
