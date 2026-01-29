#[derive(Debug, Clone, Copy)]
pub struct TimeBase(pub u32, pub u32);

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timestamp(pub i64);

impl TimeBase {
    pub const fn new(num: u32, den: u32) -> Self {
        assert!(num > 0 && den > 0);
        Self(num, den)
    }
    pub fn rescale(&self, ts: Timestamp, dst: TimeBase) -> Timestamp {
        let v = ts.0 as i128 * self.0 as i128 * dst.1 as i128 / (self.1 as i128 * dst.0 as i128);
        Timestamp(v.clamp(i64::MIN as i128, i64::MAX as i128) as i64)
    }
}

impl std::fmt::Display for TimeBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}
