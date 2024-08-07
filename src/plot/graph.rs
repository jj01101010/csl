use std::mem::size_of;

use gl::Gl;
use glam::Vec3;

use super::{
    buffer::{Buffer, BufferType},
    vao::VertexArray,
};

pub type Point = [f32; 2];

pub type AnimationCallback = fn(&mut Vec<Point>) -> ();

pub struct GraphProperties {
    pub anim: Option<AnimationCallback>,
    pub zindex: u32,
    pub data: Vec<Point>,
}

impl Default for GraphProperties {
    fn default() -> Self {
        Self {
            anim: None,
            zindex: 1,
            data: vec![],
        }
    }
}

pub struct Graph {
    pub data: Vec<Point>,
    pub graph_vao: VertexArray,
    pub graph_vbo: Buffer,
    pub position: Vec3,
    pub animation: Option<AnimationCallback>,
}

impl Graph {
    pub fn new(gl: Gl, properties: GraphProperties) -> Self {
        let graph_vao = VertexArray::new(gl.clone()).expect("Could not create VAO");
        graph_vao.bind();

        let graph_vbo = Buffer::new(gl.clone()).expect("Could not create VBO");
        graph_vbo.bind(BufferType::Array);
        graph_vbo.buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(&properties.data),
            gl::STATIC_DRAW,
        );

        unsafe {
            gl.VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Point>().try_into().unwrap(),
                std::ptr::null(),
            );
            gl.EnableVertexAttribArray(0);
        }

        graph_vao.unbind();
        graph_vbo.unbind(BufferType::Array);

        Self {
            data: properties.data,
            graph_vao,
            graph_vbo,
            animation: properties.anim,
            position: Vec3::new(150.0, 150.0, -1.0 * properties.zindex as f32),
        }
    }

    pub fn run_animation(&mut self) {
        if let Some(animation) = self.animation {
            animation(&mut self.data);
        }
    }
}
