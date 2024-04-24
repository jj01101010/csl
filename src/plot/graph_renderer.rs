use glam::{Mat4, Quat, Vec2, Vec3};

use super::{
    graph::Graph,
    shader::{Shader, ShaderProgram, ShaderType, ShaderUniform},
};

struct GraphShader {
    pub shader: ShaderProgram,
    pub transform_uniform: ShaderUniform<Mat4>,
    pub offset_uniform: ShaderUniform<Vec2>,
}

pub struct GraphRenderer {
    graph_shader: GraphShader,
    proj_matrix: glam::Mat4,
    pub offset: glam::Vec2,
}

impl Default for GraphRenderer {
    fn default() -> Self {
        let graph_shader = ShaderProgram::from_shaders(vec![
            Shader::from_file(ShaderType::Vertex, "shaders/shader_graph.vert.glsl").unwrap(),
            Shader::from_file(ShaderType::Fragment, "shaders/shader_graph.frag.glsl").unwrap(),
        ])
        .unwrap();

        let graph_transform_uniform: ShaderUniform<Mat4> =
            ShaderUniform::load(&graph_shader, "transform");
        let graph_offset_uniform: ShaderUniform<Vec2> =
            ShaderUniform::load(&graph_shader, "offset");

        let proj = glam::Mat4::orthographic_lh(0.0, 300.0, 0.0, 300.0, 0.01, 100.0);

        Self {
            graph_shader: GraphShader {
                shader: graph_shader,
                transform_uniform: graph_transform_uniform,
                offset_uniform: graph_offset_uniform,
            },
            proj_matrix: proj,
            offset: glam::Vec2::ZERO,
        }
    }
}

impl GraphRenderer {
    pub fn render(&self, graph: &Graph) {
        graph.graph_vao.bind();

        let translation = glam::Mat4::from_scale_rotation_translation(
            Vec3::new(150.0, 150.0, 1.0),
            Quat::IDENTITY,
            graph.position,
        );
        self.graph_shader.shader.use_program();
        self.graph_shader.offset_uniform.set(self.offset);
        self.graph_shader
            .transform_uniform
            .set(self.proj_matrix * translation);

        unsafe {
            gl::DrawArrays(gl::LINE_STRIP, 0, graph.data.len() as i32);
        }
    }
}
