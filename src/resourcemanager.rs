//use assimp as ai;

use object;
use std::vec;
use std::collections::HashMap;
use solids;
use shader;
use matrix;
use light;

pub struct ResourceManager {
    //importer : ai::Importer,
    modelMap : HashMap<String, model>,
    vertsMap : HashMap<String, modelData>,
    shaderSet : HashMap<String, u32>
}

impl<'resman> ResourceManager {
    
    pub fn init(&self) -> () {
        println!("ResourceManager is initialized!");
    }
    
    //Call this to get back the models vertices
    //pub fn get_model<'a, 'b>(&'a mut self, filename : &'b str) -> (Vec<f32>, Vec<f32>) {
        //if(self.modelMap.contains_key(filename)) {
            //println!("Model from {} is already loaded!", filename);
            //return match self.vertsMap.get(filename) {
                //Some(data) => {
                    //println!("Model data obtained");
                    //(data.v.clone(), data.n.clone())
                //},
                //None => {
                    //println!("No model data! error!");
                    //(Vec::new(), Vec::new())
                //}
            //}
        //} else {
            //println!("Loading unique model from {}!", filename);
            //let mut scene = self.importer.import_from_file(filename).unwrap();
            //let mut meshes = scene.get_meshes();
            //let mesh = meshes[0];
        
            //let mut verts : Vec<f32> = Vec::new();
            //for vert in mesh.get_vertices() {
                //verts.push_all(&[vert.x, vert.y, vert.z]);
                ////println!("Vertex is: {:?}", vert);
            //}
        
            //let mut norms : Vec<f32> = Vec::new();
            //for norm in mesh.get_normals() {
                //norms.push_all(&[norm.x, norm.y, norm.z]);
            //}
            //self.vertsMap.insert(String::from_str(filename), modelData { v: verts.clone(), n: norms.clone() });
            //self.modelMap.insert(String::from_str(filename), model { vbo: 0, vno: 0, vio: 0, vao: 0, file: String::from_str(filename)});
            //return (verts, norms);
        //}   

    //}
    
    pub fn new_floor(&mut self, x:f32, y:f32, z:f32) -> solids::Floor {
        //println!("floor shader!");
        let shader = self.get_shader("object");
        let m = object::new2(-2f32, -0.2f32, 2f32,  2f32, 0f32, -2f32,  0.6, 0.6, 0.8, shader);
        let mask = solids::new_mask(4.0, 4.0);
        return solids::new_floor2(x, y, z, mask, m);
    }


    pub fn new_ceiling(&mut self, x:f32, y:f32, z:f32) -> solids::Floor {
        //println!("ceiling shader!");
        let shader = self.get_shader("object");
        let m = object::new2(-2f32, 0f32, 2f32,  2f32, 0.2f32, -2f32,  0.8, 0.6, 0.6, shader);
        let mask = solids::new_mask(4.0, 4.0);
        return solids::new_ceiling2(x,y,z,mask,m);
    }

    pub fn new_door(&mut self, x:f32, y:f32, z:f32, rot:f32) -> solids::Door{
        //println!("door shader!");
        let shader = self.get_shader("object");
        let width_2: f32;
        let length_2: f32;

        if rot == 1f32 {
            width_2 = 1.5f32;
            length_2 = 0.05f32;
        } else {
            width_2 = 0.05f32;
            length_2 = 1.5f32;
        }

        let height = 4f32;

        let m = object::new2(-width_2, 0f32, length_2, width_2, height, -length_2, 0.3, 0.5, 0.8, shader);
        let mask = solids::new_mask(width_2*2.0, length_2*2.0);
        return solids::new_door2(x,y,z,rot,mask,m, width_2, length_2);
    }
    
    pub fn new_wall(&mut self, x: f32, y: f32, z: f32, rot: f32) -> solids::Wall {
        let shader = self.get_shader("object");
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

        let m = object::new2(-width_2, 0f32, length_2,  width_2, height, -length_2,  0.6, 0.7, 0.9, shader);
        let mask = solids::new_mask(width_2*2.0, length_2*2.0);
        return solids::new_wall2(x,y,z,width_2,length_2, mask, m);
    }

    pub fn new_short_wall(&mut self, x: f32, y: f32, z: f32, rot: f32) -> solids::Wall {
        let shader = self.get_shader("object");
        let width_2: f32;
        let length_2: f32;

        if rot == 1f32 || rot == 3f32 {
            width_2 = 0.4;
            length_2 = 0.1;
        } else {
            width_2 = 0.1f32;
            length_2 = 0.4;
        }

        let height = 4f32;

        let m = object::new2(-width_2, 0f32, length_2, width_2, height, -length_2, 0.6, 0.9, 0.9, shader);
        let mask = solids::new_mask(width_2*2.0, length_2*2.0);
        let (xoff, zoff) = match rot {
            1f32 => (-1.6, 0.0),
            2f32 => (0.0, -1.6),
            3f32 => (1.6, 0.0),
            4f32 => (0.0, 1.6),
            _ => (0.0, 0.0),
        };
        return solids::new_short_wall2(x,y,z,xoff,zoff,width_2,length_2,mask,m);
    }

    pub fn new_light(&mut self, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> object::Object{
        let shader = self.get_light_shader("light");
        return light::new_light2(x,y,z,r,g,b,shader);
    }
/*
    pub fn getFloor(&self, x:f32, y:f32, z:f32) -> solids::Floor {
        let m = self.getObj("floor");
        let mask = solids::new_mask(4.0,4.0);
        return solids::Floor{x: x, y: y, z: z, mask: mask, model: m};
    }

    pub fn getObj(&self, filename: &str) -> object::Object {
        if(self.modelMap.contains_key(filename)) {
            let s = match self.shaderSet.get("object") {
                Some(shader) => shader,
                None => &shader::Shader{program: 0}
            };
            return match self.modelMap.get(filename) {
                Some(model) => {
                    object::Object {
                        x: 0f32,
                        y: 0f32,
                        z: 0f32,

                        rx: 0f32,
                        ry: 0f32,
                        rz: 0f32,

                        r: 0.6,
                        g: 0.6,
                        b: 0.8,

                        model_matrix: matrix::new(),

                        num_indx: 36 as u32,

                        visible: true,

                        shader: shader::Shader{program: s.get_program()},
                        vao: model.vao,
                        vert_buff: model.vbo,
                        norm_buff: model.vno,
                        texc_buff: 0,
                        indx_buff: model.vio,
                        is_light: false,
                    }
                  
                },
                None => {
                    //Should never run!
                    object::new(-2f32, -0.2f32, 2f32,  2f32, 0f32, -2f32,  0.6, 0.6, 0.8)
                }
            }

        } else {
            let m = object::new(-2f32, -0.2f32, 2f32,  2f32, 0f32, -2f32,  0.6, 0.6, 0.8);
            let glInfo = model { vao: m.vao, vbo: m.vert_buff, vno: m.norm_buff, vio: m.indx_buff, file: String::from_str(filename)};
            self.modelMap.insert(String::from_str(filename), glInfo);
            return m;
        }
    }*/

    pub fn get_shader(&mut self, filename: &str) -> shader::Shader {
        if (self.shaderSet.contains_key(filename)) {
            return match self.shaderSet.get(filename) {
                Some(shader) => {//println!("Shader already made!");
                    shader::new2(*shader)},
                None => shader::new(VS_SRC, FS_SRC)
            };
        } else {
            let s = shader::new(VS_SRC, FS_SRC);
            //println!("Shader being made!!!!");
            self.shaderSet.insert(String::from_str(filename), s.get_program());
            return s;
        }
    }

    pub fn get_light_shader(&mut self, filename: &str) -> shader::Shader {
        if (self.shaderSet.contains_key(filename)) {
            return match self.shaderSet.get(filename) {
                Some(shader) => {//println!("Shader already made!");
                    shader::new2(*shader)},
                None => shader::new(VS_LIGHT_SRC, FS_LIGHT_SRC)
            };
        } else {
            let s = shader::new(VS_LIGHT_SRC, FS_LIGHT_SRC);
            //println!("Shader being made!!!!");
            self.shaderSet.insert(String::from_str(filename), s.get_program());
            return s;
        }
    }

}


