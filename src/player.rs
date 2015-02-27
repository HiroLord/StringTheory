use std::num::Float;
use gl::types::*;
//use std::num::sqrt;

use object;
use solids::Solid;
use camera::Camera;
use solids::GameObject;
use solids::Mask;
use solids;
use light;
use renderer;
use mapgen::Point;
//use std::num::abs;

pub struct Player {
    player_id: u32,

    pub x: GLfloat,
    pub y: GLfloat,
    pub z: GLfloat,

    model: object::Object,
    height: GLfloat,
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
    fn draw(&self, c: &Camera, renderer: &renderer::Renderer) {
        self.model.draw(c, renderer);
    }
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
        self.model.set_translation(self.x, self.height/2.0, self.z);
        self.mask.set_pos(self.x, self.height/2.0, self.z);
    }

    pub fn set_position_from_point(&mut self, point: &Point) {
        self.x = point.a;
        self.z = point.b;
        self.set_position();
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

    pub fn player_id(&self) -> u32 { self.player_id }

    /*
    pub fn move_from_camera(&mut self, c: &Camera, dx: GLfloat, dz: GLfloat) {
        let adx = c.horizontal_angle.cos() * dx + c.horizontal_angle.sin() * dz;
        let adz = -c.horizontal_angle.sin() * dx + c.horizontal_angle.cos() * dz;
        self.move_self(adx, adz);
    }
    */
}

pub fn new(id: u32, x: GLfloat, height: GLfloat, z: GLfloat,  speed: GLfloat) -> Player {
    let size = 0.78f32*2.0;
    let height = 1.7f32;
    let model = object::new(-size/2.0, 0f32, size/2.0, size/2.0, height, -size/2.0, 1.0, 0.0, 0.5);
    let mask = solids::new_mask(size, size);
    Player{ player_id: id, x: x, y: height, z: z, model: model, height: height, mask: mask, fb: 0f32, lr: 0f32, movement: Vect{x: 0f32, y: 0f32, z: 0f32}, speed: speed }
}

