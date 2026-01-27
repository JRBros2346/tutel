use tutel_core::*;

use crate::registry;

pub struct Pipeline {
    demuxer: Box<dyn Demuxer>,
    decoders: Vec<Option<Box<dyn Decoder>>>,
}

impl Pipeline {
    pub fn new(input: &mut dyn MediaInput) -> Result<Self> {
        let demuxer = registry::create_demuxer(input)?;
        let mut decoders = Vec::new();
        for stream in &demuxer.container_info().streams {
            let decoder = registry::create_decoder(stream.codec)?;
            decoders.push(Some(decoder));
        }
        Ok(Self { demuxer, decoders })
    }
    pub fn run(&mut self, input: &mut dyn MediaInput) -> Result<()> {
        while let Some(packet) = self.demuxer.read_packet(input)? {
            self.process_packet(packet)?;
        }
        for decoder in self.decoders.iter_mut().flatten() {
            decoder.flush()?;
            while let Some(frame) = decoder.receive_frame()? {
                println!("Flushed frame: {frame:?}");
            }
        }

        Ok(())
    }
    fn process_packet(&mut self, packet: Packet) -> Result<()> {
        if let Some(Some(decoder)) = self.decoders.get_mut(packet.stream_idx) {
            decoder.send_packet(packet)?;
            while let Some(frame) = decoder.receive_frame()? {
                println!("Decoded frame: {frame:?}")
            }
        }
        Ok(())
    }
}
