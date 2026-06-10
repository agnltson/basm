use std::ops;
use std::fmt;

pub struct u5 {
    value: u8,
}

impl u5 {
    // Yes this is tedious but needed
    pub fn new(value: u8) -> Self {
        Self {
            value: value & 0x1F,
        }
    }
}

impl ops::Add<u5> for u5 {
    type Output = u5;
    fn add(self, other: u5) -> Self {
        Self {
            value: (self.value + other.value) & 0x1F,
        }
    }
}

impl ops::Sub<u5> for u5 {
    type Output = u5;
    fn sub(self, other: u5) -> Self {
        Self {
            value: (self.value - other.value) & 0x1F,
        }
    }
}

impl ops::Mul<u5> for u5 {
    type Output = u5;
    fn mul(self, other: u5) -> Self {
        Self {
            value: (self.value * other.value) & 0x1F,
        }
    }
}

impl ops::Div<u5> for u5 {
    type Output = u5;
    fn div(self, other: u5) -> Self {
        Self {
            value: (self.value / other.value) & 0x1F,
        }
    }
}

impl fmt::Display for u5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Binary for u5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:b}", self.value)
    }
}

pub struct i26 {
    value: i32,
}

impl i26 {
    // Yes this is tedious but needed
    pub fn new(value: i32) -> Self {
        Self {
            value: value,
        }
    }
}

impl ops::Add<i26> for i26 {
    type Output = i26;
    fn add(self, other: i26) -> Self {
        Self {
            value: self.value + other.value,
        }
    }
}

impl ops::Sub<i26> for i26 {
    type Output = i26;
    fn sub(self, other: i26) -> Self {
        Self {
            value: self.value - other.value,
        }
    }
}

impl ops::Mul<i26> for i26 {
    type Output = i26;
    fn mul(self, other: i26) -> Self {
        Self {
            value: self.value * other.value,
        }
    }
}

impl ops::Div<i26> for i26 {
    type Output = i26;
    fn div(self, other: i26) -> Self {
        Self {
            value: self.value / other.value,
        }
    }
}

impl fmt::Display for i26 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Binary for i26 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = self.value & 0x80000000u32 as i32;
        write!(f, "{:b}", (self.value | (sign >> 5)) & 0x01FFFFFFu32 as i32)
    }
}
