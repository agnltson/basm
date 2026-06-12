use std::ops;
use std::fmt;

#[derive(Debug, Clone)]
pub struct u5(pub u8);

impl u5 {
    pub const MAX: u8 = 0x1F;
    pub const MIN: u8 = 0x00;
}

impl ops::Add<u5> for u5 {
    type Output = u5;
    fn add(self, other: u5) -> Self {
        u5(self.0 + other.0)
    }
}

impl ops::Sub<u5> for u5 {
    type Output = u5;
    fn sub(self, other: u5) -> Self {
        u5(self.0 - other.0)
    }
}

impl fmt::Display for u5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 & 0x1F)
    }
}

impl fmt::Binary for u5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.0 & 0x1F)
    }
}

#[derive(Debug, Clone)]
pub struct i26(pub i32);

impl i26 {
    pub const MAX: i32 = 0x01FFFFFFu32 as i32;
    pub const MIN: i32 = 0x81FFFFFFu32 as i32;
}

impl ops::Add<i26> for i26 {
    type Output = i26;
    fn add(self, other: i26) -> Self {
        i26(self.0 + other.0)
    }
}

impl ops::Sub<i26> for i26 {
    type Output = i26;
    fn sub(self, other: i26) -> Self {
        i26(self.0 - other.0)
    }
}

impl fmt::Display for i26 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 & 0x83FFFFFFu32 as i32)
    }
}

impl fmt::Binary for i26 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = self.0 & 0x80000000u32 as i32;
        write!(f, "{:b}", (self.0 | (sign >> 5)) & 0x01FFFFFFu32 as i32)
    }
}
