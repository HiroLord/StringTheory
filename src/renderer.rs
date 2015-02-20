use gl;
use gl::types::*;

use gbuffer;
use camera;
use object;


pub struct Renderer {
    winx: u32,
    winy: u32,
    gbuff: gbuffer::GBuffer,
}

impl Renderer {
    pub fn start_geometry_pass(&self) {
        self.gbuff.bind_for_writing();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn draw(&self, camera:&camera::Camera, object: &object::Object) -> () {
        //object.draw(camera);
    }
    pub fn start_light_pass(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.gbuff.bind_for_reading();
        let width = self.winx as i32;
        let height = self.winy as i32;
        let half_width = (width/2) as i32;
        let half_height = (height/2) as i32;
        self.gbuff.set_read_buffer(gbuffer::TextureType::Position);
        unsafe {
            gl::BlitFramebuffer(0, 0, width, height,
                                0, 0, half_width, half_height, gl::COLOR_BUFFER_BIT, gl::LINEAR);

            self.gbuff.set_read_buffer(gbuffer::TextureType::Diffuse);
            gl::BlitFramebuffer(0, 0, width, height,
                                0, half_height, half_width, height, gl::COLOR_BUFFER_BIT, gl::LINEAR);

            self.gbuff.set_read_buffer(gbuffer::TextureType::Normal);
            gl::BlitFramebuffer(0, 0, width, height,
                                half_width, half_height, width, height, gl::COLOR_BUFFER_BIT, gl::LINEAR);

            self.gbuff.set_read_buffer(gbuffer::TextureType::Texcoord);
            gl::BlitFramebuffer(0, 0, width, height,
                                half_width, 0, width, half_height, gl::COLOR_BUFFER_BIT, gl::LINEAR);
        }
    }
}

pub fn new(winx: u32, winy: u32) -> Renderer { 
    let mut gbuff = gbuffer::new();
    gbuff.init(winx, winy);
    Renderer { winx: winx, winy: winy, gbuff: gbuff }
}


