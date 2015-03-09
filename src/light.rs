use object;
use shader;
use gl::types::*;

pub fn new_light(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, shader: shader::Shader) -> object::Object {
    let scale = r+g+b;
    let mut obj = object::new(scale*-1.0, scale*-1.0, scale*-1.0, scale*1.0, scale*1.0, scale*1.0, r, g, b, shader, true);
    obj.set_translation(x,y,z);
    obj
}
