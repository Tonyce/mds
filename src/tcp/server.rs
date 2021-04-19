use crate::state::Shared;

use super::peer::Peer;
use super::protocol::Protocol;

use futures::SinkExt;
use std::error::Error;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_stream::StreamExt;
// use tokio::sync::Mutex;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio_util::codec::Framed;

pub async fn start_tcp_server(state: Arc<RwLock<Shared>>) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:2345").await?;

    loop {
        // Asynchronously wait for an inbound socket.
        let (mut socket, addr) = listener.accept().await?;
        println!("{}", addr);

        // And this is where much of the magic of this server happens. We
        // crucially want all clients to make progress concurrently, rather than
        // blocking one on completion of another. To achieve this we use the
        // `tokio::spawn` function to execute the work in the background.
        //
        // Essentially here we're executing a new task to run concurrently,
        // which will allow all of our clients to be processed concurrently.
        let state = state.clone();
        tokio::spawn(async move {
            if let Err(e) = process(state, socket, addr).await {
                eprintln!("an error occurred; error = {:?}", e);
            }
        });
    }
    // loop {
    //     // Asynchronously wait for an inbound TcpStream.
    //     let (stream, addr) = listener.accept().await?;

    //     // Clone a handle to the `Shared` state for the new connection.
    //     let state = Arc::clone(&state);

    //     // Spawn our handler to be run asynchronously.
    //     tokio::spawn(async move {
    //         tracing::debug!("accepted connection");
    //         if let Err(e) = process(state, stream, addr).await {
    //             tracing::info!("an error occurred; error = {:?}", e);
    //         }
    //     });
    // }
}

async fn process(
    state: Arc<RwLock<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut transport = Framed::new(stream, Protocol);

    let mut peer = Peer::new(state.clone(), transport).await?;

    // A client has connected, let's let everyone know.
    {
        let mut state = state.write().await;
        // let msg = format!("{} has joined the chat", username);
        // tracing::info!("{}", msg);
        // state.broadcast(addr, &msg).await;
    }

    // Process incoming messages until our stream is exhausted by a disconnect.
    loop {
        tokio::select! {
            Some(msg) = peer.rx.recv() => {
                peer.transport.send(msg).await?;
            }
            result = peer.transport.next() => match result {
                // A message was received from the current user, we should
                // broadcast this message to the other users.
                Some(Ok(msg)) => {
                    println!("msg {:?}", msg);
                    // let mut state = state.read().await;
                    // let msg = format!("{}: {}", username, msg);
                    peer.transport.send(msg).await?;
                    // state.broadcast(addr, &msg).await;
                }
                // An error occurred.
                Some(Err(e)) => {
                    // tracing::error!(
                    //     "an error occurred while processing messages for {}; error = {:?}",
                    //     username,
                    //     e
                    // );
                }
                // The stream has been exhausted.
                None => break,
            },
        }
    }

    // If this section is reached it means that the client was disconnected!
    // Let's let everyone still connected know about it.
    {
        let mut state = state.write().await;
        state.peers.remove(&addr);

        let msg = format!("{} has left", addr);
        println!("{}", msg)
        // state.broadcast(addr, &msg).await;
    }

    Ok(())
}
