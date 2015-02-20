
use gl;
use gl::types::*;

enum TextureType {
    Position = 0,
    Diffuse = 1,
    Normal = 2,
    Texcoord = 3,
    Number = 4,
}
impl Copy for GBuffer {}
struct GBuffer {
    fbo: GLuint,
    textures: [GLuint; TextureType::Number as usize],
    depth_texture: GLuint,
}

impl GBuffer {
    pub fn init(&mut self, window_width: u32, window_height: u32) {
        unsafe {
            gl::GenFramebuffers(1, &mut self.fbo); 
        }
    }
    pub fn bind_for_writing() {}
    pub fn bind_for_reading() {}
}

pub fn new() -> GBuffer { GBuffer {fbo: 0, textures: [0; TextureType::Number as usize], depth_texture: 0 } }


