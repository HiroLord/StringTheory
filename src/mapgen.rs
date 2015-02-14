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

    let mut floors = vec![ new_floor(0.0, 0.0, 0.0), new_floor(0.0, 3.0, 0.0),
                    new_floor(2.0, 0.0, 0.0), new_floor(4.0, 0.0, 0.0),
                    new_floor(4.0, 0.0, 2.0), new_floor(6.0, 0.0, 0.0),
                    new_floor(6.0, 0.0, 2.0), new_floor(4.0, 0.0, 4.0),
                    new_floor(6.0, 0.0, 4.0), new_floor(8.0, 0.0, 0.0),
                    new_floor(8.0, 0.0, 2.0), new_floor(8.0, 0.0, 4.0)];

    let mut walls = vec![ new_wall(0.0, 0.0, 1.0, 1), new_wall(-1.0, 0.0, 0.0, 2) ];

    let mut map = Map{floors: floors, walls: walls};
    map
}
