use crate::state::Shared;

use hyper::service::Service;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, HeaderMap, Request, Response, Server, Version};
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::Mutex;
use tokio::sync::RwLock;

type BoxPin<T> = Pin<Box<T>>;

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
        Box::pin(async move {
            println!("{}", state.read().await.counter);
            let res = Ok(Response::builder().body(Body::from("hello world")).unwrap());
            res
        })
        // } else {
        //     let res = Ok(Response::builder().body(Body::from("hello world")).unwrap());
        //     Box::pin(async { res })
        // }
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
