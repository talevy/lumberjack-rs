extern crate byteorder;
extern crate combine;
extern crate flate2;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use byteorder::{BigEndian, ByteOrder};
use combine::{parser, Parser, ParseResult};
use combine::byte::byte;
use combine::range::take;
use flate2::read::ZlibDecoder;


const CODE_JSON_EVENT: u8 = b'J';
const CODE_COMPRESSED: u8 = b'C';
const CODE_WINDOW_SIZE: u8 = b'W';
const PROTO_VERSION: u8 = b'2';

fn read_byte(input: &[u8]) -> ParseResult<u8, &[u8]> { take(1).map(|x: &[u8]| x[0]).parse_stream(input) }
fn count(input: &[u8]) -> ParseResult<u32, &[u8]> { take(4).map(BigEndian::read_u32).parse_stream(input) }

fn extract(input: &[u8]) -> Vec<u8> {
    let mut buf = Vec::new();
    let mut d = ZlibDecoder::new(input);
    d.read_to_end(&mut buf).unwrap();
    buf
}

fn read_events(input: &[u8]) {
    let (code, rest) = byte(PROTO_VERSION).with(parser(read_byte)).parse(input).unwrap();
    match code {
        CODE_JSON_EVENT => {
            let ((_, event), _) = (parser(count), parser(count).then(|size| take(size as usize))).parse(rest).unwrap();
            println!("{:?}", String::from_utf8_lossy(event));
        },
        CODE_COMPRESSED => {
            let (uncompressed, _) = parser(count).then(|size| take(size as usize)).map(extract).parse(rest).unwrap();
            read_events(uncompressed.as_slice());
        },
        _ => ()
    }
}

fn read_batch(data: &[u8]) {
    let mut read_batch_header = byte(PROTO_VERSION).with(byte(CODE_WINDOW_SIZE)).with(parser(count));
    let (num_frames, rest) = read_batch_header.parse(data).unwrap();
    for _ in 0..num_frames {
        read_events(rest);
    }
}

#[test]
fn test_extract_success() {
    let mut data = Vec::new();
    File::open(&Path::new(&"tests/data"))
        .and_then(|mut file| file.read_to_end(&mut data))
        .unwrap();
    read_batch(data.as_slice());
}
