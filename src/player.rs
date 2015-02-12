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

    pub fn forward(&mut self, c: &Camera, mut dz: GLfloat) {
        if dz > 0f32 {
            dz = 1f32;
        }
        else if dz < 0f32 {
            dz = -1f32;
        }

        let adx = -c.horizontal_angle.sin() * 0.15f32 * self.speed * dz;
        let adz = -c.horizontal_angle.cos() * 0.15f32 * self.speed * dz;
        self.move_self(adx, adz);
    }

    pub fn strafe(&mut self, c: &Camera, mut dx: GLfloat) {
        if dx > 0f32 {
            dx = 1f32;
        }
        else if dx < 0f32 {
            dx = -1f32;
        }
        let adx = c.horizontal_angle.cos() * 0.15f32 * dx * self.speed;
        let adz = -c.horizontal_angle.sin() * 0.15f32  * dx * self.speed;
        self.move_self(adx, adz);
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

