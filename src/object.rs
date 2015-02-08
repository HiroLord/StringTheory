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

static VS_SRC_S: &'static str =
    "#version 150\n\
    in vec3 vert_model;\n\
    void main() {\n\
        gl_Position = vec4(vert_model, 1.0);\n\
    }";

static FS_SRC_S: &'static str =
    "#version 150\n\
    out vec4 out_color;\n\
    void main() {\n\
        out_color = vec4(1.0, 0.5, 0.5, 1.0);\n\
    }";

static VS_SRC: &'static str = "\n\
#version 120
attribute vec3 vert_model;
attribute vec3 norm_model;

//uniform mat4 modelMatrix;
//uniform mat4 viewProjectionMatrix;

//varying vec4 position_modelSpace;
varying vec4 normal_modelSpace;

void main() {
    //gl_Position = viewProjectionMatrix * modelMatrix * vec4(vertPos_model, 1);
    gl_Position = vec4(vert_model, 1);
    //position_modelSpace = modelMatrix * vec4(vertPos_model, 1);
    //normal_modelSpace = normalize(modelMatrix * vec4(norm_model, 1));
    normal_modelSpace = vec4(norm_model, 1);
}
    ";

static FS_SRC: &'static str = "\n\
#version 120

//uniform vec3 materialColor;
//uniform float alpha;

//varying vec4 position_modelSpace;
varying vec4 normal_modelSpace;

void main() {
    //vec4 light_pos = vec4(0, 40, 0, 1);
    //vec3 light_color = vec3(30,30,30);

    //vec3 matDiffuseColor = vec3(0.9, 0.9, 0.9);

    //float cosTheta = clamp( dot(normal_modelSpace, light_pos), 0, 1);
    //float dist = distance(position_modelSpace, light_pos); 
    //gl_FragColor =   vec4(materialColor * vec3(0.3,0.3,0.3) + (cosTheta * materialColor * light_color) / (dist), alpha);
    gl_FragColor =   normal_modelSpace;
    //gl_FragColor =   vec4(1,1,0,1);
}
    ";


pub struct Object {
    x: f32,
    y: f32,
    z: f32,

    rx: f32,
    ry: f32,
    rz: f32,

    r: f32,
    g: f32,
    b: f32,

    num_indx: u32,

    visible: bool,

    shader: shader::Shader,
    vao: u32,
    vert_buff: u32,
    norm_buff: u32,
    texc_buff: u32,
    indx_buff: u32,
}

impl Object {
    pub fn draw(&self) -> () {
        unsafe {
            gl::BindVertexArray(self.vao);

            self.shader.bind();
            let position_handle = self.shader.get_attrib("vert_model");
            let normal_handle = self.shader.get_attrib("norm_model");

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vert_buff);
            // attribute, size, type, normalized, stride, offset
            gl::EnableVertexAttribArray(position_handle);
            gl::VertexAttribPointer(position_handle, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());

            gl::BindBuffer(gl::ARRAY_BUFFER, self.norm_buff);
            // attribute, size, type, normalized, stride, offset
            gl::VertexAttribPointer(normal_handle, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
            gl::EnableVertexAttribArray(normal_handle);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.indx_buff);
            gl::DrawElements(gl::TRIANGLES, self.num_indx as i32, gl::UNSIGNED_INT, ptr::null());

            gl::DisableVertexAttribArray(position_handle);
            gl::DisableVertexAttribArray(normal_handle);
        }
    }
}

pub fn newTri()  -> Object {
    //let shader = shader::new(VS_SRC, FS_SRC);
    let shader = shader::new(VS_SRC_S, FS_SRC_S);
    let verts: [GLfloat; 9] = [
        0.0, 0.5, 0.0,
        0.5, -0.5, 0.0,
        -0.5, -0.5, 0.0
    ];
    let norms: [GLfloat; 9] = [
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
            ];
    let mut indxs: [u32; 9] = [0; 9];
    for i in 0..(9) { indxs[i] = i as u32; }
    generate(shader, &verts, &norms, &indxs)
}

