#![feature(old_io)]
#![feature(core)]

extern crate sdl2;
extern crate rustnet;
mod player;

use player::Player;
use std::thread;
use std::sync::mpsc;
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
                "quit" => { let _ = tx.send(7u32); },
                _ => println!("Unrecognized input."),
            }
        }
    } );
    

    'main:loop {
        if rustnet::check_sockets() {

            let temp_client = rustnet::check_for_new_client();
            match temp_client{
                None => (),
                Some(sock) => {
                    player_nums += 1;
                    let mut p = player::new(sock, player_nums);

                    for player in &mut players {
                        rustnet::clear_buffer();
                        rustnet::write_byte(1);
                        rustnet::write_byte(p.player_id() as u8);
                        rustnet::send_message(player.socket());

                        rustnet::clear_buffer();
                        rustnet::write_byte(1);
                        rustnet::write_byte(player.player_id() as u8);
                        rustnet::send_message(p.socket());
                    }

                    players.push(p);
                },
            }

           
            let msg_sizes = |msg_id: u8| {
                match msg_id {
                    2 => 8,
                    _ => 1,
                }
            };

            for player in &mut players {
                if !player.socket().read_socket(){
                    println!("Lost connection to socket.");
                }
            }
            
            for i in range(0, players.len()) {
                while players[i].socket().has_msg(&msg_sizes) {
                    match players[i].read_byte() {
                        2=> {
                            let p_id = players[i].player_id();
                            let newx = players[i].read_float();
                            let newz = players[i].read_float();
                            for p in &mut players { 
                                if p.player_id() != p_id {
                                    rustnet::clear_buffer();
                                    rustnet::write_byte(2);
                                    rustnet::write_float(newx);
                                    rustnet::write_float(newz);
                                    rustnet::send_message(p.socket());
                                }
                            }
                        },
                        _ => println!("Unknown message."),
                    }
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

