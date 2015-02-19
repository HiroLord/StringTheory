
use gl::types::*;

pub struct Light {
    pub x: GLfloat,
    pub y: GLfloat,
    pub z: GLfloat,

    pub r: GLfloat,
    pub g: GLfloat,
    pub b: GLfloat,
}

pub fn new_light(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> Light{
    Light{x: x, y: y, z: z, r: r, g: g, b: b}
}

