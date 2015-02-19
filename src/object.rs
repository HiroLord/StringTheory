use shader;
use gl;
use camera;
use matrix;
use light;

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

static FS_SRC: &'static str = "\n\
#version 120

const int max_lights = 4;
uniform vec3 light_pos[max_lights];
uniform vec3 light_color[max_lights];

uniform vec3 material_color;
uniform float alpha;

varying vec4 position_modelSpace;
varying vec4 normal_modelSpace;

void main() {
    for (int i = 0; i < max_lights; i++) {
        vec4 light_pos_4 = vec4(light_pos[i], 1);

        // I don't think I should have to negate this....
        vec4 vecToLight = -normalize(position_modelSpace - light_pos_4);
        float cosTheta = clamp( dot(normal_modelSpace, vecToLight), 0, 1);
        float dist = distance(position_modelSpace, light_pos_4); 
        gl_FragColor += vec4(material_color * vec3(0.3,0.3,0.3) + (cosTheta * material_color * light_color[i]) / (dist), alpha);
    }
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
    pub fn draw(&self, camera:&camera::Camera, lights:&[light::Light]) -> () {
        unsafe {
            gl::BindVertexArray(self.vao);

            self.shader.bind();
            let position_handle = self.shader.get_attrib("vert_model");
            let normal_handle = self.shader.get_attrib("norm_model");
            
            for i in 0..4 {
                let pos_loc = self.shader.get_uniform(&format!("light_pos[{}]", i));
                let color_loc = self.shader.get_uniform(&format!("light_color[{}]", i));
                println!("pos_loc is {}, color_loc is {}", pos_loc, color_loc);
                println!("for ||{}||", &format!("light_pos[{}]", i));
                if (lights.len() > i) {
                    println!("Using light {}, pos: {} {} {} col: {} {} {}", i, lights[i].x, lights[i].y, lights[i].z,
                                                                               lights[i].r, lights[i].g, lights[i].b);
                    gl::Uniform3f(pos_loc, lights[i].x, lights[i].y, lights[i].z);
                    gl::Uniform3f(color_loc, lights[i].r, lights[i].g, lights[i].b);
                } else {
                    println!("Using 0 defaults");
                    gl::Uniform3f(pos_loc, 0.0f32, 0.0f32, 0.0f32);
                    gl::Uniform3f(color_loc, 4.0f32, 0.0f32, 0.0f32);
                }
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
}

pub fn newTri(r:f32, g:f32, b:f32)  -> Object {
    let shader = shader::new(VS_SRC, FS_SRC);
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
    generate(shader, &verts, &norms, &indxs, r, g, b)
}

pub fn new(x1:f32, y1:f32, z1:f32, x2:f32, y2:f32, z2:f32, r:f32, g:f32, b:f32)  -> Object {
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
    generate(shader, &verts, &norms, &indxs, r, g, b)
}

fn generate(shader: shader::Shader, verts: &[GLfloat], norms: &[GLfloat], indxs: &[u32], r:f32, g:f32, b:f32) -> Object {
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
    }
}


