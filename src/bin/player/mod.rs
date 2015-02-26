extern crate rustnet;
//extern crate sdl2_net;

pub struct Player {
    socket: rustnet::TCPsocket,
    player_id: u32,
}

impl Player {
    pub fn socket(&self) -> &rustnet::TCPsocket {
        &(self.socket)
    }

    pub fn player_id(&self) -> u32 { self.player_id }
}

pub fn new(socket: rustnet::TCPsocket, p_id: u32) -> Player{
    Player{socket: socket, player_id: p_id}
}
