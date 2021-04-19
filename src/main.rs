mod http;
mod state;
mod tcp;

use std::sync::Arc;
// use tokio::sync::Mutex;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    // TODO rwlock
    let state = Arc::new(RwLock::new(state::Shared::new()));

    let (http, tcp) = tokio::join!(
        http::start_http_server(state.clone()),
        tcp::server::start_tcp_server(state.clone())
    );

    if let Err(e) = http {
        eprintln!("http server error: {}", e);
    }
    if let Err(e) = tcp {
        eprintln!("tcp server error: {}", e);
    }
}
