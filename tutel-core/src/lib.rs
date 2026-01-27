pub mod codec;
pub mod container;
pub mod error;
pub mod frame;
pub mod io;
pub mod packet;
pub mod time;

pub use bytes::Bytes;
pub use codec::CodecId;
pub use container::{Attachment, ContainerInfo, Stream, StreamKind};
pub use error::{Error, Result};
pub use frame::Frame;
pub use io::{
    InputAdapter, MediaIO, MediaInput, MediaOutput, OutputAdapter, SeekableInputAdapter,
    SeekableOutputAdapter,
};
pub use packet::Packet;
pub use time::{TimeBase, Timestamp};

pub trait Demuxer {
    fn container_info(&self) -> &ContainerInfo;
    fn read_packet(&mut self, input: &mut dyn MediaInput) -> Result<Option<Packet>>;
}

pub trait Muxer {
    fn write_packet(&mut self, output: &mut dyn MediaOutput, packet: Packet) -> Result<()>;
    fn finalize(&mut self, output: &mut dyn MediaOutput) -> Result<()>;
}

pub trait Decoder {
    fn send_packet(&mut self, pkt: Packet) -> Result<()>;
    fn receive_frame(&mut self) -> Result<Option<Frame>>;
    fn flush(&mut self) -> Result<()>;
}

pub trait Encoder {
    fn send_frame(&mut self, frame: Frame) -> Result<()>;
    fn receive_packet(&mut self) -> Result<Option<Packet>>;
    fn flush(&mut self) -> Result<()>;
}

pub trait Transform {
    const NAME: &'static str;
    fn apply(&mut self, frame: Frame) -> Result<Vec<Frame>>;
    fn flush(&mut self) -> Result<Vec<Frame>> {
        Ok(Vec::new())
    }
}
