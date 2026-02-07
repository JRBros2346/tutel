pub mod codec;
pub mod container;
pub mod error;
pub mod frame;
pub mod packet;
pub mod time;

pub use bytes::Bytes;
pub use codec::CodecId;
pub use container::{Attachment, ContainerInfo, Stream, StreamKind};
pub use error::{Error, Result};
pub use frame::Frame;
pub use packet::Packet;
pub use time::{TimeBase, Timestamp};

pub trait Demuxer: Send + Sync {
    fn probe(&self, peek: &[u8]) -> u8;
    fn open(&self, input: ()) -> Result<Box<dyn DemuxSession>>;
}

pub trait DemuxSession {
    fn container_info(&self) -> &ContainerInfo;
    fn read_packet(&mut self) -> Result<Option<Packet>>;
}

pub trait Muxer {
    fn write(&mut self, packet: Packet) -> Result<()>;
    fn finalize(&mut self) -> Result<()>;
}

pub trait Decoder {
    fn decode(&mut self, pkt: Packet) -> Result<Vec<Frame>>;
    fn flush(&mut self) -> Result<Vec<Frame>>;
}

pub trait Encoder {
    fn encode(&mut self, frame: Frame) -> Result<Vec<Packet>>;
    fn flush(&mut self) -> Result<Vec<Packet>>;
}

pub trait Transform {
    fn apply(&mut self, frame: Frame) -> Result<Frame>;
    fn name(&self) -> &'static str;
}
