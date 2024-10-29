use std::{
    collections::HashMap,
    future::Future,
    io,
    net::SocketAddr,
    sync::Arc,
};

use tokio::{
    io::AsyncWriteExt,
    net::tcp::OwnedWriteHalf,
    sync::Mutex,
};

#[derive(Debug, Default)]
pub struct Shared {
    pub peer_connections: HashMap<SocketAddr, OwnedWriteHalf>,
}

impl Shared {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

pub trait SharedTrait {
    fn remove(&mut self, key: &SocketAddr) -> impl Future<Output = ()> + Send;
    fn insert(
        &mut self,
        key: SocketAddr,
        writer_half: OwnedWriteHalf,
    ) -> impl Future<Output = ()> + Send;
    fn broadcast(
        &self,
        author: SocketAddr,
        message: String,
    ) -> impl Future<Output = Result<(), io::Error>> + Send;
}

impl SharedTrait for Arc<Mutex<Shared>> {
    async fn remove(&mut self, key: &SocketAddr) {
        self.lock().await.peer_connections.remove(key);
    }

    async fn insert(&mut self, key: SocketAddr, writer_half: OwnedWriteHalf) {
        self.lock().await.peer_connections.insert(key, writer_half);
    }

    async fn broadcast(&self, author: SocketAddr, message: String) -> Result<(), io::Error> {
        let mut state = self.lock().await;
        let connections = &mut state.peer_connections;
        println!("[BROADCASTING]\n{message}\n");

        for (user, stream) in connections.iter_mut() {
            if *user != author {
                stream.write_all(message.as_bytes()).await?;
            }
        }

        Ok(())
    }
}
