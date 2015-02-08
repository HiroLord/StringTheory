extern crate sdl2;
extern crate collections;
extern crate gl;
extern crate rustnet;

use sdl2::video::{Window, WindowPos, OPENGL, gl_set_attribute};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;
use sdl2::event::{Event, poll_event};
//use sdl2::event::poll_event;
//use sdl2::event::Event::{Quit, KeyDown};
use sdl2::keycode::KeyCode;

use gl::types::*;
use std::mem;
use std::ptr;
use std::str;
use std::ffi;
use collections::vec;


mod object;
mod shader;

fn main() {
    sdl2::init(sdl2::INIT_VIDEO);

    let port: u16 = 1231;

    let mut connected = true;

    if !rustnet::init_client("127.0.0.1", port) {
        println!("Unable to connect to server on port {}", port);
        connected = false;
    } else {
        println!("Connected!");
    }

    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextProfileMask, sdl2::video::GLProfile::GLCoreProfile as i32);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMajorVersion, 3);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLContextMinorVersion, 3);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDoubleBuffer, 1);
    sdl2::video::gl_set_attribute(sdl2::video::GLAttr::GLDepthSize, 24);
    let window = match Window::new("rust-sdl2: Video", WindowPos::PosCentered, WindowPos::PosCentered, 800, 600, OPENGL) {
        Ok(window) => window,
        Err(err) => panic!("faid to create window: {}", err)
    };

    // MUST ASSIGN RESULT THIS TO A VARIABLE
    // Otherwise, it gets deleted or is optimized out or something
    let context = window.gl_create_context().unwrap();
    gl::load_with(|s| unsafe { std::mem::transmute(sdl2::video::gl_get_proc_address(s)) });

    //let obj = object::new(-0.5, -0.5, -0.5, -1.5, -1.5, -1.5);
    //let obj = object::new(-0.5, -0.5, -0.5, -1.5, -1.5, -1.5);
    //let obj = object::new(0.5, 0.5, 0.5, -1.5, -1.5, -1.5);
    let obj = object::newTri();

    let mut sent = false;

    loop {
        match poll_event() {
            Event::Quit{..} => break,
            Event::KeyDown{keycode: key, ..} => {
                if key == KeyCode::Escape {
                    break;
                }
            }
            _ => {}
        }
        obj.draw();
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
    }
    sdl2::quit();
}

fn user_defined(msg_id: u8) -> u32 {
    0
}

fn can_handle(msg_id: u8, buffer_size: u32) -> bool {
    true
}
