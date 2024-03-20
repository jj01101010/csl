use glam::{Mat4, Vec3};
use std::mem::size_of;

use super::{shader::PlotShader, vao::VertexArray};
use crate::plot::{
    buffer::{Buffer, BufferType},
    shader::{Shader, ShaderProgram, ShaderType, ShaderUniform},
};

struct Quad {
    vao: VertexArray,
}

pub struct Figure {
    render_quad: Quad,
    plot_shader: PlotShader,
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
        let vao = VertexArray::new().expect("Could not create VAO");
        vao.bind();

        let vbo = Buffer::new().expect("Could not create VBO");

        vbo.bind(BufferType::Array);

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

        let offset: ShaderUniform<[f32; 2]> =
            ShaderUniform::load(&shader_program, "offset\0").unwrap();
        let pitch: ShaderUniform<[f32; 2]> =
            ShaderUniform::load(&shader_program, "pitch\0").unwrap();
        let transform: ShaderUniform<Mat4> =
            ShaderUniform::load(&shader_program, "transform\0").unwrap();

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

        // TODO: Assign these width and height parameters based on the layout of
        //  the plot window
        let proj = glam::Mat4::orthographic_lh(0.0, width, 0.0, height, 0.01, 100.0);

        let translation = glam::Mat4::from_translation(Vec3::new(width / 2.0, height / 2.0, 0.0))
            * glam::Mat4::from_scale(Vec3::new(width / 2.0, height / 2.0, 1.0));

        shader_program.use_program();
        offset.set(properties.offset);
        pitch.set([100.0, 100.0]);
        transform.set(proj * translation);

        Self {
            render_quad: Quad { vao },
            plot_shader: PlotShader {
                shader: shader_program,
                offset,
                pitch,
                transform,
            },
        }
    }

    pub fn render(&self) {
        self.plot_shader.shader.use_program();
        self.render_quad.vao.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
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
