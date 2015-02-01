extern crate sdl2;
extern crate collections;
extern crate gl;
extern crate rustnet;

use sdl2::video::{Window, WindowPos, OPENGL, gl_set_attribute};
use sdl2::render::{RenderDriverIndex, ACCELERATED, Renderer};
use sdl2::pixels::Color;
use sdl2::event::poll_event;
use sdl2::event::Event::{Quit, KeyDown};
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

    let obj = object::new();

    loop {
        match poll_event() {
            Quit(_) => break,
            KeyDown(_, _, key, _, _, _) => {
                if key == KeyCode::Escape {
                    break;
                }
            }
            _ => {}
        }
        obj.draw();
        window.gl_swap_window();
    }
    sdl2::quit();
}
