use assimp as ai;

use object;
use std::vec;

pub struct ResourceManager<'resman> {
    importer : ai::Importer,
    obj_list : Vec<object::Object>
}

impl<'resman> ResourceManager<'resman> {
    
    pub fn init(&self) -> () {
        println!("ResourceManager is initialized!");
    }
    
    //Call this to get back the models vertices
    pub fn get_model<'a, 'b>(&'a mut self, filename : &'b str) -> (Vec<f32>, Vec<f32>) {
        let mut scene = self.importer.import_from_file(filename).unwrap();
        let mut meshes = scene.get_meshes();
        let mesh = meshes[0];
        
        let mut verts : Vec<f32> = Vec::new();
        for vert in mesh.get_vertices() {
            verts.push_all(&[vert.x, vert.y, vert.z]);
            //println!("Vertex is: {:?}", vert);
        }
        
        let mut norms : Vec<f32> = Vec::new();
        for norm in mesh.get_normals() {
            norms.push_all(&[norm.x, norm.y, norm.z]);
        }
        return (verts, norms);

    }
}


pub fn new<'resman>() -> ResourceManager<'resman> {
    ResourceManager {
       importer : ai::Importer::new(), obj_list : Vec::new()
    }
}

