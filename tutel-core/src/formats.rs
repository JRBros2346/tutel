use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct AudioFormat {
    pub id: Cow<'static, str>,
    pub bits_per_sample: u8,
    pub planar: bool,
    pub is_float: bool,
}

#[derive(Debug, Clone)]
pub struct SubtitleFormat(pub Cow<'static, str>);

#[derive(Debug, Clone)]
pub struct VideoFormat {
    pub id: Cow<'static, str>,
    pub planes: u8,
    pub bits_per_sample: u8,
    pub planar: bool,
    pub chroma_subsampling: Option<(u8, u8)>,
}
