extern crate rustnet;
//extern crate sdl2_net;

pub struct Player {
    socket: rustnet::SocketWrapper,
    player_id: u32,
}

impl Player {
    pub fn socket(&mut self) -> &mut rustnet::SocketWrapper {
        &mut self.socket
    }

    pub fn player_id(&self) -> u32 { self.player_id }

    pub fn read_byte(&mut self) -> u8 {
        self.socket.read_byte()
    }

    pub fn read_float(&mut self) -> f32 {
        self.socket.read_float()
    }
}

pub fn new(socket: rustnet::SocketWrapper, p_id: u32) -> Player{
    Player{socket: socket, player_id: p_id}
}
