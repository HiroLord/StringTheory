use gl::types::*;
use std::ops::Add;
use std::ops::Mul;
use std::num::Float;
use std::f32::consts;
use std::iter;

pub struct Matrix {
    pub data: [GLfloat; 16],
}
impl Copy for Matrix {}
impl Matrix {
    pub fn setIdentity(&mut self) -> () {
        for i in 0..16 { self.data[i] = 0.0f32; }
        self.data[0] = 1.0f32;
        self.data[5] = 1.0f32;
        self.data[10] = 1.0f32;
        self.data[15] = 1.0f32;
    }
    pub fn setTranslation(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) -> () {
        self.data[12] = x;
        self.data[13] = y;
        self.data[14] = z;
    }
    pub fn setScale(&mut self, x: GLfloat, y: GLfloat, z: GLfloat) -> () {
        self.data[0] = x;
        self.data[5] = y;
        self.data[10] = z;
    }
    pub fn setPerspectiveMatrix(&mut self, fovy: GLfloat, aspect: GLfloat, z_near: GLfloat, z_far: GLfloat) {
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
            for j in iter::range_step(0,16,4) {
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
    ret.setIdentity();
    ret
}

fn deg_to_rad(deg: GLfloat) -> GLfloat { (deg % 360.0f32) / 180.0f32 * consts::PI }



