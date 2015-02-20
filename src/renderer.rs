use gbuffer;


pub struct Renderer {
    gbuff: gbuffer::GBuffer,
}

impl Renderer {
    pub fn start_geometry_pass(&self) {
        gbuff.bind_for_writing();
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }
    pub fn draw(&self, camera:&camera::Camera, object: &object::Object) -> () {
        object.draw(camera);
    }
    pub fn start_light_pass(&self, winx: u32, winy: u32) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        gbuff.bind_for_reading();
        let half_width = winx/2.0f32;
        let half_height = winy/2.0f32;
    }
}

fn new() -> Renderer { Renderer { gbuff: gbuffer::new() } }


