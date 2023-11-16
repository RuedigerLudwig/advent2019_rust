use super::computer_error::ComputerError;
use super::{instructions, Pointer};

pub enum StepResult {
    Continue,
    StopRunning,
}

pub struct State {
    memory: Vec<i64>,
    pointer: Pointer,
    running: bool,
}

impl State {
    pub fn new(memory: Vec<i64>) -> State {
        Self {
            memory,
            pointer: Pointer::default(),
            running: true,
        }
    }

    pub fn next_instruction(&mut self) -> Result<StepResult, ComputerError> {
        if !self.running {
            return Err(ComputerError::NotRunning);
        }

        let result = match self.get_next()? {
            1 => instructions::add(self),
            2 => instructions::mul(self),
            99 => instructions::stop(),
            op => Err(ComputerError::IllegalOperation(op)),
        };

        match result {
            Ok(StepResult::Continue) => Ok(StepResult::Continue),
            Ok(StepResult::StopRunning) => {
                self.running = false;
                Ok(StepResult::StopRunning)
            }
            Err(err) => {
                self.running = false;
                Err(err)
            }
        }
    }

    pub fn get_next(&mut self) -> Result<i64, ComputerError> {
        let pointer = self.pointer.get();
        if pointer <= self.memory.len() {
            self.pointer.inc();
            Ok(self.memory[pointer])
        } else {
            Err(ComputerError::NoMoreData)
        }
    }

    pub fn get_value(&self, addr: Pointer) -> Result<i64, ComputerError> {
        self.memory
            .get(addr.get())
            .copied()
            .ok_or(ComputerError::IllegalAddress(addr))
    }

    pub fn set_value(&mut self, addr: Pointer, value: i64) -> Result<(), ComputerError> {
        if self.memory.len() <= addr.get() {
            return Err(ComputerError::IllegalAddress(addr));
        }
        self.memory[addr.get()] = value;
        Ok(())
    }
}
