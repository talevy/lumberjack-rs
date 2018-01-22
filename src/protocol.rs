use std::io::Read;
use std::io;
use byteorder::{BigEndian, ByteOrder};
use combine::{many, parser, ParseResult, Parser};
use combine::combinator::FnParser;
use combine::byte::byte;
use combine::range::take;
use combine::primitives::RangeStream;
use flate2::read::ZlibDecoder;

macro_rules! parser {
    ($name: ident, $return_type: ty, $e:expr) => {
        fn $name<'a, I>() -> FnParser<I, fn(I) -> ParseResult<$return_type, I>>
            where I: RangeStream<Item = u8, Range = &'a [u8]>
            {
                fn _event<'a, I>(input: I) -> ParseResult<$return_type, I>
                    where I: RangeStream<Item = u8, Range = &'a [u8]>
                    {
                        $e.parse_stream(input)
                    }
                parser(_event)
            }

    }
}

const CODE_JSON_EVENT: u8 = b'J';
const CODE_COMPRESSED: u8 = b'C';
const CODE_WINDOW_SIZE: u8 = b'W';
const PROTO_VERSION: u8 = b'2';

pub struct Event {
    pub sequence: usize,
    pub raw: String,
}

impl Event {
    pub fn new(seq: usize, raw: &[u8]) -> Self {
        Event {
            sequence: seq,
            raw: String::from_utf8_lossy(raw).into_owned(),
        }
    }
}

pub fn read_batch(data: &[u8]) -> Result<Vec<Event>, io::Error> {
    byte(PROTO_VERSION)
        .with(byte(CODE_WINDOW_SIZE))
        .with(any_num())
        .with(compressed_block())
        .parse(data)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse batch"))
        .and_then(|(e, _)| {
            many(event_block())
                .parse(e.as_slice())
                .map(|v| v.0)
                .map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidData, "Failed to parse event block")
                })
        })
}

parser! {
    any_num, usize,
    take(4).map(BigEndian::read_u32).map(|x| x as usize)
}

parser! {
    event_block, Event,
    byte(PROTO_VERSION).with(byte(CODE_JSON_EVENT)).with((any_num(), any_num().then(take)))
        .map(|(seq, raw)| Event::new(seq, raw))
}

parser! {
    compressed_block, Vec<u8>,
    byte(PROTO_VERSION).with(byte(CODE_COMPRESSED)).with(any_num()).then(take).and_then(extract)
}

fn extract(input: &[u8]) -> Result<Vec<u8>, io::Error> {
    let mut buf = Vec::new();
    let mut d = ZlibDecoder::new(input);
    d.read_to_end(&mut buf)?;
    Ok(buf)
}
