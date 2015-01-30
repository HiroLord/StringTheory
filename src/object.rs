use shader;
use gl;

use sdl2::video::{Window, WindowPos, OPENGL, gl_set_attribute};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;
use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi;
use collections::vec;

static VS_SRC: &'static str =
    "#version 150\n\
    in vec3 position;\n\
    void main() {\n\
        gl_Position = vec4(position, 1.0);\n\
    }";

static FS_SRC: &'static str =
    "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
        out_color = vec4(1.0, 0.5, 0.5, 1.0);\n\
    }";


static VERTEX_DATA: [GLfloat; 9] = [
    0.0, 0.5, 0.0,
    0.5, -0.5, 0.0,
    -0.5, -0.5, 0.0
];


pub struct Object {
    shader: shader::Shader,
    vao: u32,
    vbo: u32,
}

impl Object {
    pub fn draw(&self) -> () {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(self.vao);
            self.shader.bind();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

pub fn new()  -> Object {
    let shader = shader::new(VS_SRC, FS_SRC);
    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        // create vertex array obj
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // create vertex buffer obj
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       mem::transmute(&VERTEX_DATA[0]),
                       gl::STATIC_DRAW);
    }
    Object { shader: shader, vao: vao, vbo: vbo }
}




