extern crate std;

use solids;
use object;
use resourcemanager as res;
use std::old_io::File;

pub struct Map {
    floors: Vec<solids::Floor>,
    walls: Vec<solids::Wall>,
    lights: Vec<object::Object>,
    doors: Vec<solids::Door>,
    spawns: Vec<Point>,
}

pub struct Point {
    pub a: f32,
    pub b: f32,
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

    pub fn get_spawns(&self) -> &Vec<Point> {
        &self.spawns
    }

    pub fn get_spawn(&self, s: u32) -> &Point {
        &self.spawns[s as usize]
    }

    pub fn open_door(&mut self, door: i32) {
        self.doors[door as usize].open();
    }

    pub fn close_door(&mut self, door: i32) {
        self.doors[door as usize].close();
    }
}

pub fn load_map(resman : &mut res::ResourceManager) -> Map {
    let mut floors = Vec::new();
    let mut walls = Vec::new();
    let mut lights = Vec::new();
    let mut doors = Vec::new();
    let mut spawns = Vec::new();
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
                floors.push( resman.new_floor(bx * 4.0, 0.0, by * 4.0) );
                floors.push( resman.new_ceiling(bx * 4.0, 4.0, by * 4.0) );
            },
            2 => walls.push( resman.new_wall(bx * 4.0 - 2.0, 0.0, by * 4.0, 2.0) ),
            3 => walls.push( resman.new_wall(bx * 4.0, 0.0, by * 4.0 - 2.0, 1.0) ),
            4 => lights.push( resman.new_light(bx * 4.0 - 2.0, 3.0, by * 4.0 - 2.0, 4.0, 4.0, 4.0) ),
            5 => {
                walls.push(resman.new_short_wall(bx * 4.0 - 2.0, 0.0, by * 4.0, 2.0) );
                walls.push(resman.new_short_wall(bx * 4.0 - 2.0, 0.0, by * 4.0, 4.0) );
                doors.push(resman.new_door(bx * 4.0 - 2.0, 0.0, by * 4.0, 2.0) );
            },
            6 => {
                walls.push(resman.new_short_wall(bx * 4.0, 0.0, by * 4.0 - 2.0, 1.0) );
                walls.push(resman.new_short_wall(bx * 4.0, 0.0, by * 4.0 - 2.0, 3.0) );
                doors.push(resman.new_door(bx * 4.0, 0.0, by * 4.0 - 2.0, 1.0) );
            },
            10 => {
                spawns.push(Point{a: bx*4.0, b: by*4.0} );
            },
            _ => (),
        }
    }

    Map{floors: floors, walls: walls, lights: lights, doors: doors, spawns: spawns}
}

pub fn new_map(size: u32, resman:&mut res::ResourceManager) -> Map {
    load_map(resman)
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
