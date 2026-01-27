use crate::Timestamp;

pub mod audio;
pub mod subtitle;
pub mod video;

pub use audio::AudioFrame;
pub use subtitle::SubtitleFrame;
pub use video::VideoFrame;

#[derive(Debug, Clone)]
pub enum FrameData {
    Video(VideoFrame),
    Audio(AudioFrame),
    Subtitle(SubtitleFrame),
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub stream_idx: usize,
    pub pts: Option<Timestamp>,
    pub data: FrameData,
}
