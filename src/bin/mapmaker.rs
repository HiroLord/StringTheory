#![feature(core)]
#![feature(old_io)]
#![feature(old_path)]

extern crate sdl2;
//mod camera;

use std::old_path::Path;
use std::old_io::{File, Reader, Writer};
use sdl2::video::{Window, WindowPos};
use sdl2::event::{Event};
use sdl2::keycode::KeyCode;
use sdl2::pixels::Color::RGB;
use sdl2::rect::Rect;

use sdl2::render::RenderDrawer as Drawer;

struct Block { 
    x: i32,
    y: i32,
    t: u32,
    block_size: i32,
}

trait Drawable {
    fn draw(&self, drawer: &mut Drawer);
    //fn force_draw_color(&self, RGB(254,180,204));
}

impl Drawable for Block {
    fn draw(&self, drawer: &mut Drawer) {
        let color = match self.t {
            1 => RGB(0, 153, 204),
            2...3 => RGB(180, 30, 20),
            4 => RGB(250, 250, 255),
            5...6 => RGB(20, 30, 255),
            10 => RGB(0,155,0),
            _ => RGB(0,0,0),
        };

        drawer.set_draw_color(color);

        let dr_x = self.x;
        let dr_y = self.y;

        let mut square = match self.t {
            1 => Rect::new(dr_x, dr_y, self.block_size, self.block_size),
            2 | 5 => Rect::new(dr_x, dr_y, 2, self.block_size),
            3 | 6 => Rect::new(dr_x, dr_y, self.block_size, 2),
            4 => Rect::new(dr_x, dr_y, 6, 6),
            10 => Rect::new(dr_x, dr_y, self.block_size-12, self.block_size-12),
            _ => Rect::new(dr_x, dr_y, self.block_size, self.block_size),
        };

        square.x -= square.w/2;
        square.y -= square.h/2;

        drawer.fill_rect(square);
    }
}

fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();

    let args: Vec<String> = std::env::args().collect();
    let map_name: &str = match args.len() > 1 {
        true => args[1].as_slice(),
        false => "savedmap".as_slice(),
    };

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

    let block_size = 32;

    let mut blocks: Vec<Block> = load_map(map_name);

    let mut draw_block = Block{ x: 0, y: 0, t: 1, block_size: block_size }; 

    let mut event_pump = sdl_context.event_pump();

    'main:loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::MouseMotion{x: mx, y: my, ..} => {
                    let snap = match draw_block.t {
                        4 => block_size/2,
                        _ => block_size,
                    };
                    let (xoff, yoff) = match draw_block.t {
                        2|5 => (block_size/2, 0),
                        3|6 => (0, block_size/2),
                        _ => (0, 0),
                    };
                    draw_block.x = mx - ((mx + xoff) % snap) + block_size/2;
                    draw_block.y = my - ((my + yoff) % snap) + block_size/2;
                }
                Event::MouseButtonUp{mouse_btn: btn, ..} => {
                    if btn == sdl2::mouse::Mouse::Left{
                        let mut add = true;
                          
                        for black in blocks.iter() {
                            if black.t == draw_block.t {
                                if black.x == draw_block.x && black.y == draw_block.y {
                                    add = false;
                                }
                            }
                        }
                        
                        if add {
                            blocks.push( Block{x: draw_block.x, y: draw_block.y,
                                t: draw_block.t, block_size: block_size} );
                        }
                    } else if btn == sdl2::mouse::Mouse::Right {
                        let mut to_remove = Vec::new();
                        for b in 0..blocks.len() {
                            if blocks[b].t == draw_block.t {
                                if blocks[b].x == draw_block.x && blocks[b].y == draw_block.y {
                                    to_remove.push(b);
                                }
                            }
                        }
                        let mut off = 0;
                        to_remove.sort();
                        for b in to_remove.iter() {
                            blocks.remove(*b - off);
                            off += 1;
                        }
                    }
                }
                Event::KeyDown{keycode: key, ..} => {
                    if key == KeyCode::Escape { break 'main; }
                    if key == KeyCode::S { save_map(&blocks, map_name); }
                    //if key == KeyCode::L { blocks = load_map(); }
                    if key == KeyCode::Num1 { draw_block.t = 1; }
                    if key == KeyCode::Num2 { draw_block.t = 2; }
                    if key == KeyCode::Num3 { draw_block.t = 3; }
                    if key == KeyCode::Num4 { draw_block.t = 4; }
                    if key == KeyCode::Num5 { draw_block.t = 5; }
                    if key == KeyCode::Num6 { draw_block.t = 6; }
                    if key == KeyCode::Num0 { draw_block.t = 10; }
                    if key == KeyCode::M {
                        save_map(&blocks, map_name);
                        println!("Loading game.");
                        let k =
                        std::process::Command::new("cargo").arg("run").arg("--bin").arg("StringTheory").arg(map_name)
                            .status().unwrap_or_else(|e| {
                            panic!("Failed: {}", e);
                        });
                    }
                    if key == KeyCode::W {
                        let mut to_add = Vec::new();
                        for block in blocks.iter() {
                            if block.t == 1 {
                                let mut left = true;
                                let mut right = true;
                                let mut up = true;
                                let mut down = true;
                                for b2 in blocks.iter() {
                                    if b2.t == 1{
                                        if b2.x == block.x-block_size && b2.y == block.y { left = false; }
                                        if b2.x == block.x+block_size && b2.y == block.y { right = false; }
                                        if b2.y == block.y-block_size && b2.x == block.x { up = false; }
                                        if b2.y == block.y+block_size && b2.x == block.x { down = false; }
                                    }
                                    if b2.t == 2 {
                                        if b2.x == block.x && b2.y == block.y { left = false; }
                                        if b2.x == block.x + block_size && b2.y == block.y { right
                                            = false; }
                                    }
                                    if b2.t == 3 {
                                        if b2.x == block.x && b2.y == block.y { up = false; }
                                        if b2.x == block.x && b2.y == block.y + block_size { down =
                                            false; }
                                    }
                                }
                                if left {
                                    to_add.push( Block{x: block.x-block_size/2, y: block.y, t: 2, block_size:
                                        block_size} );
                                }
                                if up {
                                    to_add.push( Block{x: block.x, y: block.y-block_size/2, t: 3, block_size:
                                        block_size} );
                                }
                                if right {
                                    to_add.push( Block{x: block.x+block_size/2, y: block.y, t: 2,
                                        block_size: block_size} );
                                }
                                if down {
                                    to_add.push( Block{x: block.x, y: block.y+block_size/2, t: 3,
                                        block_size: block_size} );
                                }
                            }
                        }
                        for w in to_add {
                            blocks.push(w);
                        }
                    }
                }
                _ => {}
            }
        }
        
        drawer.set_draw_color(RGB(50, 100, 150));

        drawer.clear();
        for i in 1..11 {
            for b in blocks.iter() {
                if b.t == i {
                    //drawer.set_draw_color(color);
                    //let square = get_rect(&b);
                    b.draw(&mut drawer);
                }
            }
        }

        //drawer.set_draw_color(RGB(254,180,204));
        //draw_block.force_draw_color(RGB(254,180,204));
        draw_block.draw(&mut drawer);

        drawer.present();
    }
}

fn load_map<'a>(raw_name: &'a str) -> Vec<Block> {
    let mut blocks = Vec::new();

    let name = format!("maps/{}.map", raw_name);

    let mut file = File::open_mode(&Path::new(name.clone()),std::old_io::FileMode:: Open,
    std::old_io::FileAccess::Read);
    let size = match file.read_be_i32() {
        Ok(s) => s,
        Err(e) => {println!("{}", e); println!("New map will be saved as {}", name);return blocks;}
    };
    println!("Received {} objects.", size);
    for _ in 0..size {
        let blocktype = match file.read_be_u32() {
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
        blocks.push( Block{x: (bx*8f32) as i32, y: (by*8f32) as i32, t: blocktype, block_size: 32} );
    }
    blocks
}

fn save_map<'a>(map: &Vec<Block>, raw_name: &'a str) -> bool{
    let name = format!("maps/{}.map", raw_name);
    let mut file = File::create(&Path::new(name.clone()));
    let _ = file.write_be_i32(map.len() as i32);
    for block in map.iter() {
        let _ = file.write_be_u32(block.t);
        let _ = file.write_be_f32(block.x as f32 / 8f32);
        let _ = file.write_be_f32(block.y as f32 / 8f32);
    }
    println!("Map saved as {}", name);
    true
}
