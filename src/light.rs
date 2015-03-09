use object;
use shader;
use gl::types::*;

pub fn new_light(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, shader: shader::Shader) -> object::Object {
    let verts: [GLfloat; 6*3] = [
        // Front face
        -1.0f32, -1.0f32, 0.0f32,
         1.0f32,  1.0f32, 0.0f32,
        -1.0f32,  1.0f32, 0.0f32,
        -1.0f32, -1.0f32, 0.0f32,
         1.0f32, -1.0f32, 0.0f32,
         1.0f32,  1.0f32, 0.0f32,
            ];
    let norms: [GLfloat; 6*3] = [
        // Front face
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
        0.0f32, 0.0f32, 1.0f32,
            ];
    let mut indxs: [u32; 6] = [0; 6];
    for i in 0..(6) { indxs[i] = i as u32; }
    //let mut obj = object::generate2(&verts, &norms, &indxs, r, g, b, shader, true);

    //let mut obj = object::new(1.0, 1.0, 1.0, 1.1, 1.1, 1.1, r, g, b, shader, true);
    //let mut obj = object::new(-1.0, -1.0, -1.0, -1.1, -1.1, -1.1, r, g, b, shader, true);
    //let mut obj = object::new(-1.0, -1.0, -1.0, 1.1, 1.1, 1.1, r, g, b, shader, true);
    //let mut obj = object::new(-1.0, -1.0, -1.0, 1.0, 1.0, 1.0, r, g, b, shader, true);
    let scale = 10.0;
    let mut obj = object::new(scale*-1.0, scale*-1.0, scale*-1.0, scale*1.0, scale*1.0, scale*1.0, r, g, b, shader, true);
    obj.set_translation(x,y,z);
    obj
}
