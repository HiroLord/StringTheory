use gl::types::*;
use renderer;

use object;
use camera::Camera;

pub trait GameObject {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;

    fn draw(&self, &Camera, &renderer::Renderer);
}

pub trait Solid {
    fn get_mask(&self) -> &Mask;
}

pub struct Mask {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    width: GLfloat,
    length: GLfloat,
}

impl Mask {
    pub fn x(&self) -> f32 { self.x }
    pub fn y(&self) -> f32 { self.y }
    pub fn z(&self) -> f32 { self.z }
    
    pub fn width(&self) -> f32 { self.width }
    pub fn length(&self) -> f32 { self.length }

    pub fn move_x(&mut self, dx: f32) { self.x += dx }
    pub fn move_z(&mut self, dz: f32) { self.z += dz }
    
    pub fn get_left(&self) -> f32 { self.x - self.width/2.0 }
    pub fn get_right(&self) -> f32 { self.x + self.width/2.0 }
    pub fn get_front(&self) -> f32 { self.z + self.length/2.0 }
    pub fn get_back(&self) -> f32 { self.z - self.length/2.0 }
    
    pub fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}

pub struct Floor {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,

    mask: Mask,
    model: object::Object,
}

impl Floor{
    pub fn set_position(&mut self) {
        self.model.set_translation(self.x, self.y, self.z);
        self.mask.set_pos(self.x, self.y, self.z);
    }

    pub fn bind_shader(&self) {
        self.model.bind_shader();
    }
}

impl GameObject for Floor {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }

    fn draw(&self, c: &Camera, renderer: &renderer::Renderer) {
        self.model.draw(c, renderer);
    }
}

pub struct Wall {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    width: GLfloat,
    length: GLfloat,

    mask: Mask,
    model: object::Object,
}

impl Wall {
    pub fn set_position(&mut self) {
        self.model.set_translation(self.x, self.y, self.z);
        self.mask.set_pos(self.x,self.y,self.z);
    }
   
    pub fn width(&self) -> f32 { self.width }
    
    pub fn bind_shader(&self){
        self.model.bind_shader();
    }
}

impl GameObject for Wall {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }

    fn draw(&self, c: &Camera, renderer: &renderer::Renderer) {
        self.model.draw(c, renderer);
    }
}

impl Solid for Wall {
    fn get_mask(&self) -> &Mask { &(self.mask) }
}


pub struct Door {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
    width: GLfloat,
    length: GLfloat,

    open: bool,

    mask: Mask,
    model: object::Object,
}

impl Door {
    pub fn set_position(&mut self) {
        self.model.set_translation(self.x, self.y, self.z);
        self.mask.set_pos(self.x, self.y, self.z);
    }

    pub fn is_open(&self) -> bool { self.open }

    pub fn open(&mut self) {
        self.open = true;
    }

    pub fn close(&mut self) {
        self.open = false;
    }

    pub fn bind_shader(&self) {
        self.model.bind_shader();
    }
}

impl GameObject for Door {
    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }

    fn draw(&self, c: &Camera, renderer: &renderer::Renderer) {
        if !self.open {
            self.model.draw(c, renderer);
        }
    }
}

impl Solid for Door {
    fn get_mask(&self) -> &Mask { &(self.mask) }
}

pub struct MedBay {
    x: GLfloat,
    y: GLfloat,
    z: GLfloat,
}

pub fn new_floor(x: f32, y: f32, z: f32, mask: Mask, m:object::Object) -> Floor {
    let mut f = Floor{x: x, y: y, z: z, mask: mask, model: m};
    f.set_position();
    f
}

pub fn new_ceiling(x: f32, y: f32, z: f32, mask: Mask, m: object::Object) -> Floor {
    let mut f = Floor{x: x, y: y, z: z, mask: mask, model: m};
    f.set_position();
    f
}

pub fn new_door(x: f32, y: f32, z: f32, rot: f32, mask: Mask, m: object::Object, width_2: f32, length_2: f32) -> Door {
    let mut d = Door{x: x, y: y, z: z, width: width_2*2f32, length: length_2*2f32, open: false, mask: mask,
                                                                                    model: m};
    d.set_position();
    d
}

pub fn new_wall(x: f32, y: f32, z: f32,  width_2: f32, length_2: f32, mask: Mask, m: object::Object) -> Wall {
    let mut w = Wall{x: x, y: y, z: z, width: width_2*2f32, length: length_2*2f32, mask: mask, model: m};
    w.set_position();
    w
}

pub fn new_short_wall(x: f32, y: f32, z: f32, xoff: f32, zoff: f32, width_2: f32, length_2: f32, mask: Mask, m: object::Object) -> Wall {
    let mut w = Wall{x: x+xoff, y: y, z: z+zoff, width: width_2*2.0, length: length_2*2f32, mask:
        mask,  model: m};
    w.set_position();
    w
}

pub fn new_mask(width: f32, length: f32) -> Mask{
    Mask{x: 0f32, y: 0f32, z: 0f32, width: width, length: length}
}


