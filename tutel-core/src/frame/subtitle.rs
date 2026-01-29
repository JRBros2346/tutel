use bytes::Bytes;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct SubtitleFormat(pub Cow<'static, str>);

#[derive(Debug, Clone)]
pub struct SubtitleFrame {
    pub format: SubtitleFormat,
    pub data: Bytes,
}
