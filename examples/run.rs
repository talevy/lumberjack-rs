extern crate lumberjack as lj;

extern crate tokio_service;
extern crate futures;
extern crate env_logger;

use tokio_service::Service;
use futures::{Async, Finished};
use std::io;

#[derive(Clone)]
struct Beat;

impl Service for Beat {
    type Request = lj::Request;
    type Response = lj::Response;
    type Error = io::Error;
    type Future = Finished<lj::Response, io::Error>;

    fn call(&self, _request: lj::Request) -> Self::Future {
        let resp = lj::Response::new(_request.events.len() as u32);
        futures::finished(resp)
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
