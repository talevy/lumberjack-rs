extern crate lumberjack as lj;

extern crate env_logger;
extern crate futures;
extern crate tokio_service;

use tokio_service::Service;
use futures::Finished;

use std::io;
use std::error::Error;
use std::process::exit;

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
}

fn main() {
    exit(match inner_main() {
        Ok(_) => 0,
        Err(err) => {
            println!("{}", err);
            1
        }
    })
}

fn inner_main() -> Result<(), Box<Error>> {
    env_logger::init()?;
    let addr = "0.0.0.0:5044".parse()?;
    Ok(lj::Server::new(addr).serve(|| Ok(Beat))?)
}
