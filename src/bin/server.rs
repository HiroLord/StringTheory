#![feature(old_io)]

extern crate sdl2;
extern crate rustnet;
mod player;

use player::Player;
use std::thread;
use std::sync::mpsc;
use rustnet::TCPsocket;
//use player::Player;

fn main() {
    let _ = sdl2::init(sdl2::sdl::INIT_EVENTS);
 
    let port: u16 = 1231;

    if !rustnet::init_server(port, 100) {
        println!("Unable to start server on port {}", port);
        return
    } else {
        println!("Started StringTheory server on port {}", port);
    }

    let mut players: Vec<Player> = Vec::new();
    let mut player_nums = 0;

    let (tx, rx) = mpsc::channel();
    
    thread::spawn( move || {
        loop {
            let input = std::old_io::stdin().read_line().ok().expect("Failed to read input");
            match input.trim() { 
                "quit" => { tx.send(7u32); },
                _ => println!("Unrecognized input."),
            }
        }
    } );
    

    'main:loop {
        if rustnet::check_sockets() {

            let temp_client = rustnet::check_for_new_client();
            match temp_client{
                None => (),
                Some(s) => {
                    player_nums += 1;
                    players.push(player::new(s, player_nums));
                },
            }

           
            let can_handle = |msg_id: u8| {
                match msg_id {
                    1 => 4,
                    _ => 1,
                }
            };

            let user_defined = |msg_id: u8, socket: &TCPsocket| {
                match msg_id {
                    1 => {
                        let x = rustnet::read_float();
                        println!("Received float {}", x);
                    },
                    _ => println!("Unknown message"),
                }
            };

            for player in players.iter() {
                if !rustnet::read_socket(&(player.socket()), &can_handle, &user_defined){
                }
            }
            
        }

        let recv = rx.try_recv();
        if recv.is_ok() {
            if recv.unwrap() == 7u32 {
                break 'main;
            }
        }

        sdl2::timer::delay(10);
    }
    println!("Server closed.");
}

struct Controller {
    players: Vec<Player>,   
}

fn listen_for_input() {
}

impl Controller {

    fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }

    pub fn user_defined(&mut self, msg_id: u8, socket: &TCPsocket) -> u32 {
        match msg_id {
            1 => {
                let x = rustnet::read_float();
                println!("Received float {}", x);

                return 4
            },
            _ => return 1
        }
    }

    pub fn can_handle(msg_id: u8, buffer_size: u32) -> bool {
        match msg_id {
            1 => return buffer_size >= 4,
            _ => return true
        }
    }
}
