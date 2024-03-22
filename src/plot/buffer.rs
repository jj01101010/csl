use gl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BufferType {
    Array = gl::ARRAY_BUFFER as isize,
    ElementArray = gl::ELEMENT_ARRAY_BUFFER as isize,
}

pub struct Buffer {
    pub id: u32,
}

impl Buffer {
    pub fn new() -> Option<Self> {
        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        if vbo != 0 {
            Some(Self { id: vbo })
        } else {
            None
        }
    }

    /// Bind this vertex buffer for the given type
    pub fn bind(&self, ty: BufferType) {
        unsafe { gl::BindBuffer(ty as u32, self.id) }
    }

    /// Clear the current vertex buffer binding for the given type.
    pub fn unbind(ty: BufferType) {
        unsafe { gl::BindBuffer(ty as u32, 0) }
    }

    pub fn buffer_data(ty: BufferType, data: &[u8], usage: u32) {
        unsafe {
            gl::BufferData(
                ty as u32,
                data.len().try_into().unwrap(),
                data.as_ptr().cast(),
                usage,
            );
        }
    }

    pub fn delete(&self) {
        unsafe {
            gl::DeleteBuffers(1, &self.id);
        }
    }
}
