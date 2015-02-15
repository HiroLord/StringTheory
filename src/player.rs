use std::num::Float;
use gl::types::*;
//use std::num::sqrt;

use solids::Wall;
use camera::Camera;
use solids::GameObject;
use solids::SolidObject;
//use std::num::abs;

pub struct Player {
    pub x: GLfloat,
    pub y: GLfloat,
    pub z: GLfloat,
    
    radius: GLfloat,

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

impl SolidObject for Player {
    fn get_left(&self) -> f32 { self.x - self.radius }
    fn get_right(&self) -> f32 { self.x + self.radius }
    fn get_front(&self) -> f32 { self.z + self.radius }
    fn get_back(&self) -> f32 { self.z - self.radius }
}

impl Player {
    pub fn move_self(&mut self, objs: &Vec<Wall>) {
        let dx = self.movement.x * 0.1f32 * self.speed;
        let dz = self.movement.z * 0.1f32 * self.speed;
        self.x += dx;
        let o = self.check_collisions(objs);
        
        if o > -1 {
            let i = o as usize;
            self.x -= dx;
            
            if self.x > objs[i].x(){
                self.x = objs[i].get_right() + self.radius;
            } else {
                self.x = objs[i].get_left() - self.radius;
            }
        }

        self.z += dz;
        let p = self.check_collisions(objs);
        
        if p > -1 {
            let i = p as usize;
            self.z -= dz;
            
            if self.z > objs[i].z() {
                self.z = objs[i].get_front() + self.radius;
            } else {
                self.z = objs[i].get_back() - self.radius;
            }
        }
        
    }

    fn check_collisions(&self, objs: &Vec<Wall>) -> i32 {
        for i in range(0, objs.len()) {
            if self.get_left() < objs[i].get_right() && self.get_right() > objs[i].get_left() {
                if self.get_back() < objs[i].get_front() && self.get_front() > objs[i].get_back()
                    {
                    return i as i32
                }
            }
            /*
            if objs[i].get_rotation() == 2.0 {
                if self.z < objs[i].z() + objs[i].width()/2.0 + self.radius && self.z > objs[i].z() -
                    objs[i].width()/2.0 - self.radius {
                    if self.x - objs[i].x() < self.radius && self.x > objs[i].x(){
                        self.x = objs[i].x() + self.radius;
                    } else if objs[i].x() - self.x < self.radius && self.x < objs[i].x() {
                        self.x = objs[i].x() - self.radius;
                    }
                }
            } else {
                if self.x > objs[i].x() - objs[i].width()/2.0 - self.radius && self.x < objs[i].x() +
                    objs[i].width()/2.0 + self.radius {
                    if self.z - objs[i].z() < self.radius && self.z > objs[i].z() {
                        self.z = objs[i].z() + self.radius;
                    } else if objs[i].z() - self.z < self.radius && self.z < objs[i].z() {
                        self.z = objs[i].z() - self.radius;
                    }
                }
            }
            */
        }
        -1
    }
    
    fn abs(f: f32) -> f32 {
        if f < 0.0 { return -f }
        f
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
    Player{ x: x, y: y, z: z, radius: 0.7f32, fb: 0f32, lr: 0f32, movement: Vect{x: 0f32, y: 0f32, z: 0f32}, speed: speed }
}

