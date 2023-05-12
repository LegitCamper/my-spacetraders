use std::sync::Arc;
use tokio::{
    sync::broadcast,
    task,
    time::{sleep, Duration},
};

mod interface;
pub use interface::{AgentInfoL0, Credentials, SpaceTraders};

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
                sleep(Duration::from_millis(500)).await; // only 2 requests per second
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
    pub message: BroadcastMessage,
}
#[derive(Debug, Eq, PartialEq)]
pub enum BroadcastReceiver {
    Caller,
    Interface,
}
#[derive(Debug)]
pub enum BroadcastMessage {
    String(String),
    AgentInfo,
    AgentInfoResponse(AgentInfoL0),
    WaypointDetails(String, String),
}
