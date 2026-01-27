use crate::Timestamp;

use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct Packet {
    pub stream_idx: usize,
    pub pts: Option<Timestamp>,
    pub dts: Option<Timestamp>,
    pub duration: Option<Timestamp>,
    pub keyframe: bool,
    pub discard: bool,
    pub data: Bytes,
}
