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
    fn get_left(&self) -> f32;
    fn get_right(&self) -> f32;
    fn get_front(&self) -> f32;
    fn get_back(&self) -> f32;
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
    width: GLfloat,
    length: GLfloat,

    model: object::Object,
}

impl Wall {
    pub fn set_position(&mut self) {
        self.model.translate(self.x, self.y, self.z);
    }
   
    pub fn width(&self) -> f32 { self.width }

}

impl GameObject for Wall {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }

    fn draw(&self, c: &Camera) {
        self.model.draw(c);
    }
}

impl SolidObject for Wall {
    fn get_left(&self) -> f32 { self.x - self.width/2.0 }
    fn get_right(&self) -> f32 { self.x + self.width/2.0 }
    fn get_front(&self) -> f32 { self.z + self.length/2.0 }
    fn get_back(&self) -> f32 { self.z - self.length/2.0 }
}

pub struct MedBay {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
}

pub fn new_floor(x: f32, y: f32, z: f32) -> Floor {

    let m = object::new(-2f32, -0.2f32, 2f32,  2f32, 0f32, -2f32,  0.6, 0.6, 0.8);

    let mut f = Floor{x: x, y: y, z:z, model: m};
    f.set_position();
    f
}


pub fn new_wall(x: f32, y: f32, z: f32, rot: f32) -> Wall {
    let width_2: f32;
    let length_2: f32;

    if rot == 1f32 {
        width_2 = 2f32;
        length_2 = 0.1f32;
    } else {
        width_2 = 0.1f32;
        length_2 = 2f32;
    }

    let height = 4f32;

    let m = object::new(-width_2, 0f32, length_2,  width_2, height, -length_2,  0.6, 0.7, 0.9);
    
    let mut w = Wall{x: x, y: y, z: z, width: width_2*2f32, length: length_2*2f32, model: m};
    w.set_position();
    w
}






