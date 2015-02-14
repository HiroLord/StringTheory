#![feature(core)]
#![feature(collections)]
#![feature(std_misc)]

extern crate sdl2;
extern crate collections;
extern crate gl;
extern crate rustnet;
//extern crate time;

use sdl2::video::{Window, WindowPos, OPENGL, gl_set_attribute};
//use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
//use sdl2::pixels::Color;
use sdl2::event::{Event, poll_event};
//use sdl2::event::poll_event;
//use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;


mod object;
mod shader;
mod camera;
mod matrix;
mod player;
mod solids;
mod mapgen;

use solids::GameObject;

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);
    let window_width = 1280;
    let window_height = 720;

    let mouse_sense = 0.5f32;

    let port: u16 = 1231;

    let mut connected = true;

    if !rustnet::init_client("127.0.0.1", port) {
        println!("Unable to connect to server on port {}", port);
        connected = false;
    } else {
        println!("Connected!");
    }

    
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask, sdl2::video::GLProfile::GLCompatibilityProfile as i32);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 2);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 1);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDoubleBuffer, 1);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDepthSize, 24);
    let window = match Window::new("rust-sdl2: Video", WindowPos::PosCentered, WindowPos::PosCentered, window_width, window_height, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("Failed to create window: {}", err)
    };

    // MUST ASSIGN RESULT THIS TO A VARIABLE
    // Otherwise, it gets deleted or is optimized out or something
    let context = window.gl_create_context().unwrap();
    gl::load_with(|s| unsafe { std::mem::transmute(sdl2::video::gl_get_proc_address(s)) });

    //unsafe { gl::Disable(gl::CULL_FACE); }
    unsafe {
        gl::Enable(gl::CULL_FACE);
        gl::Enable(gl::DEPTH_TEST);
    }


    let obj = object::new(-0.5, -0.5, -1.5,    0.5, 0.5, -2.5,    0.8, 0.9, 0.4);
    let mut obj2 = object::new(0.5, 0.5, -1.5,     1.5, 1.5, -2.5,    1.0, 0.4, 0.2);
    let aspect_ratio = window_width as f32 / window_height as f32;
    let mut camera = camera::new(60.0f32, aspect_ratio, 0.0f32, 100.0f32);

    let mut sent = false;

    let midx = window_width / 2;
    let midy = window_height / 2;
    sdl2::mouse::warp_mouse_in_window(&window, midx, midy); 

    let mut player = player::new(0f32, 2f32, 0f32, 1f32);

    let mut map = mapgen::new_map(1);

    let mut running = true;

    let mut forward = 0f32;
    let mut strafe = 0f32;

    while running {
        let mut polling = true;
        while polling {
            match poll_event() {
                Event::Quit{..} => running = false,
                Event::MouseMotion{x: mx, y: my, ..} => {
                    let midx = window_width / 2;
                    let midy = window_height / 2;
                    
                    let difx = midx - mx;
                    let dify = midy - my;
                    camera.change_horizontal_angle(mouse_sense*0.01f32*(difx as f32));
                    camera.change_vertical_angle(mouse_sense*0.01f32*(dify as f32));
                    
                    sdl2::mouse::warp_mouse_in_window(&window, midx, midy); 
                }
                Event::KeyDown{keycode: key, ..} => {
                    if key == KeyCode::Escape { running = false; }
                    if key == KeyCode::W { forward = 1f32; }
                    if key == KeyCode::S { forward = -1f32; }
                    if key == KeyCode::A { strafe = -1.0f32; }
                    if key == KeyCode::D { strafe = 1.0f32; }
                    //if key == KeyCode::Z { obj2.set_translation(0.0f32, 0.0f32, 0.0f32); }
                    //if key == KeyCode::X { obj2.set_translation(1.0f32, 1.0f32, 1.0f32); }
                    if key == KeyCode::Z { obj2.translate(-0.1f32, -0.1f32, -0.1f32); }
                    if key == KeyCode::X { obj2.translate(0.1f32, 0.1f32, 0.1f32); }
                }
                Event::KeyUp{keycode: key, ..} => {
                    if key == KeyCode::W { forward = 0f32; }
                    if key == KeyCode::A { strafe = 0f32; }
                    if key == KeyCode::D { strafe = 0f32; }
                    if key == KeyCode::S { forward = 0f32; }
                }
                Event::None => polling = false,
                _ => {}
            }
        }

        player.forward(&camera, forward);
        player.strafe(&camera, strafe);
        player.move_self();

        camera.snap_to_player(&player);
        camera.update_view_projection();

        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        obj.draw(&camera);
        obj2.draw(&camera);
        //obj3.draw(&camera);

        for i in range(0, map.get_floors().len()){
            map.get_floors()[i].draw(&camera);
            //o.draw(&camera);
        }

        window.gl_swap_window();
        if connected {
            if rustnet::check_sockets(){
                if !rustnet::read_server_socket(can_handle, user_defined){
                    println!("Lost server connection.");
                    break;;
                }
            }

            if !sent {
                rustnet::clear_buffer();
                rustnet::write_byte(1);
                rustnet::write_byte(5);
                rustnet::send_ts_message();
                sent = true;
            }
        }
        sdl2::timer::delay(15);
    }
    sdl2::quit();
}

fn key_input() {
}

fn user_defined(msg_id: u8) -> u32 {
    0
}

fn can_handle(msg_id: u8, buffer_size: u32) -> bool {
    true
}
