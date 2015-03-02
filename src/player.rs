use std::num::Float;
use gl::types::*;
//use std::num::sqrt;

use object;
use solids::Solid;
use camera::Camera;
use solids::GameObject;
use solids::Mask;
use solids;
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

    old_fb: GLfloat,
    old_lr: GLfloat,

    horizontal_angle: GLfloat,
    vertical_angle: GLfloat,
    movement: Vect,

    old_horizontal_angle: GLfloat,
    old_vertical_angle: GLfloat,

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
        self.model.bind_shader();
        self.model.draw(c, renderer);
    }
}

impl Solid for Player {
    fn get_mask(&self) -> &Mask { &(self.mask) }
}

impl Player {

    pub fn get_move(&self, delta: f32) -> (f32, f32) {
        (self.movement.x * 0.1f32 * self.speed * delta, self.movement.z * 0.1f32 * self.speed *
         delta)
    }

    pub fn move_x(&mut self, dx: f32) {
        self.x += dx;
        self.set_position();
    }

    pub fn move_z(&mut self, dz: f32) {
        self.z += dz;
        self.set_position();
    }

    pub fn look_changed(&self) -> bool {
        !(self.old_horizontal_angle == self.horizontal_angle && self.vertical_angle == self.old_vertical_angle)
    }

    pub fn old_hor_angle(&self) -> f32 {
        self.old_horizontal_angle
    }

    pub fn old_ver_angle(&self) -> f32 {
        self.old_vertical_angle
    }

    pub fn make_old_look_new(&mut self) {
        self.old_horizontal_angle = self.horizontal_angle;
        self.old_vertical_angle = self.vertical_angle;
    }

    pub fn make_old_fb_new(&mut self) {
        self.old_lr = self.lr;
        self.old_fb = self.fb;
    }

    pub fn fb_changed(&self) {
    }

    pub fn fb_lr_changed(&self) -> bool {
        !(self.lr != self.old_lr && self.fb != self.old_fb)
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

    pub fn set_look_vector(&mut self, c: &Camera) {
        self.horizontal_angle = c.horizontal_angle();
        self.vertical_angle = c.vertical_angle();
    }

    pub fn set_horizontal_angle(&mut self, angle: f32) {
        self.horizontal_angle = angle;
    }

    pub fn set_vertical_angle(&mut self, angle: f32) {
        self.vertical_angle = angle;
    }

    pub fn hor_angle(&self) -> f32 {
        self.horizontal_angle
    }

    pub fn ver_angle(&self) -> f32 {
        self.vertical_angle
    }

    pub fn fb(&self) -> f32 { self.fb }
    pub fn lr(&self) -> f32 { self.lr }

    pub fn forward(&mut self, mut dz: GLfloat) {
        if dz > 1f32 {
            dz = 1f32;
        }
        else if dz < -1f32 {
            dz = -1f32;
        }

        self.fb = -dz;
        
        self.set_vector();
    }

    fn set_vector(&mut self) {
        self.movement.x = self.horizontal_angle.sin() * self.fb +
            self.horizontal_angle.cos() * self.lr;

        self.movement.z = self.horizontal_angle.cos() * self.fb - 
            self.horizontal_angle.sin() * self.lr;

        let mag = (self.movement.x * self.movement.x + self.movement.z * self.movement.z).sqrt() as
            f32;
        if mag > 1f32 {
            self.movement.x /= mag;
            self.movement.z /= mag;
        }
    }

    pub fn strafe(&mut self, mut dx: GLfloat) {
        if dx > 1f32 {
            dx = 1f32;
        }
        else if dx < -1f32 {
            dx = -1f32;
        }

        self.lr = dx;

        self.set_vector();
    }

    pub fn player_id(&self) -> u32 { self.player_id }

}

pub fn new(id: u32, x: GLfloat, height: GLfloat, z: GLfloat,  speed: GLfloat) -> Player {
    let size = 0.78f32*2.0;
    let height = 1.7f32;
    let model = object::new(-size/2.0, 0f32, size/2.0, size/2.0, height, -size/2.0, 1.0, 0.0, 0.5);
    let mask = solids::new_mask(size, size);
    Player{ player_id: id, x: x, y: height, z: z, model: model, height: height, mask: mask,
        fb: 0f32, lr: 0f32, old_fb: 0f32, old_lr: 0f32,
        horizontal_angle: 0f32, vertical_angle: 0f32, movement: Vect{x: 0f32, y: 0f32, z: 0f32},
        old_horizontal_angle: 0f32, old_vertical_angle: 0f32, speed: speed }
}

