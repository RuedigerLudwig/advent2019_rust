use super::computer_error::ComputerError;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
pub struct Pointer(usize);

impl std::fmt::Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Pointer {
    #[inline]
    pub fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub fn from_i64(addr: i64) -> Result<Self, ComputerError> {
        if !addr.is_negative() {
            Ok(Pointer(addr as usize))
        } else {
            Err(ComputerError::PointerMustNoBeNegative(addr))
        }
    }

    #[inline]
    pub fn inc(&mut self) {
        self.0 += 1;
    }

    #[inline]
    pub fn dec(&mut self) {
        self.0 -= 1;
    }
}

impl Add for Pointer {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}
