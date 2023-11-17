use super::{computer_error::ComputerError, state::State, Pointer, StepResult};
use itertools::Itertools;

pub struct IntCodeComputer {
    state: State,
}

impl IntCodeComputer {
    fn new(memory: &[i64]) -> Self {
        Self {
            state: State::new(memory),
        }
    }

    pub fn get_value_at(&self, addr: Pointer) -> i64 {
        self.state.get_value_at(addr)
    }

    pub fn set_address(&mut self, addr: Pointer, value: i64) {
        self.state.set_value(addr, value)
    }

    pub fn push_input(&mut self, value: i64) {
        self.state.push_input(value);
    }

    pub fn run(&mut self) -> Result<(), ComputerError> {
        if let Some(result) = self.run_blocking().next() {
            let _ = result?;
        }
        Ok(())
    }

    pub fn run_blocking(&mut self) -> impl Iterator<Item = Result<i64, ComputerError>> + '_ {
        struct BlockingIter<'a>(&'a mut IntCodeComputer);

        impl<'a> Iterator for BlockingIter<'a> {
            type Item = Result<i64, ComputerError>;

            fn next(&mut self) -> Option<Self::Item> {
                loop {
                    match self.0.state.next_instruction() {
                        Ok(StepResult::Waiting) | Ok(StepResult::Continue) => {}
                        Ok(StepResult::Output(value)) => return Some(Ok(value)),
                        Ok(StepResult::Halted) => return None,
                        Err(err) => return Some(Err(err)),
                    }
                }
            }
        }

        BlockingIter(self)
    }
}

pub struct ComputerFactory {
    data: Vec<i64>,
}

impl ComputerFactory {
    pub fn new(data: Vec<i64>) -> Self {
        Self { data }
    }

    pub fn init(input: &str) -> Result<Self, ComputerError> {
        let data = input
            .split(',')
            .map(|byte| byte.trim().parse())
            .try_collect()?;
        Ok(Self::new(data))
    }

    pub fn build(&self) -> IntCodeComputer {
        IntCodeComputer::new(&self.data)
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
