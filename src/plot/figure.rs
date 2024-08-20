use gl::Gl;
use glam::Vec2;
use std::{mem::size_of, vec};

use super::{
    graph::{Graph, GraphProperties},
    vao::VertexArray,
};

use crate::plot::buffer::{Buffer, BufferType};

pub struct Quad {
    vao: VertexArray,
    _vbo: Buffer,
    _ebo: Buffer,
}

impl Quad {
    pub fn new(gl: &Gl) -> Self {
        let vao = VertexArray::new(gl.clone()).unwrap();
        vao.bind();

        let vbo = Buffer::new(gl.clone()).expect("Could not create VBO");
        vbo.bind(BufferType::Array);

        // This might be overkill, since we only render 4 vertices
        let ebo = Buffer::new(gl.clone()).expect("Could not create VBO");
        ebo.bind(BufferType::ElementArray);

        vbo.buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(&VERTICES),
            gl::STATIC_DRAW,
        );

        ebo.buffer_data(
            BufferType::ElementArray,
            bytemuck::cast_slice(&INDICES),
            gl::STATIC_DRAW,
        );

        unsafe {
            gl.VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                std::ptr::null(),
            );
            gl.EnableVertexAttribArray(0);
        }

        vao.unbind();
        vbo.unbind(BufferType::Array);
        ebo.unbind(BufferType::ElementArray);

        Self {
            vao: vao,
            _vbo: vbo,
            _ebo: ebo,
        }
    }

    pub fn bind(&self) {
        self.vao.bind()
    }

    pub fn unbind(&self) {
        self.vao.unbind()
    }
}

pub struct Figure {
    pub render_quad: Quad,
    pub size: [f32; 2],
    pub pos: [f32; 2],
    pub graphs: Vec<Graph>,
    pub offset: Vec2,
}

/* TODO: Add enum FigureLayout {
    Manual(w, h),
    Aspect(aspect),
    None // Free layout from window
} to FigureProperties
*/

pub struct FigureProperties {
    pub width: Option<f32>,
    pub height: Option<f32>,
    pub offset: Vec2,
    pub graphs: Vec<GraphProperties>,
}

impl Default for FigureProperties {
    fn default() -> Self {
        Self {
            graphs: vec![],
            width: None,
            height: None,
            offset: Vec2::ZERO,
        }
    }
}

impl Figure {
    pub fn new(gl: Gl, properties: FigureProperties) -> Self {
        let width = properties.width.unwrap_or(300.0);
        let height = properties.height.unwrap_or(300.0);

        Self {
            render_quad: Quad::new(&gl),
            pos: [width / 2.0, height / 2.0],
            size: [width, height],
            graphs: properties
                .graphs
                .into_iter()
                .map(|g_prop| Graph::new(gl.clone(), g_prop))
                .collect(),
            offset: properties.offset,
        }
    }

    pub fn add_offset(&mut self, d_offset: Vec2) {
        self.offset += d_offset;
        // TODO: Graph offset
    }

    // TODO: Implement method
    pub fn point_is_inside(&self, _point: Vec2) -> bool {
        true
    }

    pub fn add_plot(&mut self, graph: Graph) {
        self.graphs.push(graph);
    }

    pub fn update(&mut self) {
        for graph in &mut self.graphs {
            graph.run_animation();
        }
    }
}

type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] = [
    [1.0, 1.0, 0.0],
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];