pub fn new() -> ResourceManager {
    ResourceManager {
       //importer : ai::Importer::new(), modelMap : HashMap::new(), vertsMap : HashMap::new(), shaderSet : HashMap::new()
       modelMap : HashMap::new(), vertsMap : HashMap::new(), shaderSet : HashMap::new()
    }
}

pub struct model {
    //Vertex, normal, index
    vbo: u32,
    vno: u32,
    vio: u32,
    vao: u32,
    file: String
}

pub struct modelData {
    v : Vec<f32>,
    n : Vec<f32>
}


//Shader code

static VS_SRC: &'static str = "
#version 120
attribute vec3 vert_model;
attribute vec3 norm_model;

uniform mat4 modelMatrix;
uniform mat4 viewProjectionMatrix;

varying vec4 position_modelSpace;
varying vec4 normal_modelSpace;

void main() {
    gl_Position = viewProjectionMatrix * modelMatrix * vec4(vert_model, 1);
    //gl_Position = viewProjectionMatrix * vec4(vert_model, 1);
    //gl_Position = vec4(vert_model, 1);
    position_modelSpace = modelMatrix * vec4(vert_model, 1);
    //normal_modelSpace = normalize(modelMatrix * vec4(norm_model, 1));
    normal_modelSpace = vec4(norm_model, 1);
}
    ";

static FS_SRC: &'static str = "

#version 120

uniform vec3 material_color;
uniform float alpha;

varying vec4 position_modelSpace;
varying vec4 normal_modelSpace;

void main() {
    gl_FragData[0] = vec4(material_color, 1);

    //gl_FragData[1] = position_modelSpace;
    //gl_FragData[2] = normal_modelSpace;

    gl_FragData[1] = position_modelSpace;
    gl_FragData[2] = normal_modelSpace;

    gl_FragData[3] = vec4(1,1,0,1);
    //gl_FragData[3] = (position_modelSpace/20 + normal_modelSpace) * vec4(material_color, 1);
}
    ";

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
