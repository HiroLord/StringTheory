extern crate std;

use solids;
use solids::new_floor;
use solids::new_wall;
use std::old_io::File;

pub struct Map {
    floors: Vec<solids::Floor>,
    walls: Vec<solids::Wall>,
}

impl Map {
    pub fn get_floors(&self) -> &Vec<solids::Floor> {
        &self.floors
    }

    pub fn get_walls(&self) -> &Vec<solids::Wall> {
        &self.walls
    }
}

pub fn load_map() -> Map {
    let mut floors = Vec::new();
    let mut walls = vec![];

    let mut file = File::open_mode(&Path::new("savedmap.map"),
                                std::old_io::FileMode::Open,
                                std::old_io::FileAccess::Read);
    
    let size = match file.read_be_i32() {
        Ok(n) => n,
        Err(e) => panic!("{}", e),
    };

    for i in range(0, size) {
        let blocktype = match file.read_be_u32() {
            Ok(n) => n,
            Err(e) => panic!("{}", e),
        } as u32;

        let bx = match file.read_be_f32() {
            Ok(n) => n,
            Err(e) => panic!("{}", e),
        } as f32;

        let by = match file.read_be_f32() {
            Ok(n) => n,
            Err(e) => panic!("{}", e),
        } as f32;
        match blocktype {
            1 => floors.push( new_floor(bx * 4.0, 0.0, by * 4.0) ),
            2 => walls.push( new_wall(bx * 4.0 - 2.0, 0.0, by * 4.0, 2.0) ),
            3 => walls.push( new_wall(bx * 4.0, 0.0, by * 4.0 - 2.0, 1.0) ),
            _ => (),
        }
    }

    Map{floors: floors, walls: walls}
}

pub fn new_map(size: u32) -> Map {
    load_map()
    /*
    let mut floors = vec![ new_floor(0.0, 0.0, 0.0), 
                    new_floor(4.0, 0.0, 0.0), new_floor(8.0, 0.0, 0.0),
                    new_floor(8.0, 0.0, 4.0), new_floor(12.0, 0.0, 0.0),
                    new_floor(12.0, 0.0, 4.0), new_floor(8.0, 0.0, 8.0),
                        new_floor(0.0, 4.0, 0.0), 
                    new_floor(4.0, 4.0, 0.0), new_floor(8.0, 4.0, 0.0),
                    new_floor(8.0, 4.0, 4.0), new_floor(12.0, 4.0, 0.0),
                    new_floor(12.0, 4.0, 4.0), new_floor(8.0, 4.0, 8.0),];
    let mut walls = vec![ new_wall(0.0, 0.0, 2.0, 1.0), new_wall(-2.0, 0.0, 0.0, 2.0),
                          new_wall(4.0, 0.0, 2.0, 1.0), new_wall(0.0, 0.0, -2.0, 1.0),
                          new_wall(4.0, 0.0, -2.0, 1.0), new_wall(8.0, 0.0, -2.0, 1.0),
                          new_wall(6.0, 0.0, 4.0, 2.0), new_wall(12.0, 0.0, -2.0, 1.0),
                          new_wall(14.0, 0.0, 0.0, 2.0), new_wall(14.0, 0.0, 4.0, 2.0),
                          new_wall(12.0, 0.0, 6.0, 1.0), ];
    let mut map = Map{floors: floors, walls: walls};
    map
    */
}
