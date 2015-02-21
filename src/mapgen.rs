extern crate std;

use solids;
use object;
use light;
use solids::new_ceiling;
use solids::new_floor;
use solids::new_door;
use solids::new_wall;
use solids::new_short_wall;
use light::new_light;
use std::old_io::File;

pub struct Map {
    floors: Vec<solids::Floor>,
    walls: Vec<solids::Wall>,
    lights: Vec<object::Object>,
    doors: Vec<solids::Door>,
}

impl Map {
    pub fn get_floors(&self) -> &Vec<solids::Floor> {
        &self.floors
    }

    pub fn get_walls(&self) -> &Vec<solids::Wall> {
        &self.walls
    }

    pub fn get_lights(&self) -> &Vec<object::Object> {
        &self.lights
    }

    pub fn get_doors(&self) -> &Vec<solids::Door> {
        &self.doors
    }

    pub fn open_door(&mut self, door: i32) {
        self.doors[door as usize].open();
    }

    pub fn close_door(&mut self, door: i32) {
        self.doors[door as usize].close();
    }
}

pub fn load_map() -> Map {
    let mut floors = Vec::new();
    let mut walls = Vec::new();
    let mut lights = Vec::new();
    let mut doors = Vec::new();
    let mut file = File::open_mode(&Path::new("savedmap.map"),
                                std::old_io::FileMode::Open,
                                std::old_io::FileAccess::Read);
    
    let size = match file.read_be_i32() {
        Ok(n) => n,
        Err(e) => panic!("{}", e),
    };

    for _ in range(0, size) {
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
            1 => {
                floors.push( new_floor(bx * 4.0, 0.0, by * 4.0) );
                floors.push( new_ceiling(bx * 4.0, 4.0, by * 4.0) );
            },
            2 => walls.push( new_wall(bx * 4.0 - 2.0, 0.0, by * 4.0, 2.0) ),
            3 => walls.push( new_wall(bx * 4.0, 0.0, by * 4.0 - 2.0, 1.0) ),
            4 => lights.push( new_light(bx * 4.0 - 2.0, 3.0, by * 4.0 - 2.0, 4.0, 4.0, 4.0) ),
            5 => {
                walls.push(new_short_wall(bx * 4.0 - 2.0, 0.0, by * 4.0, 2.0) );
                walls.push(new_short_wall(bx * 4.0 - 2.0, 0.0, by * 4.0, 4.0) );
                doors.push(new_door(bx * 4.0 - 2.0, 0.0, by * 4.0, 2.0) );
            },
            6 => {
                walls.push(new_short_wall(bx * 4.0, 0.0, by * 4.0 - 2.0, 1.0) );
                walls.push(new_short_wall(bx * 4.0, 0.0, by * 4.0 - 2.0, 3.0) );
                doors.push(new_door(bx * 4.0, 0.0, by * 4.0 - 2.0, 1.0) );
            },
            _ => (),
        }
    }

    let mut m = Map{floors: floors, walls: walls, lights: lights, doors: doors};
    m
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
