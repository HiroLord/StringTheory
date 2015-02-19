#![feature(io)]
#![feature(path)]
#![feature(core)]

extern crate sdl2;
//mod camera;

use std::old_io::File;
use sdl2::video::{Window, WindowPos};
use sdl2::event::{Event, poll_event};
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color::RGB;
use sdl2::rect::Rect;

struct Block { 
    x: i32,
    y: i32,
    t: u32,
    block_size: i32,
}

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    // 720p
    let window_width = 1280;
    let window_height = 720;

    let window = match Window::new("Corridors Map Maker", WindowPos::PosCentered,
                                   WindowPos::PosCentered, window_width, window_height,
                                   sdl2::video::MOUSE_FOCUS) {
        Ok(window) => window,
        Err(err) => panic!("Failed to create the window: {}", err)
    };

    let renderer = match sdl2::render::Renderer::from_window(window,
                                                             sdl2::render::RenderDriverIndex::Index(0),
                                                             sdl2::render::ACCELERATED) {
        Ok(renderer) => renderer,
        Err(err) => panic!("Failed to create renderer: {}", err)
    };
    
    let mut drawer = renderer.drawer();

    let mut rx = 0;
    let mut ry = 0;

    let block_size = 32;

    let mut blocks = Vec::new();

    let mut draw_block = Block{ x: 0, y: 0, t: 1, block_size: block_size }; 

    'main:loop {
        let mut polling = true;
        while polling {
            match poll_event() {
                Event::MouseMotion{x: mx, y: my, ..} => {
                    let (xoff, yoff) = match draw_block.t {
                        2 => (block_size/2, 0),
                        3 => (0, block_size/2),
                        4 => (block_size/2, block_size/2),
                        _ => (0, 0),
                    };
                    draw_block.x = (mx+xoff) - ((mx + xoff) % block_size);
                    draw_block.y = (my+yoff) - ((my + yoff) % block_size);
                }
                Event::MouseButtonUp{..} => {
                    blocks.push( Block{x: draw_block.x, y: draw_block.y, t: draw_block.t, block_size: block_size} );
                }
                Event::KeyDown{keycode: key, ..} => {
                    if key == KeyCode::Escape { break 'main; }
                    if key == KeyCode::S { save_map(&blocks); }
                    //if key == KeyCode::L { blocks = load_map(); }
                    if key == KeyCode::Num1 { draw_block.t = 1; }
                    if key == KeyCode::Num2 { draw_block.t = 2; }
                    if key == KeyCode::Num3 { draw_block.t = 3; }
                    if key == KeyCode::Num4 { draw_block.t = 4; }
                }
                Event::None => polling = false,
                _ => {}
            }
        }
        
        drawer.set_draw_color(RGB(50, 100, 150));

        drawer.clear();
        for i in range(1, 5) {
            for b in blocks.iter() {
                if b.t == i {
                    let color = match b.t {
                        1 => RGB(0, 153, 204),
                        2...3 => RGB(180, 30, 20),
                        4 => RGB(250, 250, 255),
                        _ => RGB(0,0,0),
                    };
                    drawer.set_draw_color(color);
                    let square = get_rect(&b);
                    drawer.fill_rect(square);
                }
            }
        }

        drawer.set_draw_color(RGB(254,180,204));
        let inner = get_rect(&draw_block);
        drawer.draw_rect(inner);

        drawer.present();
    }

    sdl2::quit();
}

fn get_rect(b: &Block) -> Rect {
    return match b.t {
                1 => Rect::new(b.x, b.y, b.block_size, b.block_size),
                2 => Rect::new(b.x-1, b.y, 2, b.block_size),
                3 => Rect::new(b.x, b.y-1, b.block_size, 2),
                4 => Rect::new(b.x-3, b.y-3, 6, 6),
                _ => Rect::new(b.x, b.y, b.block_size, b.block_size),
            };
}

/*
fn load_map() -> Vec<Block> {
    let mut blocks = Vec::new();

    let mut file = File::open_mode(&Path::new("savedmap.map"),std::old_io::FileMode:: Open,
    std::old_io::FileAccess::Read);
    let size = match file.read_be_i32() {
        Ok(s) => s,
        Err(e) => panic!("Error {}", e),
    };
    println!("Received {} objects.", size);
    for _ in range(0, size) {
        let blocktype = match file.read_be_i32() {
            Ok(t) => t,
            Err(e) => panic!("Error {}", e),
        };
        let bx = match file.read_be_f32() {
            Ok(x) => x,
            Err(e) => panic!("Error {}", e),
        };
        let by = match file.read_be_f32() {
            Ok(y) => y,
            Err(e) => panic!("Error {}", e),
        };
        println!("{}, {}, {}", blocktype, (bx*32f32) as i32, (by*32f32) as i32);
        blocks.push( Block{x: (bx*32f32) as i32, y: (by*32f32) as i32, t: 1} );
    }
    blocks
}
*/

fn save_map(map: &Vec<Block>) -> bool{
    let mut file = File::create(&Path::new("savedmap.map"));
    let _ = file.write_be_i32(map.len() as i32);
    for block in map.iter() {
        let _ = file.write_be_u32(block.t);
        let _ = file.write_be_f32(block.x as f32 / 32f32);
        let _ = file.write_be_f32(block.y as f32 / 32f32);
    }
    println!("Written!");
    true
}
