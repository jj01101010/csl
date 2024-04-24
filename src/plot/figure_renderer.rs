use super::figure::Figure;

pub struct FigureRenderer  {
    gl: gl::Gl
}

impl FigureRenderer {
    pub fn new(gl: gl::Gl) -> Self {
        Self {
            gl
        }
    }

    pub fn render(&self, figure: Figure) {
        todo!("Render Figure")
    }
}
