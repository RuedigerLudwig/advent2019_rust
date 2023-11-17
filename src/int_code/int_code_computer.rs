use super::{computer_error::ComputerError, state::State, Pointer, StepResult};
use itertools::Itertools;

pub struct IntCodeComputer {
    state: State,
}

impl IntCodeComputer {
    fn new(memory: Vec<i64>) -> Self {
        Self {
            state: State::new(memory),
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

    pub fn run(&mut self) -> Result<(), ComputerError> {
        if let Some(result) = self.next() {
            let _ = result?;
        }
        Ok(())
    }
}

impl Iterator for IntCodeComputer {
    type Item = Result<i64, ComputerError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.state.next_instruction() {
                // Intentionally blocking
                Ok(StepResult::Waiting) | Ok(StepResult::Continue) => {}
                Ok(StepResult::Output(value)) => return Some(Ok(value)),
                Ok(StepResult::Halted) => return None,
                Err(err) => return Some(Err(err)),
            }
        }
    }
}

pub struct ComputerFactory {
    data: Vec<i64>,
}

impl ComputerFactory {
    pub fn init(input: &str) -> Result<Self, ComputerError> {
        let data = input
            .split(',')
            .map(|byte| byte.trim().parse())
            .try_collect()?;
        Ok(Self { data })
    }

    pub fn build(&self) -> IntCodeComputer {
        IntCodeComputer::new(self.data.clone())
    }

    pub fn iter(&self) -> impl Iterator<Item = IntCodeComputer> + '_ {
        struct ComputerFactoryIterator<'a>(&'a ComputerFactory);

        impl<'a> Iterator for ComputerFactoryIterator<'a> {
            type Item = IntCodeComputer;

            fn next(&mut self) -> Option<Self::Item> {
                Some(self.0.build())
            }
        }
        ComputerFactoryIterator(self)
    }
}
