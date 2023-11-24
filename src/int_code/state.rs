use super::computer_error::ComputerError;
use super::param_mode::ParamMode;
use super::{instructions, Pointer};
use std::collections::{HashMap, VecDeque};

pub enum StepResult {
    Continue,
    Output(i64),
    Waiting,
    Halted,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RunningState {
    Running,
    Waiting,
    Error,
}

pub struct State {
    memory: HashMap<Pointer, i64>,
    pointer: Pointer,
    relative_base: i64,
    running: RunningState,
    input_buffer: VecDeque<i64>,
}

impl State {
    pub fn new(memory: &[i64]) -> State {
        let memory = memory
            .iter()
            .copied()
            .enumerate()
            .map(|(p, data)| (Pointer::new(p), data))
            .collect();

        Self {
            memory,
            pointer: Pointer::default(),
            relative_base: 0,
            running: RunningState::Running,
            input_buffer: VecDeque::new(),
        }
    }

    pub fn next_instruction(&mut self) -> Result<StepResult, ComputerError> {
        match self.running {
            RunningState::Running => {}
            RunningState::Waiting => {
                if self.input_buffer.is_empty() {
                    return Ok(StepResult::Waiting);
                }
                self.running = RunningState::Running;
            }
            RunningState::Error => return Err(ComputerError::StoppedAfterError),
        }

        match instructions::run_instruction(self) {
            Ok(StepResult::Continue) => Ok(StepResult::Continue),
            Ok(StepResult::Waiting) => {
                self.running = RunningState::Waiting;
                Ok(StepResult::Waiting)
            }
            Ok(StepResult::Output(value)) => Ok(StepResult::Output(value)),
            Ok(StepResult::Halted) => Ok(StepResult::Halted),
            Err(err) => {
                self.running = RunningState::Error;
                Err(err)
            }
        }
    }

    #[inline]
    pub fn get_value_at(&self, pointer: Pointer) -> i64 {
        self.memory.get(&pointer).copied().unwrap_or_default()
    }

    pub fn get_next(&mut self) -> i64 {
        let value = self.get_value_at(self.pointer);
        self.pointer.inc();
        value
    }

    pub fn get_value(&mut self, pm: ParamMode) -> Result<i64, ComputerError> {
        let value = self.get_next();
        match pm {
            ParamMode::Position => Ok(self.get_value_at(Pointer::from_i64(value)?)),
            ParamMode::Relative => {
                Ok(self.get_value_at(Pointer::from_i64(self.relative_base + value)?))
            }
            ParamMode::Immediate => Ok(value),
            ParamMode::Illegal => Err(ComputerError::IllegalParamMode),
        }
    }

    #[inline]
    pub fn get_address(&mut self, pm: ParamMode) -> Result<Pointer, ComputerError> {
        let value = self.get_next();
        match pm {
            ParamMode::Position => Pointer::from_i64(value),
            ParamMode::Relative => Pointer::from_i64(self.relative_base + value),
            ParamMode::Immediate | ParamMode::Illegal => Err(ComputerError::IllegalParamMode),
        }
    }

    pub fn set_value(&mut self, addr: Pointer, value: i64) {
        self.memory.insert(addr, value);
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

    pub fn adjust_relative_base(&mut self, relative_base: i64) {
        self.relative_base += relative_base
    }
}
