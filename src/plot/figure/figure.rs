use gl::Gl;
use glam::Vec2;
use std::{mem::size_of, vec};

use super::{
    super::graph::{Graph, GraphProperties},
    super::vao::VertexArray,
    super::layout::layout::Layout,
};

use crate::plot::{buffer::{Buffer, BufferType}, layout::layout::EmptyLayout};

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
    layout: Box<dyn Layout>,
}

pub struct FigureProperties {
    pub size: [f32; 2],
    pub offset: Vec2,
    pub graphs: Vec<GraphProperties>,
    pub layout: Box<dyn Layout>,
}

impl Default for FigureProperties {
    fn default() -> Self {
        Self {
            graphs: vec![],
            size: [500.0, 500.0],
            offset: Vec2::ZERO,
            layout: Box::new(EmptyLayout)
        }
    }
}

// TODO: Make Figure Builder 
// graph = GraphBuilder.new().data().xlim(0, 100)....build()
// builder
//    .title("Title") // Adds default Layout for Title
//    .graph(graph)   // Adds default Layout for Graph

impl Figure {
    pub fn new(gl: Gl, properties: FigureProperties) -> Self {
        Self {
            render_quad: Quad::new(&gl),
            pos: [0.0, 0.0],
            size: properties.size,
            graphs: properties
                .graphs
                .into_iter()
                .map(|g_prop| Graph::new(gl.clone(), g_prop))
                .collect(),
            offset: properties.offset,
            layout: properties.layout,
        }
    }

    pub fn add_offset(&mut self, offset: Vec2) {
        self.offset += offset;
        // TODO: Graph offset
    }

    pub fn recalculate_layout(&mut self, parent: &Box<dyn Layout>) {
        self.layout.calculate_size(Some(parent));
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
