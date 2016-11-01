use std::io;

use protocol::{read_batch, Event};

use futures::{Poll, Async};
use tokio_core::easy::{EasyBuf, Parse};


pub struct Request {
    pub events: Vec<Event>
}

pub struct MyParser;

impl Parse for MyParser {
    type Out = Request;

    fn parse(&mut self, buf: &mut EasyBuf) -> Poll<Request, io::Error> {
        let len = buf.len();
        if len == 0 {
            Ok(Async::NotReady)
        } else {
            let events = read_batch(buf.as_slice());
            buf.drain_to(len);
            Ok(Async::Ready(Request { events: events }))
        }
    }
}
