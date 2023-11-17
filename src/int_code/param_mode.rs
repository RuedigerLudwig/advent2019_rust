use std::cell::Cell;

pub enum ParamMode {
    Position,
    Immediate,
    Illegal,
}

impl From<usize> for ParamMode {
    fn from(value: usize) -> Self {
        match value {
            0 => ParamMode::Position,
            1 => ParamMode::Immediate,
            _ => ParamMode::Illegal,
        }
    }
}

#[derive(Debug)]
pub struct ParamModeDispenser(Cell<usize>);

impl ParamModeDispenser {
    #[inline]
    pub fn new(modes: usize) -> Self {
        Self(Cell::new(modes))
    }

    #[inline]
    pub fn next(&self) -> ParamMode {
        let old = self.0.get();
        self.0.set(old / 10);
        (old % 10).into()
    }
}
