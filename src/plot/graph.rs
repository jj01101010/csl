use std::{f32::consts::PI, mem::size_of};

use gl::Gl;
use glam::{Vec2, Vec3};

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
    pub animation: Option<AnimationCallback>,
    pub pos: Vec3,
    pub size: Vec2,
}

impl Graph {
    fn generate_quads_from_graph(data: &Vec<[f32; 2]>, dist: f32) -> Vec<[f32; 2]> {
        // For every data point we generate 2 new vertex points
        let mut output = vec![[0.0, 0.0]; 2 * data.len()];

        let mut point_iter = data.iter();

        let mut i = 0;

        while let Some(tmp_point_1) = point_iter.next() {
            let point_1: &[f32; 2];
            let point_2: &[f32; 2];
            let switched;
            if let Some(tmp_point_2) = point_iter.next() {
                point_1 = tmp_point_1;
                point_2 = tmp_point_2;
                switched = false;
            } else {
                // Last even vertex will be computed with the last odd vertex
                point_2 = tmp_point_1;
                point_1 = &data[i - 1];
                switched = true;
            }

            let x = point_2[0] - point_1[0];
            let y = point_2[1] - point_1[1];
            let beta = PI / 2.0 - y.atan2(x);
            let delta_x = -f32::cos(beta) * dist; // This distance is negative, but should be positive
            let delta_y = f32::sin(beta) * dist;

            output[i * 2 + 0] = [point_1[0] + delta_x, point_1[1] + delta_y]; // point_1 + delta
            output[i * 2 + 1] = [point_1[0] - delta_x, point_1[1] - delta_y]; // point_1 - delta
            if !switched {
                output[i * 2 + 2] = [point_2[0] + delta_x, point_2[1] + delta_y]; // point_2 + delta
                output[i * 2 + 3] = [point_2[0] - delta_x, point_2[1] - delta_y];
                // point_2 - delta
            }

            i += 2;
        }

        output
    }

    pub fn new(gl: Gl, properties: GraphProperties) -> Self {
        let data = properties.data;

        let graph_vertices = Graph::generate_quads_from_graph(&data, 0.005);

        let graph_vao = VertexArray::new(gl.clone()).expect("Could not create VAO");
        graph_vao.bind();

        let graph_vbo = Buffer::new(gl.clone()).expect("Could not create VBO");
        graph_vbo.bind(BufferType::Array);
        graph_vbo.buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(&graph_vertices),
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
            data,
            graph_vao,
            graph_vbo,
            animation: properties.anim,
            pos: Vec3::new(250.0, 250.0, -1.0 * properties.zindex as f32),
            size: Vec2::new(250.0, 250.0)
        }
    }

    pub fn run_animation(&mut self) {
        if let Some(animation) = self.animation {
            animation(&mut self.data);
        }
    }
}
