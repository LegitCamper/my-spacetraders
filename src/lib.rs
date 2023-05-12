use std::sync::Arc;
use tokio::{sync::broadcast, task};

mod interface;
pub use interface::{Credentials, SpaceTraders};

// Creates instance of iterface
pub struct InterfaceHandler {
    pub sender: broadcast::Sender<Arc<Broadcast>>,
    pub credentials: Credentials,
    // receiver: broadcast::Receiver,
    // interface: interface::SpaceTraders,
    // thread: task,
}

impl InterfaceHandler {
    pub fn new(credentials: Credentials) -> Self {
        let (sender, _receiver) = broadcast::channel(100);

        InterfaceHandler {
            sender,
            credentials,
        }
    }

    pub async fn spawn(&self, sender: broadcast::Sender<Arc<Broadcast>>) {
        let mut receiver = sender.subscribe();
        loop {
            let i = receiver.recv().await.unwrap();
            println!("{:?}", i);
            if i.receiver == BroadcastReceiver::Interface {
                println!("{:?}", i.message);
            }
        }
    }
}

// fn spawn_interface_handler(credentials: Credentials, receiver: broadcast::Receiver, sender: broadcast::Sender) {
//     let space_traders = SpaceTraders::new(credentials);

//     loop {
//         for i in receiver {
//             if i.receiver == Broadcast::Interface {
//                 println!("{:?}", i.message);
//             }
//         }
//     }
// }

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
