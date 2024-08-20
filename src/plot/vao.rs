use gl::{self, Gl};

pub struct VertexArray {
    id: u32,
    gl: Gl,
}

impl VertexArray {
    pub fn new(gl: Gl) -> Option<Self> {
        let mut vao = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }
        if vao != 0 {
            Some(Self { id: vao, gl })
        } else {
            None
        }
    }

    pub(crate) fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.id);
        }
    }

    pub(crate) fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteVertexArrays(1, &self.id) }
    }
}
