use crate::{Codec, TimeBase};

use bytes::Bytes;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StreamKind {
    Audio,
    Video,
    Subtitle,
}

#[derive(Debug, Clone)]
pub struct Stream {
    pub id: u32,
    pub index: usize,
    pub time_base: TimeBase,
    pub kind: StreamKind,
    pub codec: Codec,
    pub extradata: Option<Bytes>,
}
