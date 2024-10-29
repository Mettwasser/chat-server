use std::sync::Arc;

use message::Message;
use shared::Shared;
use tokio::sync::{
    mpsc::{
        UnboundedReceiver,
        UnboundedSender,
    },
    Mutex,
};

pub mod event;
pub mod message;
pub mod shared;

pub type Tx = UnboundedSender<Message>;
pub type Rx = UnboundedReceiver<Message>;
pub type Error = Box<dyn std::error::Error + Sync + Send>;
pub type Result<T> = std::result::Result<T, Error>;
pub type State = Arc<Mutex<Shared>>;
