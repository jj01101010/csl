use std::{fs::File, io::Read, marker::PhantomData};

use gl::Gl;
use glam::{Vec2, Vec3};
use log::warn;

pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize,
}

pub struct Shader {
    id: u32,
    gl: gl::Gl,
}

impl Shader {
    pub fn new(gl: gl::Gl, ty: ShaderType) -> Option<Self> {
        let shader = unsafe { gl.CreateShader(ty as u32) };
        if shader != 0 {
            Some(Self { id: shader, gl })
        } else {
            None
        }
    }

    pub fn set_source(&self, src: &str) {
        unsafe {
            self.gl.ShaderSource(
                self.id,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap()),
            );
        }
    }

    pub fn compile(&self) {
        unsafe { self.gl.CompileShader(self.id) };
    }

    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe {
            self.gl
                .GetShaderiv(self.id, gl::COMPILE_STATUS, &mut compiled)
        };
        compiled == i32::from(gl::TRUE)
    }

    fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe {
            self.gl
                .GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut needed_len)
        };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            self.gl.GetShaderInfoLog(
                self.id,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn delete(&self) {
        unsafe { self.gl.DeleteShader(self.id) };
    }

    pub fn from_source(gl: gl::Gl, ty: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(gl, ty).ok_or_else(|| "Couldn't allocate new shader".to_string())?;
        id.set_source(source);
        id.compile();
        if id.compile_success() {
            Ok(id)
        } else {
            let out = id.info_log();
            id.delete();
            Err(out)
        }
    }

    pub fn from_file(gl: gl::Gl, ty: ShaderType, filename: &str) -> Result<Self, String> {
        let mut file = File::open(filename).expect("Could not open shader file");

        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .expect("Could not read file");

        Self::from_source(gl, ty, buffer.as_str())
    }
}

pub struct ShaderProgram {
    pub id: u32,
    gl: gl::Gl,
}
impl ShaderProgram {
    pub fn new(gl: gl::Gl) -> Option<Self> {
        let prog = unsafe { gl.CreateProgram() };
        if prog != 0 {
            Some(Self { id: prog, gl })
        } else {
            None
        }
    }

    pub fn from_shaders(gl: gl::Gl, shaders: Vec<Shader>) -> Result<Self, String> {
        let program = Self::new(gl).expect("Could not create program");

        shaders.iter().for_each(|shader| {
            program.attach_shader(shader);
        });
        program.link_program();
        shaders.iter().for_each(|shader| {
            shader.delete();
        });

        if program.link_success() {
            Ok(program)
        } else {
            let out = format!("Program Link Error: {}", program.info_log());
            program.delete();
            Err(out)
        }
    }

    fn attach_shader(&self, shader: &Shader) {
        unsafe { self.gl.AttachShader(self.id, shader.id) };
    }

    fn link_program(&self) {
        unsafe { self.gl.LinkProgram(self.id) };
    }

    fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { self.gl.GetProgramiv(self.id, gl::LINK_STATUS, &mut success) };
        success == i32::from(gl::TRUE)
    }

    fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe {
            self.gl
                .GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut needed_len)
        };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            self.gl.GetProgramInfoLog(
                self.id,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn use_program(&self) {
        unsafe { self.gl.UseProgram(self.id) };
    }

    pub fn delete(&self) {
        unsafe { self.gl.DeleteProgram(self.id) };
    }

    pub fn from_vert_frag(gl: gl::Gl, vert: &str, frag: &str) -> Result<Self, String> {
        let p = Self::new(gl.clone()).ok_or_else(|| "Couldn't allocate a program".to_string())?;
        let v = Shader::from_source(gl.clone(), ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex Compile Error: {}", e))?;
        let f = Shader::from_source(gl.clone(), ShaderType::Fragment, frag)
            .map_err(|e| format!("Fragment Compile Error: {}", e))?;
        p.attach_shader(&v);
        p.attach_shader(&f);
        p.link_program();
        v.delete();
        f.delete();
        if p.link_success() {
            Ok(p)
        } else {
            let out = format!("Program Link Error: {}", p.info_log());
            p.delete();
            Err(out)
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> i32 {
        unsafe { self.gl.GetUniformLocation(self.id, name.as_ptr().cast()) }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        self.delete();
    }
}

pub struct ShaderUniform<T> {
    id: i32,
    gl: Gl,
    phantom: PhantomData<T>,
}

impl<T> ShaderUniform<T> {
    pub fn load(gl: Gl, program: &ShaderProgram, name: &str) -> Self {
        // OpenGL needs 0 terminated strings
        let null_name = name.to_owned() + "\0";
        let id = program.get_uniform_location(&null_name);
        if id == -1 {
            warn!("Could not find uniform variable '{name}'. This might be due to an optimization from GLSL.");
        }
        // OpenGL ignores every id < 0
        Self {
            id,
            gl,
            phantom: PhantomData,
        }
    }
}

impl ShaderUniform<f32> {
    pub fn set(&self, value: f32) {
        unsafe {
            self.gl.Uniform1f(self.id, value);
        }
    }
}

impl ShaderUniform<[f32; 2]> {
    pub fn set(&self, value: [f32; 2]) {
        unsafe {
            self.gl.Uniform2f(self.id, value[0], value[1]);
        }
    }
}

impl ShaderUniform<Vec2> {
    pub fn set(&self, value: Vec2) {
        unsafe {
            self.gl.Uniform2f(self.id, value.x, value.y);
        }
    }
}

impl ShaderUniform<Vec3> {
    pub fn set(&self, value: Vec3) {
        unsafe {
            self.gl.Uniform3f(self.id, value.x, value.y, value.z);
        }
    }
}

impl ShaderUniform<glam::Mat4> {
    pub fn set(&self, value: glam::Mat4) {
        unsafe {
            self.gl
                .UniformMatrix4fv(self.id, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
    }
}
