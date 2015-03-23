#![feature(core)]
#![feature(collections)]
#![feature(std_misc)]
#![feature(old_io)]
#![feature(old_path)]

extern crate sdl2;
extern crate collections;
extern crate gl;
extern crate rustnet;
//extern crate assimp;
//extern crate time;

use sdl2::video::{Window, WindowPos, OPENGL, gl_set_attribute};
//use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
//use sdl2::pixels::Color;
use sdl2::event::{Event};
//use sdl2::event::poll_event;
//use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;
use sdl2::timer::get_ticks;

mod object;
mod shader;
mod camera;
mod matrix;
mod player;
mod solids;
mod mapgen;
mod light;
mod gbuffer;
mod renderer;
mod resourcemanager;

use player::Player;
use solids::Mask;
use solids::Solid;
use solids::GameObject;

#[allow(unused_variables)]
fn main() {
    let sdl_context = sdl2::init(sdl2::INIT_VIDEO).unwrap();
    let window_width = 1280;
    let window_height = 720;

    let mouse_sense = 0.5f32;

    let port: u16 = 1231;

    //let ip = "128.61.104.39"; // Desktop
    //let ip = "lr.room409.xyz";
    //let ip = "room409.xyz";
    //let ip = "192.168.1.146";
    let ip = "127.0.0.1";

    let option = rustnet::init_client(ip, port);
    let mut socket: rustnet::SocketWrapper;

    match option {
        Some(sock) => socket = sock,
        None => {
                println!("Unable to connect to {}:{}", ip, port);
                return;
        },
    }
    
    //sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask, sdl2::video::GLProfile::GLCoreProfile as i32);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask, sdl2::video::GLProfile::GLCompatibilityProfile as i32);

    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 2);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 1);
    
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDoubleBuffer, 1);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDepthSize, 24);
    let window = match Window::new("Corridors", WindowPos::PosCentered, WindowPos::PosCentered, window_width, window_height, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("Failed to create window: {}", err)
    };

    // MUST ASSIGN RESULT THIS TO A VARIABLE
    // Otherwise, it gets deleted or is optimized out or something
    let context = window.gl_create_context().unwrap();
    gl::load_with(|s| unsafe { std::mem::transmute(sdl2::video::gl_get_proc_address(s)) });

    let renderer = renderer::new(window_width as u32, window_height as u32);

    let aspect_ratio = window_width as f32 / window_height as f32;
    let mut camera = camera::new(60.0f32, aspect_ratio, 0.0f32, 100.0f32);

    sdl2::mouse::show_cursor(false);

    let midx = window_width / 2;
    let midy = window_height / 2;
    sdl2::mouse::warp_mouse_in_window(&window, midx, midy); 

    let mut manager : resourcemanager::ResourceManager = resourcemanager::new();

    let mut map = mapgen::new_map(1, &mut manager);

    let play = manager.new_player(0, 0f32, 1.5f32, 0f32, 1f32);
    let mut players: Vec<Player> = Vec::new();
    players.push(play);

    if map.get_spawns().len() > 0 {
        players[0].set_position_from_point(map.get_spawn(0));
    } else {
        players[0].set_x(map.get_floors()[0].x());
        players[0].set_z(map.get_floors()[0].z());
    }

    let (start_x, start_z) = (players[0].x(), players[0].z());

    let mut running = true;

    let mut forward = 0f32;
    let mut strafe = 0f32;
  
    let mut event_pump = sdl_context.event_pump();

    let _ = sdl2::joystick::Joystick::open(0);
    let mut event_pump = sdl_context.event_pump();

    let mut start_time = get_ticks();
    let mut frames = 0;
    let mut last_time = get_ticks();


    let mut last_look_time: u32 = 0;
    let mut last_pos_time: u32 = 0;
    
    while running {

        let delta: f32 = ((get_ticks() - last_time) as f32)/16.66666f32;
        last_time = get_ticks();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} => running = false,
                Event::MouseMotion{x: mx, y: my, ..} => {
                    let midx = window_width / 2;
                    let midy = window_height / 2;
                    
                    let difx = midx - mx;
                    let dify = midy - my;
                    camera.change_horizontal_angle(mouse_sense*0.005f32*(difx as f32));
                    camera.change_vertical_angle(mouse_sense*0.005f32*(dify as f32));
                    
                    sdl2::mouse::warp_mouse_in_window(&window, midx, midy); 
                },
                Event::KeyDown{keycode: key, ..} => {
                    if key == KeyCode::Escape { running = false; }
                    if key == KeyCode::W { forward = 1f32; }
                    if key == KeyCode::S { forward = -1f32; }
                    if key == KeyCode::A { strafe = -1.0f32; }
                    if key == KeyCode::D { strafe = 1.0f32; }
                    if key == KeyCode::E {
                        for i in 0..map.get_doors().len() {
                            map.open_door(i as i32);
                        }
                    }
                },
                Event::KeyUp{keycode: key, ..} => {
                    if key == KeyCode::W { forward = 0f32; }
                    if key == KeyCode::A { strafe = 0f32; }
                    if key == KeyCode::D { strafe = 0f32; }
                    if key == KeyCode::S { forward = 0f32; }
                    if key == KeyCode::E {
                        for i in 0..map.get_doors().len() {
                            map.close_door(i as i32);
                        }
                    }
                },
                _ => {}
            }
        }

        let msg_size = |msg_id: u8| -> u32{
            match msg_id {
                0 => 4,
                1 => 1,
                2 => 9,
                3 => 9,
                4 => 9,
                _ => 1,
            }
        };

        if rustnet::check_sockets(0) {
            if !socket.read_socket() {
            } else {
                while socket.has_msg(&msg_size) {
                    match socket.read_byte() {
                        0 => {
                            println!("Received raw float {}", socket.read_float() );
                        },
                        1 => {
                            let new_id = socket.read_byte() as u32;
                            println!("New player! {} ", new_id);
                            //let new_player = player::new(new_id, start_x, 1.5f32, start_z, 1f32);
                            let new_player = manager.new_player(new_id, start_x, 1.5f32, start_z, 1f32);
                            players.push(new_player);
                        },
                        2 => {
                            let p_id = socket.read_byte() as u32;
                            let p_ha = socket.read_float();
                            let p_va = socket.read_float();
                            for p in &mut players {
                                if p.player_id() == p_id {
                                    p.set_horizontal_angle(p_ha);
                                    p.set_vertical_angle(p_va);
                                    break;
                                }
                            }
                        },
                        3 => {
                            let p_id = socket.read_byte() as u32;
                            let p_x = socket.read_float();
                            let p_z = socket.read_float();
                            for p in &mut players {
                                if p.player_id() == p_id {
                                    p.set_x(p_x);
                                    p.set_z(p_z);
                                    break;
                                }

                            }
                        },
                        4 => {
                            let p_id = socket.read_byte() as u32;
                            let p_fb = socket.read_float();
                            let p_lr = socket.read_float();
                            for p in &mut players {
                                if p.player_id() == p_id {
                                    p.forward(p_fb);
                                    p.strafe(p_lr);
                                    break;
                                }

                            }
                        },
                        _ => println!("Unknown message"),
                    }
                }
            }
        }

        players[0].set_look_vector(&camera);
        players[0].forward(forward);
        players[0].strafe(strafe);

        for p in 0..players.len() {
            let (dx, dz) = players[p].get_move(delta);
           
            players[p].move_x(dx);
            let mut i = 0;
            let mut maxlen = map.get_walls().len();
            if map.get_doors().len() > maxlen {
                maxlen = map.get_doors().len();
            }
            while i < maxlen {
                if i < map.get_walls().len() && check_collision(players[p].get_mask(), map.get_walls()[i].get_mask()) {
                    players[p].move_x(-dx);
                    align_x(&mut players[p], (map.get_walls()[i].get_mask()));
                    //break;
                }
                if i < map.get_doors().len() && !map.get_doors()[i].is_open() && check_collision(players[p].get_mask(), 
                                                                map.get_doors()[i].get_mask()) {
                    players[p].move_x(-dx);
                    align_x(&mut players[p], (map.get_doors()[i].get_mask()));
                    //break;
                }
                i += 1;
            }

            i = 0;
            players[p].move_z(dz);
            while i < maxlen {
                if i < map.get_walls().len() && check_collision(players[p].get_mask(), map.get_walls()[i].get_mask()) {
                    players[p].move_z(-dz);
                    align_z(&mut players[p], (map.get_walls()[i].get_mask()));
                    //break;
                }
                if i < map.get_doors().len() && !map.get_doors()[i].is_open() && check_collision(players[p].get_mask(),
                                                            map.get_doors()[i].get_mask()) {
                    players[p].move_z(-dz);
                    align_z(&mut players[p], (map.get_doors()[i].get_mask()));
                    //break;
                }

                i += 1;
            }
        }

        camera.snap_to_player(&players[0]);
        camera.update_view_projection();

        if players[0].look_changed() && get_ticks() > last_look_time + 50u32 {
            rustnet::clear_buffer();
            rustnet::write_byte(2);
            rustnet::write_float(players[0].hor_angle());
            rustnet::write_float(players[0].ver_angle());
            rustnet::send_message(&socket);
            players[0].make_old_look_new();
            last_look_time = get_ticks();
        }

        if get_ticks() > last_pos_time + 2000u32 {
            rustnet::clear_buffer();
            rustnet::write_byte(3);
            rustnet::write_float(players[0].x());
            rustnet::write_float(players[0].z());
            rustnet::send_message(&socket);
            last_pos_time = get_ticks();
        }

        if players[0].fb_lr_changed() {
            rustnet::clear_buffer();
            rustnet::write_byte(4);
            rustnet::write_float(-players[0].fb());
            rustnet::write_float(players[0].lr());
            rustnet::send_message(&socket);
            players[0].make_old_fb_new();
        }

        renderer.start_geometry_pass();

        for p in &players{
            p.draw(&camera, &renderer);
        }
        
        
        map.get_floors()[0].bind_shader();
        for floor in map.get_floors() {
            floor.draw(&camera, &renderer);
        }

        for door in map.get_doors() {
            door.draw(&camera, &renderer);
        }
        
        for wall in map.get_walls() {
            wall.draw(&camera, &renderer);
        }
        
        renderer.start_light_pass();
        map.get_lights()[0].bind_shader();
        for light in map.get_lights() {
            light.draw(&camera, &renderer);
        }
        
        window.gl_swap_window();
        
        let time = sdl2::timer::get_ticks();
        frames += 1;

        if time - start_time >= 1000 {
            start_time = time;
            println!("fps: {}", frames);
            frames = 0;
        }
    }
}


fn align_x(p: &mut Player, obj: &Mask) {
    let rad = p.get_mask().width()/2.0;
    if p.x() > obj.x() {
        p.set_x(obj.get_right() + rad);
    } else {
        p.set_x(obj.get_left() - rad);
    }
}

fn align_z(p: &mut Player, obj: &Mask) {
    let rad = p.get_mask().length()/2.0;
    if p.z() > obj.z() {
        p.set_z(obj.get_front() + rad);
    } else {
        p.set_z(obj.get_back() - rad);
    }
}

fn check_collision(obja: &Mask, objb: &Mask) -> bool{
    if obja.x() < objb.get_right() + obja.width()/2.0 && obja.x() > objb.get_left() - obja.width()/2.0 {
        if obja.z() < objb.get_front() + obja.length()/2.0 && obja.z() > objb.get_back() -
            obja.length()/2.0 {
            return true
        }
    }
    false
}

