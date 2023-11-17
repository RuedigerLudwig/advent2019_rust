use std::collections::VecDeque;

use super::computer_error::ComputerError;
use super::param_mode::ParamMode;
use super::{instructions, Pointer};

pub enum InternalStepResult {
    Continue,
    Output(i64),
    Waiting,
    Halted,
}

pub enum ExternalStepResult {
    Continue,
    Output(i64),
    Halted,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RunningState {
    Running,
    Waiting,
    Halted,
}

pub struct State {
    memory: Vec<i64>,
    pointer: Pointer,
    running: RunningState,
    input_buffer: VecDeque<i64>,
}

impl State {
    pub fn new(memory: Vec<i64>) -> State {
        Self {
            memory,
            pointer: Pointer::default(),
            running: RunningState::Running,
            input_buffer: VecDeque::new(),
        }
    }

    pub fn next_instruction(&mut self) -> Result<ExternalStepResult, ComputerError> {
        match self.running {
            RunningState::Running => {}
            RunningState::Waiting => {
                if self.input_buffer.is_empty() {
                    return Ok(ExternalStepResult::Continue);
                }
                self.running = RunningState::Running;
            }
            RunningState::Halted => return Err(ComputerError::NotRunning),
        }

        match instructions::run_instruction(self) {
            Ok(InternalStepResult::Continue) => Ok(ExternalStepResult::Continue),
            Ok(InternalStepResult::Waiting) => {
                self.running = RunningState::Waiting;
                Ok(ExternalStepResult::Continue)
            }
            Ok(InternalStepResult::Output(value)) => Ok(ExternalStepResult::Output(value)),
            Ok(InternalStepResult::Halted) => {
                self.running = RunningState::Halted;
                Ok(ExternalStepResult::Halted)
            }
            Err(err) => {
                self.running = RunningState::Halted;
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

    pub fn get_value(&mut self, pm: ParamMode) -> Result<i64, ComputerError> {
        let pointer = self.pointer.get();
        if pointer <= self.memory.len() {
            self.pointer.inc();
            match pm {
                ParamMode::Position => self.get_value_at(self.memory[pointer].try_into()?),
                ParamMode::Immediate => Ok(self.memory[pointer]),
                ParamMode::Illegal => Err(ComputerError::IllegalParamMode),
            }
        } else {
            Err(ComputerError::NoMoreData)
        }
    }

    pub fn get_address(&mut self, pm: ParamMode) -> Result<Pointer, ComputerError> {
        let pointer = self.pointer.get();
        if pointer <= self.memory.len() {
            self.pointer.inc();
            match pm {
                ParamMode::Position => self.memory[pointer].try_into(),
                ParamMode::Immediate | ParamMode::Illegal => Err(ComputerError::IllegalParamMode),
            }
        } else {
            Err(ComputerError::NoMoreData)
        }
    }

    pub fn get_value_at(&self, addr: Pointer) -> Result<i64, ComputerError> {
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

    #[inline]
    pub fn get_input(&mut self) -> Option<i64> {
        self.input_buffer.pop_front()
    }

    #[inline]
    pub fn push_input(&mut self, value: i64) {
        self.input_buffer.push_back(value);
    }

    pub fn repeat(&mut self) {
        self.pointer.dec();
    }

    pub fn set_pointer(&mut self, target: Pointer) {
        self.pointer = target
    }
}
