use super::computer_error::ComputerError;

#[derive(Debug, PartialEq, Eq, Default)]
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

    #[inline]
    pub fn get(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}

impl TryFrom<i64> for Pointer {
    type Error = ComputerError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if !value.is_negative() {
            Ok(Pointer(value as usize))
        } else {
            Err(ComputerError::IllegalPointerI64(value))
        }
    }
}
