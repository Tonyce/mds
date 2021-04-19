use crate::state::{Rx, Shared, Tx};

use super::protocol::Protocol;

use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use tokio_util::codec::Framed;

use std::io;
use std::sync::Arc;

/// The state for each connected client.
pub struct Peer {
    /// The TCP socket wrapped with the `Lines` codec, defined below.
    ///
    /// This handles sending and receiving data on the socket. When using
    /// `Lines`, we can work at the line level instead of having to manage the
    /// raw byte operations.
    pub lines: Framed<TcpStream, Protocol>,

    /// Receive half of the message channel.
    ///
    /// This is used to receive messages from peers. When a message is received
    /// off of this `Rx`, it will be written to the socket.
    pub rx: Rx,
}

impl Peer {
    /// Create a new instance of `Peer`.
    pub async fn new(
        state: Arc<RwLock<Shared>>,
        lines: Framed<TcpStream, Protocol>,
    ) -> io::Result<Peer> {
        // Get the client socket address
        let addr = lines.get_ref().peer_addr()?;

        // Create a channel for this peer
        let (tx, rx) = mpsc::unbounded_channel();

        // Add an entry for this `Peer` in the shared state map.
        state.write().await.peers.insert(addr, tx);

        Ok(Peer { lines, rx })
    }
}
