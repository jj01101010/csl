use gl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    Array = gl::ARRAY_BUFFER as isize,
    ElementArray = gl::ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer {
    pub id: u32,
    gl: gl::Gl
}

impl Buffer {
    pub fn new(gl: gl::Gl) -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            gl.GenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Some(Self { id: vbo, gl })
        } else {
            None
        }
    }

    /// Bind this vertex buffer for the given type
    pub fn bind(&self, ty: BufferType) {
        unsafe { self.gl.BindBuffer(ty as u32, self.id) }
    }

    /// Clear the current vertex buffer binding for the given type.
    pub fn unbind(&self, ty: BufferType) {
        unsafe { self.gl.BindBuffer(ty as u32, 0) }
    }

    pub fn buffer_data(&self, ty: BufferType, data: &[u8], usage: u32) {
        unsafe {
            self.gl.BufferData(
                ty as u32,
                data.len().try_into().unwrap(),
                data.as_ptr().cast(),
                usage,
            );
        }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.id)
        }
    }
}
