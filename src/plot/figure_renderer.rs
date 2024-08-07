use glam::{Mat4, Quat, Vec2, Vec3};

use super::{
    figure::Figure,
    shader::{Shader, ShaderProgram, ShaderType, ShaderUniform},
};

struct FigureShader {
    shader: ShaderProgram,
    transform_uniform: ShaderUniform<Mat4>,
    offset_uniform: ShaderUniform<Vec2>,
    pitch_uniform: ShaderUniform<Vec2>,
}

pub struct FigureRenderer {
    figure_shader: FigureShader,
    proj_matrix: glam::Mat4,
    gl: gl::Gl,
}

impl FigureRenderer {
    /// Create a new FigureRenderer
    pub fn new(gl: gl::Gl) -> Self {
        // Create the shader to render the figures
        let figure_shader = ShaderProgram::from_shaders(
            gl.clone(),
            vec![
                Shader::from_file(gl.clone(), ShaderType::Vertex, "shaders/shader.vert.glsl")
                    .expect("Could not get shader"),
                Shader::from_file(gl.clone(), ShaderType::Fragment, "shaders/shader.frag.glsl")
                    .expect("Could not get shader"),
            ],
        )
        .expect("Could not create program");

        // Get the corresponding uniform variables
        let transform_uniform = ShaderUniform::load(gl.clone(), &figure_shader, "transform");
        let offset_uniform = ShaderUniform::load(gl.clone(), &figure_shader, "offset");
        let pitch_uniform: ShaderUniform<Vec2> =
            ShaderUniform::load(gl.clone(), &figure_shader, "pitch");

        // Calculate the projection matrix
        let proj = glam::Mat4::orthographic_lh(0.0, 300.0, 0.0, 300.0, 0.01, 100.0);

        Self {
            gl,
            figure_shader: FigureShader {
                shader: figure_shader,
                transform_uniform,
                offset_uniform,
                pitch_uniform,
            },
            proj_matrix: proj,
        }
    }

    /// Render the figure to the screen
    pub fn render(&self, figure: &Figure) {
        self.figure_shader.shader.use_program();

        figure.render_quad.bind();

        self.figure_shader
            .pitch_uniform
            .set(Vec2 { x: 100.0, y: 100.0 });
        let translation = glam::Mat4::from_scale_rotation_translation(
            Vec3::new(figure.size[0], figure.size[1], 1.0),
            Quat::IDENTITY,
            Vec3::new(figure.pos[0], figure.pos[1], 0.0),
        );

        self.figure_shader
            .transform_uniform
            .set(self.proj_matrix * translation);
        self.figure_shader.offset_uniform.set(figure.offset);

        unsafe {
            self.gl
                .DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        figure.render_quad.unbind();
    }
}
