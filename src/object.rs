use shader;
use gl;
use camera;
use matrix;
use renderer;

use gl::types::*;
use std::mem;
use std::ptr;


static VS_SRC: &'static str = "
#version 120
attribute vec3 vert_model;
attribute vec3 norm_model;

uniform mat4 modelMatrix;
uniform mat4 viewProjectionMatrix;

varying vec4 position_modelSpace;
varying vec4 normal_modelSpace;

void main() {
    gl_Position = viewProjectionMatrix * modelMatrix * vec4(vert_model, 1);
    //gl_Position = viewProjectionMatrix * vec4(vert_model, 1);
    //gl_Position = vec4(vert_model, 1);
    position_modelSpace = modelMatrix * vec4(vert_model, 1);
    //normal_modelSpace = normalize(modelMatrix * vec4(norm_model, 1));
    normal_modelSpace = vec4(norm_model, 1);
}
    ";

static FS_SRC: &'static str = "

#version 120

uniform vec3 material_color;
uniform float alpha;

varying vec4 position_modelSpace;
varying vec4 normal_modelSpace;

void main() {
    gl_FragData[0] = vec4(material_color, 1);

    //gl_FragData[1] = position_modelSpace;
    //gl_FragData[2] = normal_modelSpace;

    gl_FragData[1] = position_modelSpace;
    gl_FragData[2] = normal_modelSpace;

    gl_FragData[3] = vec4(1,1,0,1);
    //gl_FragData[3] = (position_modelSpace/20 + normal_modelSpace) * vec4(material_color, 1);
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

    model_matrix: matrix::Matrix,

    num_indx: u32,

    visible: bool,

    shader: shader::Shader,

    vao: u32,
    vert_buff: u32,
    norm_buff: u32,
    texc_buff: u32,
    indx_buff: u32,
    is_light: bool,
}

impl Object {
    pub fn set_translation(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) -> () {
        self.x = x;
        self.y = y;
        self.z = z;
        self.model_matrix.set_translation(x,y,z);
    }
    pub fn translate(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) -> () {
        self.x += x;
        self.y += y;
        self.z += z;
        self.model_matrix.set_translation(self.x,self.y,self.z);
    }
    pub fn draw(&self, camera:&camera::Camera, renderer: &renderer::Renderer) -> () {
        unsafe {
            gl::BindVertexArray(self.vao);

            //self.shader.bind();
            let position_handle = self.shader.get_attrib("vert_model");
            //let normal_handle = self.shader.get_attrib("norm_model");
            
            if self.is_light {
                // bind our deferred textures
                let diffuse_tex = self.shader.get_uniform("diffuse_tex");
                let position_tex = self.shader.get_uniform("position_tex");
                let normal_tex = self.shader.get_uniform("normal_tex");
                //let last_tex = self.shader.get_uniform("last_tex");
                gl::Uniform1i(diffuse_tex, renderer.gbuff.textures[0] as i32); 
                gl::Uniform1i(position_tex, renderer.gbuff.textures[1] as i32); 
                gl::Uniform1i(normal_tex, renderer.gbuff.textures[2] as i32); 
                //gl::Uniform1i(last_tex, renderer.gbuff.textures[3] as i32); 

                // we also pass in our world pos and window size
                let light_world_pos = self.shader.get_uniform("light_world_pos");
                gl::Uniform3f(light_world_pos, self.x, self.y, self.z);

                let window_size_pos = self.shader.get_uniform("window_size");
                gl::Uniform2f(window_size_pos, renderer.winx as f32, renderer.winy as f32);
            }

            let normal_handle = self.shader.get_attrib("norm_model");

            let model_handle = self.shader.get_uniform("modelMatrix");
            gl::UniformMatrix4fv(model_handle, 1, gl::FALSE, mem::transmute(&self.model_matrix.data[0]));

            let view_projection_handle = self.shader.get_uniform("viewProjectionMatrix");
            gl::UniformMatrix4fv(view_projection_handle, 1, gl::FALSE, mem::transmute(&camera.view_projection.data[0]));

            let material_color_handle = self.shader.get_uniform("material_color");
            gl::Uniform3f(material_color_handle, self.r, self.g, self.b);

            let alpha_handle = self.shader.get_uniform("alpha");
            gl::Uniform1f(alpha_handle, 1.0f32);

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

    pub fn bind_shader(&self){
        self.shader.bind();
    }
}

pub fn new_tri(r:f32, g:f32, b:f32)  -> Object {
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
    let mut indxs: [u32; 3] = [0; 3];
    for i in 0..(3) { indxs[i] = i as u32; }
    generate(&verts, &norms, &indxs, r, g, b)
}

pub fn new(x1:f32, y1:f32, z1:f32, x2:f32, y2:f32, z2:f32, r:f32, g:f32, b:f32)  -> Object {
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
        // Front face
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,

        // Top face
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,

        // Back face
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,

        // Bottom face
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,

        // Left face
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,

        // Right face
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
            ];
    let mut indxs: [u32; 6*6] = [0; 6*6];
    for i in 0..(6*6) { indxs[i] = i as u32; }
    generate(&verts, &norms, &indxs, r, g, b)
}

pub fn generate(verts: &[GLfloat], norms: &[GLfloat], indxs: &[u32], r:f32, g:f32, b:f32) -> Object {
    generate_general(verts, norms, indxs, r, g, b, VS_SRC, FS_SRC, false)
}

pub fn generate_general(verts: &[GLfloat], norms: &[GLfloat], indxs: &[u32], r:f32, g:f32, b:f32,
                        vertex_shader: &str, fragment_shader: &str, is_light: bool) -> Object {
    let shader = shader::new(vertex_shader, fragment_shader);
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

        r: r,
        g: g,
        b: b,

        model_matrix: matrix::new(),

        num_indx: indxs.len() as u32,

        visible: true,

        shader: shader,
        vao: vao,
        vert_buff: vert_buff,
        norm_buff: norm_buff,
        texc_buff: 0,
        indx_buff: indx_buff,
        is_light: is_light,
    }
}

pub fn new2(x1:f32, y1:f32, z1:f32, x2:f32, y2:f32, z2:f32, r:f32, g:f32, b:f32, s:shader::Shader)  -> Object {
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
        // Front face
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,

        // Top face
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,
        0.0f32, 1.0f32, 0.0f32,

        // Back face
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,
        0.0f32, 0.0f32, -1.0f32,

        // Bottom face
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,
        0.0f32, -1.0f32, 0.0f32,

        // Left face
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,
        -1.0f32, 0.0f32, 0.0f32,

        // Right face
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
        1.0f32, 0.0f32, 0.0f32,
            ];
    let mut indxs: [u32; 6*6] = [0; 6*6];
    for i in 0..(6*6) { indxs[i] = i as u32; }
    generate2(&verts, &norms, &indxs, r, g, b, s, false)
}

pub fn generate2(verts: &[GLfloat], norms: &[GLfloat], indxs: &[u32], r:f32, g:f32, b:f32,
                        s : shader::Shader, is_light: bool) -> Object {
    let mut vert_buff:u32 = 0;
    let mut norm_buff:u32 = 0;
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

        r: r,
        g: g,
        b: b,

        model_matrix: matrix::new(),

        num_indx: indxs.len() as u32,

        visible: true,

        shader: s,
        vao: vao,
        vert_buff: vert_buff,
        norm_buff: norm_buff,
        texc_buff: 0,
        indx_buff: indx_buff,
        is_light: is_light,
    }
}

