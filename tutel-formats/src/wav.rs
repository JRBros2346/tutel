use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};
use tutel_core::*;

const RIFF: &[u8; 4] = b"RIFF";
const WAVE: &[u8; 4] = b"WAVE";
const FMT: &[u8; 4] = b"fmt ";
const DATA: &[u8; 4] = b"data";

pub struct WavDemuxer<R: Read + Seek> {
    reader: R,
    container_info: ContainerInfo,
    bytes_remaining: u64,
}

impl<R: Read + Seek> WavDemuxer<R> {
    pub fn new(mut reader: R) -> Result<Self> {
        let mut tag = [0u8; 4];
        reader.read_exact(&mut tag)?;
        if &tag != RIFF {
            return Err(Error::InvalidFormat);
        }
        let _riff_size = reader.read_u32::<LittleEndian>()?;
        reader.read_exact(&mut tag)?;
        if &tag != WAVE {
            return Err(Error::InvalidFormat);
        }
        reader.read_exact(&mut tag)?;
        let mut channels = 0u16;
        let mut sample_rate = 0u32;
        let mut bits_per_sample = 0u16;
        let mut data_size = 0u64;
        loop {
            if reader.read_exact(&mut tag).is_err() {
                break;
            }
            let chunk_size = reader.read_u32::<LittleEndian>()? as u64;
            match &tag {
                FMT => {
                    let audio_format = reader.read_u16::<LittleEndian>()?;
                    if audio_format != 1 {
                        return Err(Error::InvalidCodec);
                    }
                    channels = reader.read_u16::<LittleEndian>()?;
                    sample_rate = reader.read_u32::<LittleEndian>()?;
                    let _byte_rate = reader.read_u32::<LittleEndian>()?;
                    let _block_align = reader.read_u16::<LittleEndian>()?;
                    bits_per_sample = reader.read_u16::<LittleEndian>()?;
                    if chunk_size > 16 {
                        reader.seek(SeekFrom::Current((chunk_size - 16) as i64))?;
                    }
                }
                DATA => {
                    data_size = chunk_size;
                    break;
                }
                _ => {
                    reader.seek(std::io::SeekFrom::Current(chunk_size as i64))?;
                }
            }
        }
        if data_size == 0 {
            return Err(Error::InvalidFormat);
        }
        Ok(Self {
            reader,
            container_info: ContainerInfo {
                streams: vec![],
                attachments: vec![],
            },
            bytes_remaining: 0,
        })
    }
}
