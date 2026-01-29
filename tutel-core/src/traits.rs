use std::io::{Read, Write};

use crate::{Error, Frame, Packet, Stream};

pub trait Demuxer {
    fn streams(&self) -> &[Stream];
    fn read_packet(&mut self) -> Result<Option<Packet>, Error>;
}

pub trait DemuxerFactory {
    fn probe(data: &[u8]) -> bool;
    fn open(input: Box<dyn Read>) -> Result<Box<dyn Demuxer>, Error>;
}

pub trait Muxer {
    fn requires_seek() -> bool
    where
        Self: Sized;
    fn write(&mut self, packet: Packet) -> Result<(), Error>;
    fn finalize(&mut self) -> Result<(), Error>;
}

pub trait MuxerFactory {
    fn format_name(&self) -> &'static str;
    fn open(&self, output: Box<dyn Write>, streams: &[Stream]) -> Result<Box<dyn Muxer>, Error>;
}

pub trait Decoder {
    fn decode(&mut self, pkt: Packet) -> Result<Vec<Frame>, Error>;
    fn flush(&mut self) -> Result<Vec<Frame>, Error>;
}

pub trait Encoder {
    fn encode(&mut self, frame: Frame) -> Result<Vec<Packet>, Error>;
    fn flush(&mut self) -> Result<Vec<Packet>, Error>;
}

pub trait Transform {
    fn apply(&mut self, frame: Frame) -> Result<Frame, Error>;
    fn name(&self) -> &'static str;
}
