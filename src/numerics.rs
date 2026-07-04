use std::ops;
use std::fmt;

#[derive(Debug, Clone)]
pub struct BeltIdx(pub u8);

impl BeltIdx {
    pub const MAX: u8 = 0x1F;
    pub const MIN: u8 = 0x00;
}

impl ops::Add<BeltIdx> for BeltIdx {
    type Output = BeltIdx;
    fn add(self, other: BeltIdx) -> Self {
        BeltIdx(self.0 + other.0)
    }
}

impl ops::Sub<BeltIdx> for BeltIdx {
    type Output = BeltIdx;
    fn sub(self, other: BeltIdx) -> Self {
        BeltIdx(self.0 - other.0)
    }
}

impl fmt::Display for BeltIdx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 & 0x1F)
    }
}

impl fmt::Binary for BeltIdx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0 & 0x1F)
    }
}

#[derive(Debug, Clone)]
pub struct Immediate(pub i32);

impl Immediate {
    pub const MAX: i32 = 0x01FFFFFFu32 as i32;
    pub const MIN: i32 = 0xFE000000u32 as i32;
}

impl ops::Add<Immediate> for Immediate {
    type Output = Immediate;
    fn add(self, other: Immediate) -> Self {
        Immediate(self.0 + other.0)
    }
}

impl ops::Sub<Immediate> for Immediate {
    type Output = Immediate;
    fn sub(self, other: Immediate) -> Self {
        Immediate(self.0 - other.0)
    }
}

impl fmt::Display for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Binary for Immediate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:026b}", self.0 & 0x03FFFFFFu32 as i32)
    }
}
