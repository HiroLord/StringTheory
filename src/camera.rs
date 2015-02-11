use matrix;
use std::num::Float;
use gl::types::*;

pub struct Camera {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    pub horizontal_angle: GLfloat,
    pub vertical_angle: GLfloat,
    projection: matrix::Matrix,
    pub view_projection: matrix::Matrix,
}

impl Camera {
    pub fn update_view_projection(&mut self) {
        let mut view = matrix::rotated(-self.vertical_angle, -self.horizontal_angle, 0.0f32) * matrix::translated(-self.x,-self.y,-self.z);
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
}

pub fn new(fovy: GLfloat, aspect: GLfloat, z_near: GLfloat, z_far: GLfloat) -> Camera {
    let mut ret = Camera{ x: 0.0f32, y: 0.0f32, z: 0.0f32,
                          horizontal_angle: 0.0f32, vertical_angle: 0.0f32,
                          projection: matrix::new(), view_projection: matrix::new() };
    ret.projection.set_perspective_matrix(fovy, aspect, z_near, z_far);
    ret.update_view_projection();
    ret
}

