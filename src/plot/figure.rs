use glam::{Mat4, Vec3};
use log::info;
use std::mem::size_of;

use super::{graph::Graph, shader::PlotShader, vao::VertexArray};
use crate::plot::{
    buffer::{Buffer, BufferType},
    shader::{Shader, ShaderProgram, ShaderType, ShaderUniform},
};

struct Quad {
    vao: VertexArray,
    vbo: Buffer,
    ebo: Buffer,
}

impl Quad {
    pub fn delete(&self) {
        self.vbo.delete();
        self.ebo.delete();
        self.vao.delete();
    }
}

pub struct Figure {
    render_quad: Quad,
    plot_shader: PlotShader,
    size: [f32; 2],
    pos: [f32; 2],
    pub graphs: Vec<Graph>
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
    pub offset: [f32; 2],
}

impl Default for FigureProperties {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            offset: [0.0, 0.0],
        }
    }
}

impl Figure {
    pub fn new(properties: FigureProperties) -> Self {
        let vao = VertexArray::new().unwrap();
        vao.bind();

        let vbo = Buffer::new().expect("Could not create VBO");
        vbo.bind(BufferType::Array);

        // This might be overkill, since we only render 4 vertices
        let ebo = Buffer::new().expect("Could not create VBO");
        ebo.bind(BufferType::ElementArray);

        Buffer::buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(&VERTICES),
            gl::STATIC_DRAW,
        );

        Buffer::buffer_data(
            BufferType::ElementArray,
            bytemuck::cast_slice(&INDICES),
            gl::STATIC_DRAW,
        );

        unsafe {
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                size_of::<Vertex>().try_into().unwrap(),
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);
        }

        VertexArray::unbind();
        Buffer::unbind(BufferType::Array);
        Buffer::unbind(BufferType::ElementArray);

        let vert_shader = Shader::from_file(ShaderType::Vertex, "shaders/shader.vert.glsl")
            .expect("Could not get shader");
        let frag_shader = Shader::from_file(ShaderType::Fragment, "shaders/shader.frag.glsl")
            .expect("Could not get shader");

        let shader_program = ShaderProgram::from_shaders(vec![vert_shader, frag_shader])
            .expect("Could not create program");

        let offset: ShaderUniform<[f32; 2]> = ShaderUniform::load(&shader_program, "offset");
        let pitch: ShaderUniform<[f32; 2]> = ShaderUniform::load(&shader_program, "pitch");
        let transform: ShaderUniform<Mat4> = ShaderUniform::load(&shader_program, "transform");

        let width = match properties.width {
            Some(w) => w,
            None => {
                300.0 // TODO: Get this from window
            }
        };

        let height = match properties.height {
            Some(h) => h,
            None => {
                300.0 // TODO: Get this from window
            }
        };

        shader_program.use_program();
        offset.set(properties.offset);
        pitch.set([100.0, 100.0]);

        Self {
            render_quad: Quad { vao, ebo, vbo },
            plot_shader: PlotShader {
                shader: shader_program,
                offset,
                pitch,
                transform,
            },
            pos: [width / 2.0, height / 2.0],
            size: [width, height],
            graphs: vec![]
        }
    }

    pub fn add_plot(&mut self, graph: Graph) {
        self.graphs.push(graph);
    }

    pub fn render(&mut self) {
        self.plot_shader.shader.use_program();

        // TODO: Assign these width and height parameters based on the layout of
        //  the plot window
        let proj = glam::Mat4::orthographic_lh(0.0, self.size[0], 0.0, self.size[1], 0.01, 100.0);

        let translation = glam::Mat4::from_translation(Vec3::new(self.pos[0], self.pos[1], 0.0))
            * glam::Mat4::from_scale(Vec3::new(self.size[0] / 2.0, self.size[1] / 2.0, 1.0));

        self.plot_shader.transform.set(proj * translation);

        self.render_quad.vao.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        for graph in &mut self.graphs {
            graph.render();
            let new_data = graph.anim.unwrap()(&graph.data);
            match new_data {
                None => {

                },
                Some(data) => {
                    graph.data = data;
                }
            }
        }
    }
}

impl Drop for Figure {
    fn drop(&mut self) {
        info!("Deleting figure!");
        self.plot_shader.shader.delete();
        self.render_quad.delete();
    }
}

type Vertex = [f32; 3];
type TriIndexes = [u32; 3];

const VERTICES: [Vertex; 4] = [
    [1.0, 1.0, 0.0],
    [1.0, -1.0, 0.0],
    [-1.0, -1.0, 0.0],
    [-1.0, 1.0, 0.0],
];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];
