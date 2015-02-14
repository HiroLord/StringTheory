use gl::types::*;

use object;
use camera::Camera;

pub trait GameObject {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn draw(&self, &Camera);
}

pub trait SolidObject {
}

pub struct Floor {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,

    model: object::Object,
}

impl Floor{
    pub fn set_position(&mut self) {
        self.model.translate(self.x, self.y, self.z);
    }
}

impl GameObject for Floor {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }

    fn draw(&self, c: &Camera) {
        self.model.draw(c);
    }
}


pub struct Wall {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,

    model: object::Object,
}

impl Wall {
    pub fn set_position(&mut self) {
        self.model.translate(self.x, self.y, self.z);
    }
}

impl GameObject for Wall {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }

    fn draw(&self, c: &Camera) {
        self.model.draw(c);
    }
}


pub struct MedBay {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
}

pub fn new_floor(x: f32, y: f32, z: f32) -> Floor {

    let m = object::new(-1.0, -0.1, 1.0,  1.0, 0.1, -1.0,  0.6, 0.6, 0.8);

    let mut f = Floor{x: x, y: y, z:z, model: m};
    f.set_position();
    f
}


pub fn new_wall(x: f32, y: f32, z: f32, rot: i32) -> Wall {
    let m: object::Object;
    
    if rot == 1 {
        m = object::new(-1.0, 0.0, 0.1,  1.0, 3.0, -0.1,  0.8, 0.6, 0.6);
    } else {
        m = object::new(-0.1, 0.0, 1.0,  0.1, 3.0, -1.0, 0.7, 0.5, 0.5);
    }
    let mut w = Wall{x: x, y: y, z: z, model: m};
    w.set_position();
    w
}






