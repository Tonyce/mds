use crate::state::Shared;

use anyhow::{Error, Result};
use hyper::body;
use hyper::service::Service;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, HeaderMap, Request, Response, Server, Version};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::Mutex;
use tokio::sync::RwLock;

type BoxPin<T> = Pin<Box<T>>;

#[derive(Deserialize, Debug)]
struct ReqBody {
    #[serde(rename = "socketAddr")]
    socker_addr: String,
    msg: String,
}

struct Serv {
    state: Arc<RwLock<Shared>>,
}

impl Service<Request<Body>> for Serv {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = BoxPin<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, mut req: Request<Body>) -> Self::Future {
        let req_version = req.version();
        let state = self.state.clone();

        // if req_version == Version::HTTP_2 {
        // let u: ReqBody = serde_json::from_slice(j).unwrap();
        Box::pin(async move {
            let (_parts, body) = req.into_parts();
            // let body_buf = body::to_bytes(body).await.unwrap();
            // let body: ReqBody = serde_json::from_slice(&body_buf).unwrap();
            // let socket_addr: SocketAddr = body.socker_addr.parse().unwrap();
            let socket_addr = body::to_bytes(body)
                .await
                .map_err(Error::msg)
                .and_then(|body_buf| serde_json::from_slice(&body_buf).map_err(Error::msg))
                .and_then(|body: ReqBody| body.socker_addr.parse().map_err(Error::msg));
            match socket_addr {
                Ok(addr) => {
                    let state = state.write().await;
                    let p = state.peers.get(&addr);
                    if let Some(peer) = p {
                        peer.send(b"hello".to_vec()).unwrap();
                    }
                    let res = Ok(Response::builder().body(Body::from("hello world")).unwrap());
                    res
                }
                Err(e) => {
                    println!("e {:?}", e);
                    let res = Ok(Response::builder().body(Body::from("e")).unwrap());
                    res
                }
            }
        })
    }
}

struct MakeSvc {
    state: Arc<RwLock<Shared>>,
    // counter: Counter,
    // backends: Arc<Mutex<Backend>>,
    // client: Arc<Client<HttpConnector, Body>>
}

impl<T> Service<T> for MakeSvc {
    type Response = Serv;
    type Error = hyper::Error;
    type Future = BoxPin<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let state = self.state.clone();
        // let backends = self.backends.clone();
        let fut = async move { Ok(Serv { state }) };
        Box::pin(fut)
    }
}

pub async fn start_http_server(state: Arc<RwLock<Shared>>) -> Result<(), hyper::Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let service = MakeSvc { state };
    let server = Server::bind(&addr).serve(service);

    // let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        return Err(e);
    }
    Ok(())
}
