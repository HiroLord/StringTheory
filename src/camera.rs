use matrix;
use gl::types::*;

pub struct Camera {
    projection: matrix::Matrix,
    pub view: matrix::Matrix,
    pub view_projection: matrix::Matrix,
}

impl Camera {
    pub fn update_view_projection(&mut self) {
        let mul = self.projection * self.view;
        for i in 0..16 { self.view_projection.data[i] = mul.data[i]; }
    }
}

pub fn new(fovy: GLfloat, aspect: GLfloat, z_near: GLfloat, z_far: GLfloat) -> Camera {
    let mut ret = Camera{ projection: matrix::new(), view: matrix::new(), view_projection: matrix::new() };
    ret.projection.setPerspectiveMatrix(fovy, aspect, z_near, z_far);
    ret.update_view_projection();
    ret
}

