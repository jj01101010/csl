use std::mem::size_of;

use glam::{Mat4, Vec3};

use super::{
    buffer::{Buffer, BufferType},
    shader::{Shader, ShaderProgram, ShaderType, ShaderUniform},
    vao::VertexArray,
};

pub type Point = [f32; 2];

struct GraphShader {
    pub shader: ShaderProgram,
    pub transform_uniform: ShaderUniform<Mat4>,
}

pub type AnimationCallback = fn(&mut Vec<Point>) -> ();

pub struct GraphProperties {
    pub anim: Option<AnimationCallback>,
    pub zindex: u32,
}

impl Default for GraphProperties {
    fn default() -> Self {
        Self {
            anim: None,
            zindex: 1,
        }
    }
}

pub struct Graph {
    pub data: Vec<Point>,
    graph_vao: VertexArray,
    graph_shader: GraphShader,
    pub properties: GraphProperties,
}

impl Graph {
    pub fn new(data: Vec<Point>, properties: GraphProperties) -> Self {
        let graph_vao = VertexArray::new().expect("Could not create VAO");
        graph_vao.bind();

        let graph_vbo = Buffer::new().expect("Could not create VBO");
        graph_vbo.bind(BufferType::Array);
        Buffer::buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(&data),
            gl::STATIC_DRAW,
        );

        unsafe {
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Point>().try_into().unwrap(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }

        VertexArray::unbind();
        Buffer::unbind(BufferType::Array);

        let graph_shader = ShaderProgram::from_shaders(vec![
            Shader::from_file(ShaderType::Vertex, "shaders/shader_graph.vert.glsl").unwrap(),
            Shader::from_file(ShaderType::Fragment, "shaders/shader_graph.frag.glsl").unwrap(),
        ])
        .unwrap();

        let graph_transform_uniform: ShaderUniform<Mat4> =
            ShaderUniform::load(&graph_shader, "transform\0");

        Self {
            data,
            graph_vao,
            graph_shader: GraphShader {
                shader: graph_shader,
                transform_uniform: graph_transform_uniform,
            },
            properties,
        }
    }

    pub fn render(&self) {
        let proj = glam::Mat4::orthographic_lh(0.0, 300.0, 0.0, 300.0, 0.01, 100.0);

        let translation = glam::Mat4::from_translation(Vec3::new(
            150.0,
            150.0,
            -1.0 * self.properties.zindex as f32,
        )) * glam::Mat4::from_scale(Vec3::new(150.0, 150.0, 1.0));

        self.graph_shader.shader.use_program();
        self.graph_shader.transform_uniform.set(proj * translation);

        self.graph_vao.bind();
        unsafe {
            gl::DrawArrays(gl::LINE_STRIP, 0, self.data.len() as i32);
        }
    }

    pub fn run_animation(&mut self) {
        if let Some(animation) = self.properties.anim {
            animation(&mut self.data);
        }
    }
}
