use gl;

pub struct VertexArray {
    pub id: u32,
}

impl VertexArray {
    pub fn new() -> Option<Self> {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        if vao != 0 {
            Some(Self { id: vao })
        } else {
            None
        }
    }

    pub(crate) fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub(crate) fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) }
    }
}
