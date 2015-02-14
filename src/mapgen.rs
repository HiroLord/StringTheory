use solids;
use solids::new_floor;

pub struct Map {
    floors: Vec<solids::Floor>,
}

impl Map {
    pub fn get_floors(&self) -> &Vec<solids::Floor> {
        &self.floors
    }
}

pub fn new_map(size: u32) -> Map {
    let mut map = Map{floors: vec![ new_floor(0.0, 0.0, 0.0), new_floor(0.0, 4.0, 0.0),
                    new_floor(2.0, 0.0, 0.0), new_floor(4.0, 0.0, 0.0),
                    new_floor(4.0, 0.0, 2.0)] };
    map
}
