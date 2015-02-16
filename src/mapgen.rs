use solids;
use solids::new_floor;
use solids::new_wall;

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

pub fn new_map(size: u32) -> Map {
    
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
}
