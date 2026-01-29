use tutel_core::*;

pub type DynDemuxer = Box<dyn Demuxer>;
pub type DynDecoder = Box<dyn Decoder>;
pub type DynMuxer = Box<dyn Muxer>;
pub type DynTransform = Box<dyn Transform>;
