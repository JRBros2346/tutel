use tutel_core::*;

use tutel_formats::wav::WavDemuxer;

pub fn create_demuxer(input: &mut dyn MediaInput) -> Result<Box<dyn Demuxer>> {
    input.seek(std::io::SeekFrom::Start(0))?;
    Err(Error::InvalidFormat)
}

pub fn create_decoder(codec: CodecId) -> Result<Box<dyn Decoder>> {
    Err(Error::InvalidCodec)
}
