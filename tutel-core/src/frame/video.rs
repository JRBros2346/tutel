use bytes::Bytes;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct VideoPixelFormat {
    pub id: Cow<'static, str>,
    pub planes: u8,
    pub bits_per_sample: u8,
    pub planar: bool,
    pub chroma_subsampling: Option<(u8, u8)>,
}

#[derive(Debug, Clone)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub format: VideoPixelFormat,
    pub keyframe: bool,
    pub data: Vec<Bytes>,
}
