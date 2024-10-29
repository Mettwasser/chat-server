use std::net::SocketAddr;

use crate::event::Event;

pub struct Message {
    pub event: Event,
    pub author: SocketAddr,
}

impl Message {
    pub fn new(event: Event, addr: SocketAddr) -> Self {
        Self {
            author: addr,
            event,
        }
    }
}
