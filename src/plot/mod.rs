extern crate glfw;
use std::mem::size_of;

use gl;
use glfw::{fail_on_errors, Action, Context, GlfwReceiver, Key, WindowEvent};

use crate::plot::{
    buffer::{Buffer, BufferType},
    shader::ShaderProgram,
    vao::VertexArray,
};

pub mod buffer;
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

impl Plot {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(fail_on_errors!()).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        glfw.window_hint(glfw::WindowHint::Resizable(false));
        // Create a windowed mode window and its OpenGL context
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        // Make the window's context current
        window.make_current();
        window.set_key_polling(true);

        Self {
            window,
            glfw_context: glfw,
            window_events: events,
        }
    }

    pub fn show(&mut self) {
        #[cfg(debug_assertions)]
        polygon_mode(PolygonMode::Line);

        let vao = VertexArray::new().expect("Could not create VAO");
        vao.bind();

        let vbo = Buffer::new().expect("Could not create VBO");

        vbo.bind(BufferType::Array);

        type Vertex = [f32; 3];
        type TriIndexes = [u32; 3];

        const VERTICES: [Vertex; 4] = [
            [0.5, 0.5, 0.0],
            [0.5, -0.5, 0.0],
            [-0.5, -0.5, 0.0],
            [-0.5, 0.5, 0.0],
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

        const VERT_SHADER: &str = r#"#version 330 core
        layout (location = 0) in vec3 pos;
        void main() {
          gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
        }
      "#;

        const FRAG_SHADER: &str = r#"#version 330 core
          out vec4 final_color;
          void main() {
            final_color = vec4(1.0, 0.5, 0.2, 1.0);
          }
        "#;

        let shader = ShaderProgram::from_vert_frag(VERT_SHADER, FRAG_SHADER)
            .expect("Could not compile shaders");

        vao.bind();

        // Loop until the user closes the window
        while !self.window.should_close() {
            unsafe {
                gl::ClearColor(0.0, 1.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            // Poll for and process events
            self.glfw_context.poll_events();
            for (_, event) in glfw::flush_messages(&self.window_events) {
                if let glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) = event {
                    self.window.set_should_close(true)
                }
            }

            shader.use_program();
            unsafe {
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            }

            // Swap front and back buffers
            self.window.swap_buffers();
        }
        shader.delete();
    }
}
