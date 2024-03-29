use byteorder::{BigEndian, ByteOrder};
use bytes::{Buf, BufMut, BytesMut};
use futures::SinkExt;
use prost::Message;
use std::{error::Error, fmt, fs, io, usize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixListener;
use tokio::signal;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};

pub struct Protocol;

impl Protocol {
    fn new() -> Self {
        Protocol
    }
}

impl Decoder for Protocol {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Vec<u8>>> {
        // println!("buf_len:{:?} is_empty:{}", buf.len(), buf.is_empty());
        if buf.is_empty() {
            return Ok(None);
        }
        if buf.len() > 4 {
            let proto_len_buf = &buf[0..4];
            let body_len = BigEndian::read_uint(&proto_len_buf.to_vec(), 4);
            if buf.len() < body_len as usize {
                return Ok(None);
            }
            let body = buf.split_to((body_len + 4) as usize);
            Ok(Some(body[4..].to_vec()))
        } else {
            Ok(None)
        }
    }

    fn decode_eof(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match self.decode(buf)? {
            Some(frame) => Ok(Some(frame)),
            None => {
                if buf.is_empty() {
                    Ok(None)
                } else {
                    Err(io::Error::new(
                        io::ErrorKind::Other,
                        "bytes remaining on stream",
                    ))
                }
            }
        }
    }

    fn framed<T: tokio::io::AsyncRead + tokio::io::AsyncWrite + Sized>(
        self,
        io: T,
    ) -> Framed<T, Self>
    where
        Self: Sized,
    {
        Framed::new(io, self)
    }
}

impl Encoder<Vec<u8>> for Protocol {
    type Error = io::Error;

    fn encode(&mut self, item: Vec<u8>, dst: &mut BytesMut) -> io::Result<()> {
        // println!("{:?}", item);
        dst.put(&item[..]);
        Ok(())
    }
}
