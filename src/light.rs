use object;
use shader;
use gl::types::*;
use std::num::Float;

pub fn new_light(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, shader: shader::Shader) -> object::Object {
    let cutoff = 0.1;
    //let scale = ((r+g+b)/(cutoff*3.0));
    let scale = ((r+g+b)/cutoff*3.0).sqrt();
    let mut obj = object::new(scale*-1.0, scale*-1.0, scale*-1.0, scale*1.0, scale*1.0, scale*1.0, r, g, b, shader, true);
    obj.set_translation(x,y,z);
    obj
}
