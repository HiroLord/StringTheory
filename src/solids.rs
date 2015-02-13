use std::num::Float;
use gl::types::*;

use object;
use camera::Camera;

pub struct Floor {
    pub x: GLfloat,
    pub y: GLfloat,
    pub z: GLfloat,

    model: object::Object,
}

impl Floor {
    pub fn set_position(&mut self) {
        self.model.translate(self.x, self.y, self.z);
    }

    pub fn draw(&mut self, c: &Camera) {
        self.model.draw(c);
    }
}

pub fn new_floor(x: f32, y: f32, z: f32) -> Floor {

    let m = object::new(-1.0, -0.1, 1.0,  1.0, 0.1, -1.0,  0.6, 0.6, 0.8);

    let mut f = Floor{x: x, y: y, z:z, model: m};
    f.set_position();
    f
}
