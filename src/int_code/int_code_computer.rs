use super::{computer_error::ComputerError, state::State, Pointer, StepResult};
use itertools::Itertools;
use std::str::FromStr;

pub struct IntCodeComputer {
    state: State,
}

impl IntCodeComputer {
    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            state: State::new(memory),
        }
    }

    pub fn run(&mut self) -> Result<(), ComputerError> {
        while matches!(self.state.next_instruction()?, StepResult::Continue) {}
        Ok(())
    }

    pub fn get_address(&self, addr: Pointer) -> Result<i64, ComputerError> {
        self.state.get_value(addr)
    }

    pub fn set_address(&mut self, addr: Pointer, value: i64) -> Result<(), ComputerError> {
        self.state.set_value(addr, value)
    }
}

impl FromStr for IntCodeComputer {
    type Err = ComputerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(
            s.split(',').map(|byte| byte.parse()).try_collect()?,
        ))
    }
}
