use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc;

/// Shorthand for the transmit half of the message channel.
pub type Tx = mpsc::UnboundedSender<Vec<u8>>;

/// Shorthand for the receive half of the message channel.
pub type Rx = mpsc::UnboundedReceiver<Vec<u8>>;

pub struct Shared {
    pub peers: HashMap<SocketAddr, Tx>,
    pub counter: usize,
}

impl Shared {
    /// Create a new, empty, instance of `Shared`.
    pub fn new() -> Self {
        Shared {
            peers: HashMap::new(),
            counter: 1,
        }
    }

    // Send a `LineCodec` encoded message to every peer, except
    // for the sender.
    // async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
    //     for peer in self.peers.iter_mut() {
    //         if *peer.0 != sender {
    //             let _ = peer.1.send(message.into());
    //         }
    //     }
    // }
}
