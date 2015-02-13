use std::num::Float;
use gl::types::*;
//use std::num::sqrt;

use camera::Camera;

pub struct Player {
    pub x: GLfloat,
    pub y: GLfloat,
    pub z: GLfloat,

    fb: GLfloat,
    lr: GLfloat,

    movement: Vect,

    speed: GLfloat,
}

struct Vect {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
}

impl Player {
    pub fn move_self(&mut self) {
        self.x += self.movement.x * 0.1f32 * self.speed;
        self.z += self.movement.z * 0.1f32 * self.speed;
    }

    pub fn forward(&mut self, c: &Camera, mut dz: GLfloat) {
        if dz > 1f32 {
            dz = 1f32;
        }
        else if dz < -1f32 {
            dz = -1f32;
        }

        self.fb = -dz;
        
        self.set_vector(c);
    }

    fn set_vector(&mut self, c: &Camera) {
        self.movement.x = c.horizontal_angle().sin() * self.fb +
            c.horizontal_angle().cos() * self.lr;

        self.movement.z =  c.horizontal_angle().cos() * self.fb - 
            c.horizontal_angle().sin() * self.lr;

        let mag = (self.movement.x * self.movement.x + self.movement.z * self.movement.z).sqrt() as
            f32;
        if mag > 1f32 {
            self.movement.x /= mag;
            self.movement.z /= mag;
        }
    }

    pub fn strafe(&mut self, c: &Camera, mut dx: GLfloat) {
        if dx > 1f32 {
            dx = 1f32;
        }
        else if dx < -1f32 {
            dx = -1f32;
        }

        self.lr = dx;

        self.set_vector(c);
    }

    /*
    pub fn move_from_camera(&mut self, c: &Camera, dx: GLfloat, dz: GLfloat) {
        let adx = c.horizontal_angle.cos() * dx + c.horizontal_angle.sin() * dz;
        let adz = -c.horizontal_angle.sin() * dx + c.horizontal_angle.cos() * dz;
        self.move_self(adx, adz);
    }
    */
}

pub fn new(x: GLfloat, y: GLfloat, z: GLfloat, speed: GLfloat) -> Player {
    Player{ x: x, y: y, z: z, fb: 0f32, lr: 0f32, movement: Vect{x: 0f32, y: 0f32, z: 0f32}, speed: speed }
}

