extern crate glfw;
use std::{f32::consts::PI, fs::File, io::Read, iter::zip, mem::size_of};

use gl;
use glam::{Mat4, Vec3};
use glfw::{fail_on_errors, Action, Context, GlfwReceiver, Key, WindowEvent};

use crate::plot::{
    buffer::{Buffer, BufferType}, graph::{Graph, Point}, shader::{PlotShader, Shader, ShaderProgram, ShaderUniform}, vao::VertexArray
};

pub mod buffer;
pub mod graph;
pub mod shader;
pub mod vao;

pub struct Plot {
    // TODO: Abstract this (e.g. PlotContext)
    glfw_context: glfw::Glfw,
    // TODO: Abstract this (e.g. Window)
    window: glfw::PWindow,
    // TODO: Abstract this (e.g. Window)
    window_events: GlfwReceiver<(f64, WindowEvent)>,
}

#[cfg(debug_assertions)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolygonMode {
    Point = gl::POINT as isize,
    Line = gl::LINE as isize,
    Fill = gl::FILL as isize,
}

#[cfg(debug_assertions)]
pub fn polygon_mode(mode: PolygonMode) {
    unsafe { gl::PolygonMode(gl::FRONT_AND_BACK, mode as u32) };
}

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).expect("Could not open shader file");

    let mut buffer = String::new();

    file.read_to_string(&mut buffer)
        .expect("Could not read file");
    buffer
}

impl Plot {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
        //glfw.window_hint(glfw::WindowHint::Resizable(false));
        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        unsafe {
            gl::Viewport(0, 0, width as i32, height as i32);
        }
        // Make the window's context current
        window.make_current();
        window.set_key_polling(true);
        window.set_size_polling(true);

        Self {
            window,
            glfw_context: glfw,
            window_events: events,
        }
    }

    pub fn show(&mut self) {
        #[cfg(debug_assertions)]
        polygon_mode(PolygonMode::Fill);

        let vao = VertexArray::new().expect("Could not create VAO");
        vao.bind();

        let vbo = Buffer::new().expect("Could not create VBO");

        vbo.bind(BufferType::Array);

        type Vertex = [f32; 3];
        type TriIndexes = [u32; 3];

        const VERTICES: [Vertex; 4] = [
            [1.0, 1.0, 0.0],
            [1.0, -1.0, 0.0],
            [-1.0, -1.0, 0.0],
            [-1.0, 1.0, 0.0],
        ];

        const INDICES: [TriIndexes; 2] = [[0, 1, 3], [1, 2, 3]];
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


        let x = (0..=100).map(|x| {
            (x as f32)/100.0
        });

        let y = x.clone().map(|x| {
            f32::sin(2.0 * PI * x)
        });

        let points: Vec<Point> = zip(x, y).into_iter().map(|(x, y)| {
            [x, y]
        }).collect();

        let graph = Graph::new(points.into_boxed_slice());

        let graph_vao = VertexArray::new().expect("Could not create VAO");
        graph_vao.bind();
        
        let graph_vbo = Buffer::new().expect("Could not create VBO");
        graph_vbo.bind(BufferType::Array);
        Buffer::buffer_data(
            BufferType::Array,
            bytemuck::cast_slice(&graph.data),
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

        let file_content = read_file("shaders/shader.vert.glsl");
        let vert_shader = file_content.as_str();
        let file_content = read_file("shaders/shader.frag.glsl");
        let frag_shader = file_content.as_str();
        let file_content = read_file("shaders/shader_graph.frag.glsl");
        let frag_shader_graph = file_content.as_str();
        let file_content = read_file("shaders/shader_graph.vert.glsl");
        let vert_shader_graph = file_content.as_str();

        let shader = ShaderProgram::from_vert_frag(vert_shader, frag_shader)
            .expect("Could not compile shaders");

        let graph_shader = ShaderProgram::from_vert_frag(vert_shader_graph, frag_shader_graph)
            .expect("Could not compile shaders");


        let vp = ShaderUniform::load(&shader, "vp\0").expect("Could not load vp");
        let offset = ShaderUniform::load(&shader, "offset\0").expect("Could not load offset");
        let pitch = ShaderUniform::load(&shader, "pitch\0").expect("Could not load pitch");
        let transform_uniform = ShaderUniform::load(&shader, "transform\0").expect("Could not load pitch");


        let plot_shader = PlotShader {
            shader,
            vp,
            offset,
            pitch,
            transform: transform_uniform,
        };

        let graph_transform_uniform: ShaderUniform<Mat4> = ShaderUniform::load(&graph_shader, "transform\0").expect("Could not load pitch");


        let size = self.window.get_size();
        plot_shader.shader.use_program();
        plot_shader.offset.set([0.0, 0.0]);
        plot_shader.pitch.set([100.0, 100.0]);
        plot_shader.vp.set([size.0 as f32, size.1 as f32]);

        let mvp = glam::Mat4::from_translation(Vec3::new(
            0.0,
            1.618/4.0,
            0.0
        ));
        
        let scale = glam::Mat4::from_scale(Vec3::new(0.5, 1.618/4.0, 1.0));
        //let scale = glam::Mat4::IDENTITY;

        plot_shader.transform.set((mvp * scale).clone());

        graph_shader.use_program();
        graph_transform_uniform.set(mvp * scale);


        // Loop until the user closes the window
        while !self.window.should_close() {
            unsafe {
                gl::ClearColor(0.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            // Poll for and process events
            self.glfw_context.poll_events();
            for (_, event) in glfw::flush_messages(&self.window_events) {
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true)
                    }
                    glfw::WindowEvent::Size(w, h) => {
                        plot_shader.shader.use_program();
                        unsafe {
                            gl::Viewport(0, 0, w, h);
                        }
                        let pos = self.window.get_pos();
                        self.window.set_pos(pos.0 + 1, pos.1);
                        plot_shader.vp.set([w as f32, h as f32]);
                    }
                    _ => {}
                }
            }

            plot_shader.shader.use_program();
            vao.bind();
            unsafe {
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            }

            graph_shader.use_program();
            graph_vao.bind();
            unsafe {
                gl::DrawArrays(gl::LINE_STRIP, 0, graph.data.len() as i32);
            }

            // Swap front and back buffers
            self.window.swap_buffers();
        }
        plot_shader.shader.delete();
    }
}
