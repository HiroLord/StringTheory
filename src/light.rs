use object;
use shader;
use gl::types::*;

pub fn new_light(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> object::Object {
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
    let mut obj = object::generate_general(&verts, &norms, &indxs, r, g, b, VS_LIGHT_SRC, FS_LIGHT_SRC, true);
    obj.set_translation(x,y,z);
    obj
}

pub fn new_light2(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, shader: shader::Shader) -> object::Object {
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
    let mut obj = object::generate2(&verts, &norms, &indxs, r, g, b, shader, true);
    obj.set_translation(x,y,z);
    obj
}
static VS_LIGHT_SRC: &'static str = "
#version 120
attribute vec3 vert_model;
attribute vec3 norm_model;

uniform mat4 modelMatrix;
uniform mat4 viewProjectionMatrix;

void main() {
    gl_Position = vec4(vert_model, 1);
}
    ";

static FS_LIGHT_SRC: &'static str = "

#version 120

uniform vec3 light_world_pos;
uniform vec2 window_size;
uniform vec3 material_color;
uniform float alpha;

uniform sampler2D diffuse_tex;
uniform sampler2D position_tex;
uniform sampler2D normal_tex;
uniform sampler2D last_tex;

varying vec4 position_modelSpace;

void main() {
    vec2 tex_coord = gl_FragCoord.xy / window_size;
    //vec2 tex_coord = gl_FragCoord.xy / vec2(1280,720);

    vec4 pos = texture2D(position_tex, tex_coord);
    vec4 normal = texture2D(normal_tex, tex_coord);
    vec3 color = texture2D(diffuse_tex, tex_coord).xyz;
    //vec3 last = texture2D(last_tex, tex_coord).xyz;

    //normal = normalize(normal);

    //vec4 light_pos = vec4(3, 1, 0, 1);
    vec4 light_pos = vec4(light_world_pos, 1);

    // I don't think I should have to negate this....
    //vec3 vecToLight = normalize(pos - light_pos);
    //vec3 vecToLight = -normalize(pos - light_pos);
    vec4 vecToLight = -normalize(pos - light_pos);
    float cosTheta = clamp( dot(normal, vecToLight), 0, 1);
    //float cosTheta = clamp( dot(normal, vecToLight), 0, 1) + clamp( -dot(normal, vecToLight), 0, 1);
    //gl_FragColor = vec4(cosTheta, cosTheta, cosTheta, 1);
    float dist = distance(pos, light_pos); 
    gl_FragColor = vec4((cosTheta * color * material_color) / (dist), 1);
    //gl_FragColor = vec4(material_color, 1);
    //gl_FragColor = vec4(color, 1);
    //gl_FragColor = vec4(normal, 1);
    //gl_FragColor = vec4(normal/2 + 0.5);
    //gl_FragColor = vec4(pos, 1);
    //gl_FragColor = pos/2 + 0.5;
    //gl_FragColor = vec4(last, 1);
    //gl_FragColor = vec4(tex_coord,1, 1);
    //gl_FragColor = vec4(texture2D(diffuse_tex, vec2(0.5,0.5)).xyz, 1);
    //gl_FragColor = vec4(texture2D(diffuse_tex, tex_coord).xyz, 1);
    //gl_FragColor = vec4(texture2D(diffuse_tex, gl_FragCoord.xy).xyz, 1);
    //gl_FragColor = vec4(1,0,0,1);
    //gl_FragColor = vec4(final_color + material_color * vec3(0.3,0.3,0.3), 0);
}
    ";
