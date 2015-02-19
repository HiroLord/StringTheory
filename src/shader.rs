use gl;

use gl::types::*;
use std::ptr;
use std::str;
use std::ffi;

pub struct Shader {
    program: GLuint,
}


impl Shader {
    pub fn get_attrib(&self, attrib: &str) -> u32 { unsafe { gl::GetAttribLocation(self.program,
                                                                ffi::CString::from_slice(attrib.as_bytes()).as_ptr()) as u32 } }
    pub fn get_uniform(&self, uniform: &str) -> i32 { unsafe { gl::GetUniformLocation(self.program,
                                                                ffi::CString::from_slice(uniform.as_bytes()).as_ptr()) as i32 } }
    pub fn bind(&self) -> () {
        unsafe {
            gl::UseProgram(self.program);
            gl::GetAttribLocation(self.program, ffi::CString::from_slice("out_color".as_bytes()).as_ptr());
        }
    }
}

pub fn new(VS_SRC: &str, FS_SRC: &str) -> Shader {
    let vertexShader = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fragmentShader = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vertexShader, fragmentShader);
    Shader {program: program}
}


fn compile_shader(src: &str, ty:GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        gl::ShaderSource(shader, 1, &ffi::CString::from_slice(src.as_bytes()).as_ptr(), ptr::null());
        gl::CompileShader(shader);
        // Get the status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        
        // If there was an error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf: Vec<u8> = Vec::with_capacity((len+1) as usize);
            buf.resize((len+1) as usize, 0);
            gl::GetShaderInfoLog(shader, len, &mut len, buf.as_mut_ptr() as *mut GLchar);
            let log = str::from_utf8(buf.as_slice()).unwrap();
            panic!("{}", log);
        }
    }
    shader
}

fn link_program(vertexShader: GLuint, fragmentShader: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertexShader);
        gl::AttachShader(program, fragmentShader);
        gl::LinkProgram(program);
        // Link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf: Vec<u8> = Vec::with_capacity((len+1) as usize);
            buf.resize((len+1) as usize, 0);
            gl::GetProgramInfoLog(program, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);
            let log = str::from_utf8(buf.as_slice()).unwrap();
            panic!("{}", log);
        }
        program
    }
}

