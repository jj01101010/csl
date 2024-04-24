use glam::{Mat4, Quat, Vec2, Vec3};
use log::info;
use std::mem::size_of;

use super::{graph::Graph, graph_renderer::GraphRenderer, shader::PlotShader, vao::VertexArray};

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
    pub pos: [f32; 2],
    pub graphs: Vec<Graph>,
    graph_renderer: GraphRenderer,
    offset: Vec2,
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
}

impl Default for FigureProperties {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            offset: Vec2::ZERO,
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

        let offset: ShaderUniform<Vec2> = ShaderUniform::load(&shader_program, "offset");
        let pitch: ShaderUniform<Vec2> = ShaderUniform::load(&shader_program, "pitch");
        let transform: ShaderUniform<Mat4> = ShaderUniform::load(&shader_program, "transform");

        let width = properties.width.unwrap_or(300.0);
        let height = properties.height.unwrap_or(300.0);

        println!("{}, {}", width, height);

        shader_program.use_program();
        pitch.set(Vec2 { x: 100.0, y: 100.0 });

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
            graphs: vec![],
            offset: properties.offset,
            graph_renderer: GraphRenderer::default(),
        }
    }

    pub fn add_offset(&mut self, d_offset: Vec2) {
        self.offset += d_offset;
        self.graph_renderer.offset += d_offset;
    }

    // TODO: Implement method
    pub fn point_is_inside(&self, _point: Vec2) -> bool {
        return true;
    }

    pub fn add_plot(&mut self, graph: Graph) {
        self.graphs.push(graph);
    }

    pub fn update(&mut self) {
        for graph in &mut self.graphs {
            graph.run_animation();
        }
    }

    pub fn render(&mut self) {
        self.plot_shader.shader.use_program();

        // TODO: Assign these width and height parameters based on the layout of
        //  the plot window
        // TODO: This should be in the window?
        let proj = glam::Mat4::orthographic_lh(0.0, 300.0, 0.0, 300.0, 0.01, 100.0);

        let translation = glam::Mat4::from_scale_rotation_translation(
            Vec3::new(self.size[0], self.size[1], 1.0),
            Quat::IDENTITY,
            Vec3::new(self.pos[0], self.pos[1], 0.0),
        );

        self.plot_shader.transform.set(proj * translation);
        self.plot_shader.offset.set(self.offset);

        self.render_quad.vao.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        for graph in &mut self.graphs {
            self.graph_renderer.render(&graph);
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
    [1.0, 0.0, 0.0],
    [0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0],
];

const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];
