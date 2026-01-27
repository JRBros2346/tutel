use bytes::Bytes;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct AudioSampleFormat {
    pub id: Cow<'static, str>,
    pub bits_per_sample: u8,
    pub planar: bool,
    pub is_float: bool,
}

#[derive(Debug, Clone)]
pub struct AudioFrame {
    pub sample_rate: u32,
    pub channels: u16,
    pub format: AudioSampleFormat,
    pub samples: Vec<Bytes>,
}
