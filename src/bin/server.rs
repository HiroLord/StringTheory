#![feature(core)] 
extern crate sdl2;
extern crate rustnet;
mod player;

use player::IsPlayer;

//use player::Player;

fn main() {
    let sdl_context = sdl2::init(sdl2::sdl::INIT_EVENTS);
 
    let port: u16 = 1231;

    if !rustnet::init_server(port, 100) {
        println!("Unable to start server on port {}", port);
        return
    }

    let mut players = Vec::new();

    let mut running = true;

    while running {
        if rustnet::check_sockets() {

            let temp_client = rustnet::check_for_new_client();
            match temp_client{
                None => (),
                Some(s) => players.push(player::new(s)),
            }

            for player in players.iter() {
                if !rustnet::read_socket(&(player.socket()), can_handle, user_defined){
                    println!("Lost user connection. IMPLEMENT!");
                    running = false;
                }
            }
        }
        sdl2::timer::delay(20);
    }
}

fn user_defined(msg_id: u8) -> u32 {
    match msg_id {
        1 => {
            let k = rustnet::read_byte();
            println!("Received byte {}", k);
            return 1
        },
        _ => return 1
    }
}

fn can_handle(msg_id: u8, buffer_size: u32) -> bool {
    match msg_id {
        1 => return buffer_size >= 1,
        _ => return true
    }
}
