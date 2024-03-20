use std::{fs::File, io::Read, marker::PhantomData};

pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize,
}

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(ty: ShaderType) -> Option<Self> {
        let shader = unsafe { gl::CreateShader(ty as u32) };
        if shader != 0 {
            Some(Self { id: shader })
        } else {
            None
        }
    }

    pub fn set_source(&self, src: &str) {
        unsafe {
            gl::ShaderSource(
                self.id,
                1,
                &(src.as_bytes().as_ptr().cast()),
                &(src.len().try_into().unwrap()),
            );
        }
    }

    pub fn compile(&self) {
        unsafe { gl::CompileShader(self.id) };
    }

    pub fn compile_success(&self) -> bool {
        let mut compiled = 0;
        unsafe { gl::GetShaderiv(self.id, gl::COMPILE_STATUS, &mut compiled) };
        compiled == i32::from(gl::TRUE)
    }

    fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetShaderiv(self.id, gl::INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            gl::GetShaderInfoLog(
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
        unsafe { gl::DeleteShader(self.id) };
    }

    pub fn from_source(ty: ShaderType, source: &str) -> Result<Self, String> {
        let id = Self::new(ty).ok_or_else(|| "Couldn't allocate new shader".to_string())?;
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

    pub fn from_file(ty: ShaderType, filename: &str) -> Result<Self, String> {
        let mut file = File::open(filename).expect("Could not open shader file");

        let mut buffer = String::new();

        file.read_to_string(&mut buffer)
            .expect("Could not read file");

        Self::from_source(ty, buffer.as_str())
    }
}

pub struct ShaderProgram(pub u32);
impl ShaderProgram {
    pub fn new() -> Option<Self> {
        let prog = unsafe { gl::CreateProgram() };
        if prog != 0 {
            Some(Self(prog))
        } else {
            None
        }
    }

    pub fn from_shaders(shaders: Vec<Shader>) -> Result<Self, String> {
        let program = Self::new().expect("Could not create program");

        shaders.iter().for_each(|shader| {
            program.attach_shader(&shader);
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
        unsafe { gl::AttachShader(self.0, shader.id) };
    }

    fn link_program(&self) {
        unsafe { gl::LinkProgram(self.0) };
    }

    fn link_success(&self) -> bool {
        let mut success = 0;
        unsafe { gl::GetProgramiv(self.0, gl::LINK_STATUS, &mut success) };
        success == i32::from(gl::TRUE)
    }

    fn info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe { gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len) };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            gl::GetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.0) };
    }

    pub fn delete(self) {
        unsafe { gl::DeleteProgram(self.0) };
    }

    pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
        let p = Self::new().ok_or_else(|| "Couldn't allocate a program".to_string())?;
        let v = Shader::from_source(ShaderType::Vertex, vert)
            .map_err(|e| format!("Vertex Compile Error: {}", e))?;
        let f = Shader::from_source(ShaderType::Fragment, frag)
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
}

pub struct ShaderUniform<T> {
    id: i32,
    phantom: PhantomData<T>,
}

impl<T> ShaderUniform<T> {
    pub fn load(program: &ShaderProgram, name: &str) -> Option<Self> {
        let id;
        unsafe {
            id = gl::GetUniformLocation(program.0, name.as_ptr().cast());
        }
        if id != -1 {
            Some(Self {
                id,
                phantom: PhantomData,
            })
        } else {
            None
        }
    }
}

impl ShaderUniform<f32> {
    pub fn set(&self, value: f32) {
        unsafe {
            gl::Uniform1f(self.id, value);
        }
    }
}

impl ShaderUniform<[f32; 2]> {
    pub fn set(&self, value: [f32; 2]) {
        unsafe {
            gl::Uniform2f(self.id, value[0], value[1]);
        }
    }
}

impl ShaderUniform<glam::Mat4> {
    pub fn set(&self, value: glam::Mat4) {
        unsafe {
            gl::UniformMatrix4fv(self.id, 1, gl::FALSE, &value.to_cols_array()[0]);
        }
    }
}

pub struct PlotShader {
    pub shader: ShaderProgram,
    pub offset: ShaderUniform<[f32; 2]>,
    pub pitch: ShaderUniform<[f32; 2]>,
    pub transform: ShaderUniform<glam::Mat4>,
}
