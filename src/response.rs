use std::fmt::{self, Write};

use byteorder::{BigEndian, WriteBytesExt};
use tokio_core::easy::Serialize;

pub struct Response {
    sequence: u32
}

pub struct Serializer;

impl Response {
    pub fn new() -> Self {
        Response {
            sequence: 0
        }
    }

    pub fn sequence(mut self, sequence: u32) -> Self {
        self.sequence = sequence;
        self
    }
}

impl Serialize for Serializer {
    type In = Response;

    fn serialize(&mut self, msg: Response, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&[b'2', b'A']);
        buf.write_u32::<BigEndian>(msg.sequence).unwrap();
    }
}


