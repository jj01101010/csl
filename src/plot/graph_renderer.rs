use glam::{Mat4, Vec3};

use super::{
    graph::Graph,
    shader::{Shader, ShaderProgram, ShaderType, ShaderUniform},
};

struct GraphShader {
    pub shader: ShaderProgram,
    pub transform_uniform: ShaderUniform<Mat4>,
}

pub struct GraphRenderer {
    graph_shader: GraphShader,
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

        Self {
            graph_shader: GraphShader {
                shader: graph_shader,
                transform_uniform: graph_transform_uniform,
            },
        }
    }
}

impl GraphRenderer {
    pub fn render(&self, graph: &Graph) {
        graph.graph_vao.bind();

        let proj = glam::Mat4::orthographic_lh(0.0, 300.0, 0.0, 300.0, 0.01, 100.0);

        let translation = glam::Mat4::from_translation(graph.position)
            * glam::Mat4::from_scale(Vec3::new(150.0, 150.0, 1.0));

        self.graph_shader.shader.use_program();
        self.graph_shader.transform_uniform.set(proj * translation);

        unsafe {
            gl::DrawArrays(gl::LINE_STRIP, 0, graph.data.len() as i32);
        }
    }
}
