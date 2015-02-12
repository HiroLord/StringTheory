use std::num::Float;
use gl::types::*;
//use std::num::sqrt;

use camera::Camera;

pub struct Player {
    pub x: GLfloat,
    pub y: GLfloat,
    pub z: GLfloat,

    speed: GLfloat,
}

impl Player {
    pub fn move_self(&mut self, dx: GLfloat, dz: GLfloat) {
        self.x += dx;
        self.z += dz;
    }

    pub fn move_from_camera(&mut self, c: &Camera, dx: GLfloat, dz: GLfloat) {
        let adx = c.horizontal_angle.cos() * dx + c.horizontal_angle.sin() * dz;
        let adz = -c.horizontal_angle.sin() * dx + c.horizontal_angle.cos() * dz;
        self.move_self(adx, adz);
    }
}

pub fn new(x: GLfloat, y: GLfloat, z: GLfloat, speed: GLfloat) -> Player {
    Player{ x: x, y: y, z: z, speed: speed }
}

