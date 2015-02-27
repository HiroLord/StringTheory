use gl;

use gl::types::*;
use std::ptr;
use std::str;
use std::ffi;

pub struct Shader {
    program: GLuint,
}


impl Shader {
    /*pub fn get_attrib(&self, attrib: &str) -> u32 { unsafe { gl::GetAttribLocation(self.program, 
                                                                ffi::CString::from_slice(attrib.as_bytes()).as_ptr()) as u32 } }*/
    pub fn get_attrib(&self, attrib: &str) -> u32 { unsafe { gl::GetAttribLocation(self.program,
                                                                ffi::CString::new(attrib).unwrap().as_ptr()) as u32 } }
 
    /*pub fn get_uniform(&self, uniform: &str) -> i32 { unsafe { gl::GetUniformLocation(self.program,
                                                                ffi::CString::from_slice(uniform.as_bytes()).as_ptr()) as i32 } }*/
    pub fn get_uniform(&self, uniform: &str) -> i32 { unsafe { gl::GetUniformLocation(self.program,
                                                                ffi::CString::new(uniform).unwrap().as_ptr()) as i32 } }
    pub fn bind(&self) -> () {
        //println!("Binding shader!");
        unsafe {
            gl::UseProgram(self.program);
            //gl::GetAttribLocation(self.program, ffi::CString::from_slice("out_color".as_bytes()).as_ptr());
            gl::GetAttribLocation(self.program, ffi::CString::new("out_color").unwrap().as_ptr());
        }
    }

    pub fn get_program(&self) -> u32 {
        return self.program;
    }
}

pub fn new(VS_SRC: &str, FS_SRC: &str) -> Shader {
    let vertex_shader = compile_shader(VS_SRC, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let program = link_program(vertex_shader, fragment_shader);
    Shader {program: program}
}

pub fn new2(s:u32) -> Shader {
    Shader { program: s}
}

fn compile_shader(src: &str, ty:GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        gl::ShaderSource(shader, 1, &ffi::CString::new(src).unwrap().as_ptr(), ptr::null());
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

fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
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