pub fn new(x1:f32, y1:f32, z1:f32, x2:f32, y2:f32, z2:f32)  -> Object {
    let shader = shader::new(VS_SRC, FS_SRC);
    let verts: [GLfloat; 6*6*3] = [
        // Front face
        x1, y1, z1,
        x2, y2, z1,
        x1, y2, z1,
        x1, y1, z1,
        x2, y1, z1,
        x2, y2, z1,

        // Top face
        x1, y2, z1,
        x2, y2, z2,
        x1, y2, z2,
        x1, y2, z1,
        x2, y2, z1,
        x2, y2, z2,

        // Back face
        x2, y1, z2,
        x1, y2, z2,
        x2, y2, z2,
        x2, y1, z2,
        x1, y1, z2,
        x1, y2, z2,

        // Bottom face
        x1, y1, z2,
        x2, y1, z1,
        x1, y1, z1,
        x1, y1, z2,
        x2, y1, z2,
        x2, y1, z1,

        // Left face
        x1, y1, z2,
        x1, y2, z1,
        x1, y2, z2,
        x1, y1, z2,
        x1, y1, z1,
        x1, y2, z1,

        // Right face
        x2, y1, z1,
        x2, y2, z2,
        x2, y2, z1,
        x2, y1, z1,
        x2, y1, z2,
        x2, y2, z2,
            ];
    let norms: [GLfloat; 6*6*3] = [
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,

        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,

        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,

        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,

        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,

        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
            ];
    let mut indxs: [u32; 6*6*3] = [0; 6*6*3];
    for i in 0..(6*6*3) { indxs[i] = i as u32; }
    generate(shader, &verts, &norms, &indxs)
}

fn generate(shader: shader::Shader, verts: &[GLfloat], norms: &[GLfloat], indxs: &[u32]) -> Object {
    let mut vert_buff:u32 = 0;
    let mut norm_buff:u32 = 0;
    //let mut vert_buff:u32;
    let mut indx_buff:u32 = 0;
    let mut vao = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vert_buff);
        gl::GenBuffers(1, &mut norm_buff);
        //gl::GenBuffers(1, &mut texc_buff);
        gl::GenBuffers(1, &mut indx_buff);

        gl::BindBuffer(gl::ARRAY_BUFFER, vert_buff);
        gl::BufferData(gl::ARRAY_BUFFER, (verts.len()*mem::size_of::<GLfloat>()) as GLsizeiptr,
                        mem::transmute(&verts[0]), gl::STATIC_DRAW);
        gl::BindBuffer(gl::ARRAY_BUFFER, norm_buff);
        gl::BufferData(gl::ARRAY_BUFFER, (norms.len()*mem::size_of::<GLfloat>()) as GLsizeiptr,
                        mem::transmute(&norms[0]), gl::STATIC_DRAW);
        //gl::BindBuffer(gl::ARRAY_BUFFER, texc_buff);
        //gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (.len()*mem::size_of::<GLfloat>()) as GLsizeiptr,
                        //mem::transmute(&[0]), gl::STATIC_DRAW);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, indx_buff);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indxs.len()*mem::size_of::<GLfloat>()) as GLsizeiptr,
                        mem::transmute(&indxs[0]), gl::STATIC_DRAW);

    }
    Object {
        x: 0f32,
        y: 0f32,
        z: 0f32,

        rx: 0f32,
        ry: 0f32,
        rz: 0f32,

        r: 0f32,
        g: 0f32,
        b: 0f32,

        num_indx: indxs.len() as u32,

        visible: true,

        shader: shader,
        vao: vao,
        vert_buff: vert_buff,
        norm_buff: norm_buff,
        texc_buff: 0,
        indx_buff: indx_buff,
    }
}


