use std::num::Float;
use gl::types::*;

use object;

pub struct Floor {
    pub x: GLfloat,
    pub y: GLfloat,
    pub z: GLfloat,

    model: Object,
}

impl Floor {
    pub fn get_model(&mut self) -> Object {
        self.model
    }
}
