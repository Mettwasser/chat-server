use std::{
    net::SocketAddr,
    sync::Arc,
};

use chrono::Local;
use futures::StreamExt;
use server::{
    event::Event,
    message::Message,
    shared::{
        Shared,
        SharedTrait,
    },
    Result,
    Rx,
    State,
    Tx,
};
use tokio::{
    net::{
        TcpListener,
        TcpStream,
    },
    sync::{
        mpsc,
        Mutex,
    },
};
use tokio_util::codec::{
    FramedRead,
    LinesCodec,
};

async fn process_client_messages(
    stream: TcpStream,
    author: SocketAddr,
    tx: Tx,
    mut state: State,
) -> Result<()> {
    let (reader, writer) = stream.into_split();

    state.insert(author, writer).await;
    tx.send(Message::new(Event::Join, author))?;

    let mut reader = FramedRead::new(reader, LinesCodec::new());
    while let Some(result) = reader.next().await {
        match result {
            Ok(line) => tx.send(Message::new(Event::Message(line), author))?,
            Err(_) => break,
        }
    }

    state.remove(&author).await;
    tx.send(Message::new(Event::Leave, author))?;

    Ok(())
}

async fn handle_events(mut rx: Rx, state: State) -> Result<()> {
    while let Some(message) = rx.recv().await {
        let broadcast_text = match message.event {
            Event::Join => format!("{} has joined the chat.\n", message.author),
            Event::Leave => format!("{} has left the chat.\n", message.author),
            Event::Message(line) => format!("[{} - {}]\n{}\n", message.author, Local::now(), line),
        };

        state.broadcast(message.author, broadcast_text).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6000").await?;

    println!("Server started at 127.0.0.1:6000");

    let (tx, rx) = mpsc::unbounded_channel::<Message>();
    let state = Arc::new(Mutex::new(Shared::new()));

    tokio::spawn(handle_events(rx, state.clone()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let tx = tx.clone();

        tokio::spawn(process_client_messages(stream, addr, tx, state.clone()));
    }
}
