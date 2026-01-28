use crate::{AudioFormat, SubtitleFormat, Timestamp, VideoFormat};

use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct AudioFrame {
    pub sample_rate: u32,
    pub channels: u16,
    pub format: AudioFormat,
    pub samples: Vec<Bytes>,
}

#[derive(Debug, Clone)]
pub struct SubtitleFrame {
    pub format: SubtitleFormat,
    pub data: Bytes,
}

#[derive(Debug, Clone)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub format: VideoFormat,
    pub keyframe: bool,
    pub data: Vec<Bytes>,
}

#[derive(Debug, Clone)]
pub enum FrameData {
    Video(VideoFrame),
    Audio(AudioFrame),
    Subtitle(SubtitleFrame),
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub pts: Timestamp,
    pub dts: Option<Timestamp>,
    pub stream_id: u32,
    pub data: FrameData,
}
