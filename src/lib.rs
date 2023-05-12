use std::sync::Arc;
use tokio::{sync::broadcast, task};

mod interface;
pub use interface::{Credentials, SpaceTraders};

// Creates instance of iterface
pub struct InterfaceHandler {
    pub sender: broadcast::Sender<Arc<Broadcast>>,
    pub credentials: Credentials,
}

impl InterfaceHandler {
    pub fn new(credentials: Credentials) -> Self {
        let (sender, _receiver) = broadcast::channel(100);

        InterfaceHandler {
            sender,
            credentials,
        }
    }

    pub async fn spawn(&self) {
        let mut receiver = self.sender.subscribe();
        task::spawn(async move {
            loop {
                let i = receiver.recv().await.unwrap();
                if i.receiver == BroadcastReceiver::Interface {
                    println!("{:?}", i.message);
                }
            }
        });
    }
}

// struct to handle the dataflow through broadcast
#[derive(Debug)]
pub struct Broadcast {
    pub receiver: BroadcastReceiver,
    pub message: Option<String>,
}
#[derive(Debug, Eq, PartialEq)]
pub enum BroadcastReceiver {
    Caller,
    Interface,
}
