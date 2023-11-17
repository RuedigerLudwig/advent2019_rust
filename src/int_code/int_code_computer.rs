use super::{computer_error::ComputerError, state::State, ExternalStepResult, Pointer};
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

    pub fn run(&mut self) -> Result<Option<i64>, ComputerError> {
        loop {
            match self.state.next_instruction()? {
                ExternalStepResult::Continue => {}
                ExternalStepResult::Output(value) => return Ok(Some(value)),
                ExternalStepResult::Halted => return Ok(None),
            }
        }
    }

    pub fn get_value_at(&self, addr: Pointer) -> Result<i64, ComputerError> {
        self.state.get_value_at(addr)
    }

    pub fn set_address(&mut self, addr: Pointer, value: i64) -> Result<(), ComputerError> {
        self.state.set_value(addr, value)
    }

    pub fn push_input(&mut self, value: i64) {
        self.state.push_input(value);
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
