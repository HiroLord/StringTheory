use matrix;
use std::num::Float;
use gl::types::*;
use std::f32::consts::PI;

use player::Player;

const PI_3_2: f32 = PI*(3.0/2.0); //4.71238898039;
const PI_5_2: f32 = PI*(5.0/2.0); //7.85398163398;

pub struct Camera {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    horizontal_angle: GLfloat,
    vertical_angle: GLfloat,
    projection: matrix::Matrix,
    pub view_projection: matrix::Matrix,
}

impl Camera {
    pub fn update_view_projection(&mut self) {
        let view = matrix::rotated(-self.vertical_angle, -self.horizontal_angle, 0.0f32) * matrix::translated(-self.x,-self.y,-self.z);
        let mul = self.projection * view;
        for i in 0..16 { self.view_projection.data[i] = mul.data[i]; }
    }
    pub fn set_translation(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
    pub fn translate(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) {
        self.x += self.horizontal_angle.sin() * self.vertical_angle.cos() * z +
                  self.horizontal_angle.cos() * x +
                  self.horizontal_angle.sin() * self.vertical_angle.sin() * y;

        self.y += -self.vertical_angle.sin() * z +
                  self.vertical_angle.cos() * y;

        self.z += self.horizontal_angle.cos() * self.vertical_angle.cos() * z +
                  -self.horizontal_angle.sin() * x +
                  self.horizontal_angle.cos() * self.vertical_angle.sin() * y;
    }
    pub fn snap_to_player(&mut self, p: &Player) {
        self.x = p.x;
        self.y = p.y;
        self.z = p.z;
    }

    pub fn horizontal_angle(&self) -> f32 {
        self.horizontal_angle
    }

    pub fn vertical_angle(&self) -> f32 {
        self.vertical_angle
    }

    pub fn change_horizontal_angle(&mut self, dh: GLfloat) {
        self.horizontal_angle += dh;
    }

    pub fn change_vertical_angle(&mut self, dv: GLfloat) {
        self.vertical_angle += dv;
        if self.vertical_angle < PI_3_2 { self.vertical_angle = PI_3_2 }
        else if self.vertical_angle > PI_5_2 { self.vertical_angle = PI_5_2 }
    }
}

pub fn new(fovy: GLfloat, aspect: GLfloat, z_near: GLfloat, z_far: GLfloat) -> Camera {
    let mut ret = Camera{ x: 0.0f32, y: 0.0f32, z: 0.0f32,
                          horizontal_angle: 0.0f32, vertical_angle: 2.0*PI,
                          projection: matrix::new(), view_projection: matrix::new() };
    ret.projection.set_perspective_matrix(fovy, aspect, z_near, z_far);
    ret.update_view_projection();
    ret
}

