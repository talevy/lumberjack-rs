use request;
use response;

use std::io;
use std::net::SocketAddr;

use tokio_io::codec::{Decoder, Encoder, Framed};
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_proto::pipeline::ServerProto;
use tokio_proto::TcpServer;
use tokio_service::NewService;

use bytes::BytesMut;
use byteorder::{BigEndian, WriteBytesExt};

use protocol::read_batch;

pub struct Server {
    addr: SocketAddr,
    threads: usize,
}

impl Server {
    pub fn new(addr: SocketAddr) -> Self {
        Server {
            addr: addr,
            threads: 1,
        }
    }

    pub fn addr(mut self, addr: SocketAddr) -> Self {
        self.addr = addr;
        self
    }

    pub fn threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }

    pub fn serve<T>(self, service: T) -> Result<(), io::Error>
    where
        T: NewService<Request = request::Request, Response = response::Response, Error = io::Error>
            + 'static
            + Send
            + Sync,
    {
        let mut server = TcpServer::new(LumberjackProto, self.addr);
        server.threads(self.threads);
        Ok(server.serve(service))
    }
}

struct LumberjackCodec;

impl Decoder for LumberjackCodec {
    type Item = request::Request;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let len = src.len();
        if len == 0 {
            Ok(None)
        } else {
            let events = read_batch(src.as_ref())?;
            src.split_to(len);
            Ok(Some(request::Request { events: events }))
        }
    }
}

impl Encoder for LumberjackCodec {
    type Item = response::Response;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.extend_from_slice(&[b'2', b'A']);
        dst.as_mut().write_u32::<BigEndian>(item.sequence())?;
        Ok(())
    }
}

struct LumberjackProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LumberjackProto {
    type Request = request::Request;
    type Response = response::Response;
    type Transport = Framed<T, LumberjackCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LumberjackCodec))
    }
}
