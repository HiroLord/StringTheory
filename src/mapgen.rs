use std::num::Float;
use gl::types::*;

use object;
use solids;
use solids::new_floor;

pub fn new_map(size: u32) -> Vec<solids::Floor> {
    let map = vec![ new_floor(0.0, 0.0, 0.0), new_floor(0.0, 4.0, 0.0),
                    new_floor(2.0, 0.0, 0.0), new_floor(4.0, 0.0, 0.0),
                    new_floor(4.0, 0.0, 2.0)];
    map
}
