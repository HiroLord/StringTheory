use gl::types::*;
use std::ops::Add;
use std::ops::Mul;
use std::num::Float;
use std::f32::consts;
use std::iter;

#[derive(Clone)]
pub struct Matrix {
    pub data: [GLfloat; 16],
}
impl Copy for Matrix {}
impl Matrix {
    pub fn set_identity(&mut self) -> () {
        for i in 0..16 { self.data[i] = 0.0f32; }
        self.data[0] = 1.0f32;
        self.data[5] = 1.0f32;
        self.data[10] = 1.0f32;
        self.data[15] = 1.0f32;
    }
    pub fn set_translation(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) -> () {
        self.data[12] = x;
        self.data[13] = y;
        self.data[14] = z;
    }
    pub fn set_scale(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) -> () {
        self.data[0] = x;
        self.data[5] = y;
        self.data[10] = z;
    }
    
    pub fn rotate(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) {
        let res = rotated(x,y,z);
        for i in 0..16 { self.data[i] = res.data[i]; }
    }

    pub fn set_perspective_matrix(&mut self, fovy: GLfloat, aspect: GLfloat, z_near: GLfloat, z_far: GLfloat) {
        //let f = 1.0f32/tan(deg_to_rad(fovy/2.0f32));
        let f = 1.0f32/deg_to_rad(fovy/2.0f32).tan();
        self.data[0] = f/aspect;
        self.data[5] = f;
        self.data[10] = (z_far+z_near)/(z_near-z_far);
        self.data[11] = -1.0f32;
        self.data[14] = (2.0f32*z_far*z_near)/(z_near-z_far);
    }
}

impl Add for Matrix {
    type Output = Matrix;
    fn add(self, other: Matrix) -> Matrix {
        let mut ret = Matrix { data: [0.0f32; 16] };
        for i in 0..16 {
                ret.data[i] = self.data[i] + other.data[i];
        }
        ret
    }
}
impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, other: Matrix) -> Matrix {
        let mut ret = Matrix { data: [0.0f32; 16] };
        for i in 0..4 {
            for j in iter::range_step_inclusive(0,15,4) {
                ret.data[i+j] = self.data[i] * other.data[j] +
                                self.data[i+4] * other.data[j+1] +
                                self.data[i+8] * other.data[j+2] +
                                self.data[i+12] * other.data[j+3];
            }
        }
        ret
    }
}


pub fn new() -> Matrix {
    let mut ret = Matrix { data: [0.0f32; 16] };
    ret.set_identity();
    ret
}

pub fn translated(x: GLfloat, y:GLfloat, z:GLfloat) -> Matrix {
    let mut ret = new();
    ret.set_translation(x,y,z);
    ret
}
pub fn rotated(x: GLfloat, y:GLfloat, z:GLfloat) -> Matrix {
    let mut rot_x = new();
    let mut rot_y = new();
    let mut rot_z = new();

    // The X rotation matrix
    rot_x.data[0] = 1.0f32;
    rot_x.data[5] = x.cos();
    rot_x.data[6] = x.sin();
    rot_x.data[9] = -x.sin();
    rot_x.data[10] = x.cos();
    rot_x.data[15] = 1.0f32;

    rot_y.data[0] = y.cos();
    rot_y.data[2] = -y.sin();
    rot_y.data[5] = 1.0f32;
    rot_y.data[8] = y.sin();
    rot_y.data[10] = y.cos();
    rot_y.data[15] = 1.0f32;

    rot_z.data[0] = z.cos();
    rot_z.data[1] = z.sin();
    rot_z.data[4] = -z.sin();
    rot_z.data[5] = z.cos();
    rot_z.data[10] = 1.0f32;
    rot_z.data[15] = 1.0f32;

    rot_x * rot_y * rot_z
}

fn deg_to_rad(deg: GLfloat) -> GLfloat { (deg % 360.0f32) / 180.0f32 * consts::PI }



