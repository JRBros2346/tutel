use crate::{CodecId, TimeBase};

use bytes::Bytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamKind {
    Audio,
    Data,
    Subtitle,
    Video,
}

#[derive(Debug, Clone)]
pub struct Stream {
    pub id: u32,
    pub index: usize,
    pub time_base: TimeBase,
    pub kind: StreamKind,
    pub codec: CodecId,
    pub extradata: Option<Bytes>,
}

pub struct Attachment {
    pub id: u32,
    pub name: Option<String>,
    pub mime_type: Option<String>,
    pub data: Bytes,
}

pub struct ContainerInfo {
    pub streams: Vec<Stream>,
    pub attachments: Vec<Attachment>,
}
