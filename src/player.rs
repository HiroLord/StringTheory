use std::num::Float;
use gl::types::*;
//use std::num::sqrt;

use solids::Wall;
use solids::Solid;
use camera::Camera;
use solids::GameObject;
use solids::Mask;
use solids;
use light;
use renderer;
//use std::num::abs;

pub struct Player {
    pub x: GLfloat,
    pub y: GLfloat,
    pub z: GLfloat,
   
    mask: Mask,

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

impl GameObject for Player {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }
    fn draw(&self, c: &Camera, renderer: &renderer::Renderer) {}
}

impl Solid for Player {
    fn get_mask(&self) -> &Mask { &(self.mask) }
}

impl Player {

    pub fn get_move(&self) -> (f32, f32) {
        (self.movement.x * 0.1f32 * self.speed, self.movement.z * 0.1f32 * self.speed)
    }

    pub fn move_x(&mut self, dx: f32) {
        self.x += dx;
        self.set_position();
    }

    pub fn move_z(&mut self, dz: f32) {
        self.z += dz;
        self.set_position();
    }

    pub fn set_position(&mut self) {
        self.mask.set_pos(self.x, self.y, self.z);
    }

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
        self.set_position();
    }

    pub fn set_z(&mut self, z: f32) {
        self.z = z;
        self.set_position();
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

pub fn new(x: GLfloat, height: GLfloat, z: GLfloat, speed: GLfloat) -> Player {
    let size = 0.78f32*2.0;
    let mask = solids::new_mask(size, size);
    Player{ x: x, y: height, z: z, mask: mask, fb: 0f32, lr: 0f32, movement: Vect{x: 0f32, y: 0f32, z: 0f32}, speed: speed }
}

