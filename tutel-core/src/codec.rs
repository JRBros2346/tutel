use std::fmt;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CodecId(u32);

impl CodecId {
    /// # Safety
    /// The bytes must be valid ASCII characters.
    pub const unsafe fn from_bytes_unchecked(bytes: [u8; 4]) -> Self {
        Self(u32::from_be_bytes(bytes))
    }
    pub const fn as_bytes(self) -> [u8; 4] {
        self.0.to_be_bytes()
    }
}

impl std::str::FromStr for CodecId {
    type Err = CodecIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; 4] = s
            .as_bytes()
            .try_into()
            .map_err(|_| CodecIdError::InvalidLength)?;

        Self::try_from(bytes)
    }
}

impl TryFrom<&str> for CodecId {
    type Error = CodecIdError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl TryFrom<[u8; 4]> for CodecId {
    type Error = CodecIdError;

    fn try_from(bytes: [u8; 4]) -> Result<Self, Self::Error> {
        if !bytes.iter().all(|b| b.is_ascii_graphic()) {
            return Err(CodecIdError::NonPrintableAscii);
        }
        Ok(Self(u32::from_be_bytes(bytes)))
    }
}

impl fmt::Debug for CodecId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.as_bytes();
        if let Ok(s) = std::str::from_utf8(&bytes)
            && s.chars().all(|c| !c.is_control())
        {
            return write!(f, "\"{s}\"");
        }
        write!(f, "{bytes:?}")
    }
}

impl fmt::Display for CodecId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.as_bytes();
        if let Ok(s) = std::str::from_utf8(&bytes)
            && s.chars().all(|c| !c.is_control())
        {
            return write!(f, "{s}");
        }
        write!(
            f,
            "0x{:02X}{:02X}{:02X}{:02X}",
            bytes[0], bytes[1], bytes[2], bytes[3]
        )
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CodecIdError {
    #[error("codec id must be exactly 4 bytes")]
    InvalidLength,
    #[error("codec id must contain printable ASCII characters")]
    NonPrintableAscii,
}
