use std::{
    fs::File,
    io::{self, Cursor, Read, Seek, SeekFrom, Stdin, Stdout, Write},
};

pub trait MediaInput {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
    #[allow(unused_variables)]
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        Err(std::io::Error::new(io::ErrorKind::Unsupported, "seek"))
    }
    fn size(&mut self) -> Option<u64> {
        None
    }
    fn is_seekable(&self) -> bool {
        false
    }
}

pub trait MediaOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    #[allow(unused_variables)]
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        Err(std::io::Error::new(io::ErrorKind::Unsupported, "seek"))
    }
    fn is_seekable(&self) -> bool {
        false
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub trait MediaIO: MediaInput + MediaOutput {}
impl<T: MediaInput + MediaOutput> MediaIO for T {}

impl MediaInput for File {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Read::read(self, buf)
    }
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        Seek::seek(self, pos)
    }
    fn size(&mut self) -> Option<u64> {
        self.metadata().ok().map(|m| m.len())
    }
    fn is_seekable(&self) -> bool {
        true
    }
}
impl MediaOutput for File {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Write::write(self, buf)
    }
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        Seek::seek(self, pos)
    }
    fn flush(&mut self) -> io::Result<()> {
        Write::flush(self)
    }
    fn is_seekable(&self) -> bool {
        true
    }
}

impl MediaInput for Cursor<Vec<u8>> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Read::read(self, buf)
    }
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        Seek::seek(self, pos)
    }
    fn size(&mut self) -> Option<u64> {
        Some(self.get_ref().len() as u64)
    }
    fn is_seekable(&self) -> bool {
        true
    }
}

impl MediaOutput for Cursor<Vec<u8>> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Write::write(self, buf)
    }
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        Seek::seek(self, pos)
    }
    fn flush(&mut self) -> io::Result<()> {
        Write::flush(self)
    }
    fn is_seekable(&self) -> bool {
        true
    }
}

impl MediaInput for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        Read::read(self, buf)
    }
}

impl MediaOutput for Stdout {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Write::write(self, buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        Write::flush(self)
    }
}

pub struct InputAdapter<R: Read>(pub R);
impl<R: Read> MediaInput for InputAdapter<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}
impl<R: Read> From<R> for InputAdapter<R> {
    fn from(value: R) -> Self {
        Self(value)
    }
}

pub struct SeekableInputAdapter<R: Read + Seek>(pub R);
impl<R: Read + Seek> MediaInput for SeekableInputAdapter<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.0.seek(pos)
    }
    fn size(&mut self) -> Option<u64> {
        #[allow(clippy::seek_from_current)]
        let cur = self.0.seek(SeekFrom::Current(0)).ok()?;
        let end = self.0.seek(SeekFrom::End(0)).ok()?;
        let _ = self.0.seek(SeekFrom::Start(cur));
        Some(end)
    }
    fn is_seekable(&self) -> bool {
        true
    }
}
impl<R: Read + Seek> From<R> for SeekableInputAdapter<R> {
    fn from(value: R) -> Self {
        Self(value)
    }
}

pub struct OutputAdapter<W: Write>(pub W);
impl<W: Write> MediaOutput for OutputAdapter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
}
impl<W: Write> From<W> for OutputAdapter<W> {
    fn from(value: W) -> Self {
        Self(value)
    }
}

pub struct SeekableOutputAdapter<W: Write + Seek>(pub W);
impl<W: Write + Seek> MediaOutput for SeekableOutputAdapter<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        self.0.seek(pos)
    }
    fn is_seekable(&self) -> bool {
        true
    }
}
impl<W: Write + Seek> From<W> for SeekableOutputAdapter<W> {
    fn from(value: W) -> Self {
        Self(value)
    }
}
