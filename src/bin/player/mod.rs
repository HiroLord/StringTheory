extern crate rustnet;
//extern crate sdl2_net;

pub struct Player {
    socket: rustnet::TCPsocket,
}

pub trait IsPlayer {
    fn socket(&self) -> &rustnet::TCPsocket;
}

impl IsPlayer for Player {
    fn socket(&self) -> &rustnet::TCPsocket {
        &(self.socket)
    }
}

pub fn new(socket: rustnet::TCPsocket) -> Player{
    Player{socket: socket}
}
