extern crate lumberjack as lj;

extern crate tokio_service;
extern crate futures;
extern crate env_logger;

use tokio_service::Service;
use futures::{Async, Empty};
use std::io;

#[derive(Clone)]
struct Beat;

impl Service for Beat {
    type Request = lj::Request;
    type Response = lj::Response;
    type Error = io::Error;
    type Future = Empty<lj::Response, io::Error>;

    fn call(&self, _request: lj::Request) -> Self::Future {
        let mut resp = lj::Response::new();
        //TODO(talevy) futures::empty()
    }

    fn poll_ready(&self) -> Async<()> {
        Async::Ready(())
    }
}

fn main() {
    drop(env_logger::init());
    let addr = "0.0.0.0:5044".parse().unwrap();
    lj::Server::new(addr).serve(Beat);
}
