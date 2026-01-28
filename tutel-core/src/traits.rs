use crate::{Error, Frame, Packet, Stream};

pub trait Demuxer {
    fn streams(&self) -> &[Stream];
    fn read_packet(&mut self) -> Result<Option<Packet>, Error>;
}

pub trait Muxer {
    fn streams(&self) -> &[Stream];
    fn write(&mut self, packet: Packet) -> Result<(), Error>;
    fn finalize(&mut self) -> Result<(), Error>;
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
