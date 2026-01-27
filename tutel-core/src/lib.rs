use bytes::Bytes;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("codec error: {0}")]
    Codec(String),
    #[error("format error: {0}")]
    Format(String),
}

#[derive(Clone, Debug)]
pub struct Packet {
    pub data: Bytes,
    pub stream_idx: usize,
    pub pts: Option<i64>,
    pub dts: Option<i64>,
    pub flags: PacketFlags,
}

bitflags::bitflags! {
    #[derive(Clone, Debug)]
    pub struct PacketFlags: u32 {
        const KEY = 0x1;
    }
}

#[derive(Debug)]
pub enum MediaFrame {
    Video(VideoFrame),
    Audio(AudioFrame),
    Subtitle(SubtitleFrame),
}

#[derive(Debug)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub format: PixelFormat,
    pub data: Vec<Bytes>,
    pub pts: Option<i64>,
}

#[derive(Debug)]
pub struct AudioFrame {
    pub sample_rate: u32,
    pub channels: u16,
    pub format: SampleFormat,
    pub samples: Vec<Bytes>,
    pub pts: Option<i64>,
}

#[derive(Debug)]
pub struct SubtitleFrame {
    pub text: String,
    pub pts: Option<i64>,
}

#[derive(Debug, Clone, Copy)]
pub enum PixelFormat {
    YUV420P,
    RGB24,
    NV12,
}

#[derive(Debug, Clone, Copy)]
pub enum SampleFormat {
    S16,
    FLT,
}

pub trait Demuxer {
    fn read_packet(&mut self) -> Result<Option<Packet>, Error>;
    fn stream_count(&self) -> usize;
    fn stream_info(&self, idx: usize) -> Option<StreamInfo>;
}

pub struct StreamInfo {
    pub kind: MediaKind,
    pub id: usize,
    pub codec_tag: Option<String>,
}

#[derive(Debug)]
pub enum MediaKind {
    Video,
    Audio,
    Subtitle,
}

pub trait Decoder: Send {
    fn decode(&mut self, pkt: Packet) -> Result<Vec<MediaFrame>, Error>;
    fn flush(&mut self) -> Result<Vec<MediaFrame>, Error>;
}

pub trait Encoder: Send {
    fn encode(&mut self, frame: MediaFrame) -> Result<Vec<Packet>, Error>;
    fn flush(&mut self) -> Result<Vec<Packet>, Error>;
}
