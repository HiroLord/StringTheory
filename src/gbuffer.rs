
use std::ptr;
use gl;
use gl::types::*;

pub enum TextureType {
    Position = 0,
    Diffuse = 1,
    Normal = 2,
    Texcoord = 3,
    Number = 4,
}
impl Copy for GBuffer {}
pub struct GBuffer {
    fbo: GLuint,
    textures: [GLuint; TextureType::Number as usize],
    depth_texture: GLuint,
}

impl GBuffer {
    pub fn init(&mut self, window_width: u32, window_height: u32) {
        unsafe {
            // Create FBO
            gl::GenFramebuffers(1, &mut self.fbo); 
            println!("Framebuffer {}", self.fbo);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.fbo);
            //gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo);

            // Create textures
            gl::GenTextures(self.textures.len() as i32, (&mut self.textures[0]) as *mut u32);
            gl::GenTextures(1, &mut self.depth_texture);
            for i in 0..self.textures.len() {
                println!("Texture {}", self.textures[i]);
            }
            println!("Texture {}", self.depth_texture);

            for i in 0..(TextureType::Number as usize) {
                gl::BindTexture(gl::TEXTURE_2D, self.textures[i]);
                //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB32F as i32, window_width as i32, window_height as i32, 0, gl::RGB, gl::FLOAT, ptr::null());
                //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, window_width as i32, window_height as i32, 0, gl::RGB, gl::BYTE, ptr::null());
                //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, window_width as i32, window_height as i32, 0, gl::RGB, gl::SHORT, ptr::null());
                //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, window_width as i32, window_height as i32, 0, gl::RGB, gl::INT, ptr::null());
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, window_width as i32, window_height as i32, 0, gl::RGB, gl::FLOAT, ptr::null());
                gl::FramebufferTexture2D(gl::DRAW_FRAMEBUFFER, (gl::COLOR_ATTACHMENT0 + i as u32) as u32, gl::TEXTURE_2D, self.textures[i], 0);
            }
            
            gl::BindTexture(gl::TEXTURE_2D, self.depth_texture);
            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT32F as i32, window_width as i32, window_height as i32, 0, gl::DEPTH_COMPONENT, gl::FLOAT, ptr::null());
            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32, window_width as i32, window_height as i32, 0, gl::DEPTH_COMPONENT, gl::BYTE, ptr::null());
            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32, window_width as i32, window_height as i32, 0, gl::DEPTH_COMPONENT, gl::SHORT, ptr::null());
            //gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32, window_width as i32, window_height as i32, 0, gl::DEPTH_COMPONENT, gl::INT, ptr::null());
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::DEPTH_COMPONENT as i32, window_width as i32, window_height as i32, 0, gl::DEPTH_COMPONENT, gl::FLOAT, ptr::null());
            gl::FramebufferTexture2D(gl::DRAW_FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::TEXTURE_2D, self.depth_texture, 0);

            let draw_buffers = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1, gl::COLOR_ATTACHMENT2, gl::COLOR_ATTACHMENT3];
            gl::DrawBuffers(draw_buffers.len() as i32, (&draw_buffers[0]) as *const u32);

                println!("glGetError: {}", gl::GetError());
            let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if status != gl::FRAMEBUFFER_COMPLETE {
                println!("Framebuffer creation failed!");
                println!("glGetError: {}", gl::GetError());
                println!("status: {}", status);
                if status == gl::FRAMEBUFFER_UNSUPPORTED {
                    println!("Framebuffer Unsupported");
                }
            } else {
                println!("Framebuffer creation success!");
            }

            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0);
        }
    }
    pub fn bind_for_writing(&self) { unsafe { gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.fbo); } }
    pub fn bind_for_reading(&self) { unsafe { gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.fbo); } }
    pub fn set_read_buffer(&self, tex_type: TextureType) { unsafe { gl::ReadBuffer((gl::COLOR_ATTACHMENT0 + tex_type as u32) as u32); } }
}

pub fn new() -> GBuffer { GBuffer {fbo: 0, textures: [0; TextureType::Number as usize], depth_texture: 0 } }


