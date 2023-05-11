use tokio::{task, sync::broadcast};

mod interface;
pub use interface::{SpaceTraders, Credentials};

// Creates instance of iterface
pub struct InterfaceHandler {
    sender: broadcast::Sender,
    credentials: Credentials,
    // receiver: broadcast::Receiver,
    // interface: interface::SpaceTraders,
    // thread: task,
}

impl InterfaceHandler {
    pub fn new(credentials: Credentials) -> Self {
        let (sender, _receiver) = broadcast::channel(100);
        
        // let thread = task::spawn(async {
        //     self.spawn_interface_handler()
        // })

        InterfaceHandler {
            sender,
            credentials,
        }
    }

    pub fn spawn(&self, sender: broadcast::Sender) {
        loop {
            for i in sender.subscribr.rec() {
                if i.receiver == Broadcast::Interface {
                    println!("{:?}", i.message);
                }
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
pub struct Broadcast {
    pub receiver: BroadcastReceiver,
    pub message: Option<String>,
}
pub enum BroadcastReceiver {
    Caller,
    Interface,
}
