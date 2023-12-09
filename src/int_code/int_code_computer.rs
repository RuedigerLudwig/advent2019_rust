use std::collections::VecDeque;

use super::{computer_error::ComputerError, state::State, Pointer, StepResult};
use itertools::{Either, Itertools};

pub struct IntCodeComputer {
    init_memory: Vec<i64>,
    state: State,
    peeked: VecDeque<i64>,
}

impl IntCodeComputer {
    fn new(memory: &[i64]) -> Self {
        Self {
            init_memory: Vec::from(memory),
            state: State::new(memory),
            peeked: VecDeque::new(),
        }
    }

    pub fn reset(&mut self) {
        self.state = State::new(&self.init_memory);
        self.peeked.clear();
    }

    fn run(&mut self) -> Result<Option<i64>, ComputerError> {
        loop {
            match self.state.next_instruction()? {
                StepResult::Continue => {}
                StepResult::Output(value) => return Ok(Some(value)),
                StepResult::Halted => return Ok(None),
                StepResult::Waiting => return Err(ComputerError::WaitingForInput),
            }
        }
    }

    pub fn get_memory_value(&self, addr: Pointer) -> i64 {
        self.state.get_value_at(addr)
    }

    pub fn manipulate_memory(&mut self, addr: Pointer, value: i64) {
        self.state.set_value(addr, value)
    }

    pub fn as_iter(&mut self) -> impl Iterator<Item = Result<i64, ComputerError>> + '_ {
        struct BlockingRunner<'b>(&'b mut IntCodeComputer);

        impl<'a> BlockingRunner<'a> {
            #[inline]
            pub fn new(computer: &'a mut IntCodeComputer) -> BlockingRunner<'a> {
                Self(computer)
            }
        }

        impl<'a> Iterator for BlockingRunner<'a> {
            type Item = Result<i64, ComputerError>;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.receive_next().transpose()
            }
        }
        BlockingRunner::new(self)
    }

    #[inline]
    pub fn run_till_halt(&mut self) -> Result<(), ComputerError> {
        while self.run()?.is_some() {}
        Ok(())
    }

    #[inline]
    pub fn send_i64(&mut self, value: i64) {
        self.state.push_input(value);
    }

    #[inline]
    pub fn send_bool(&mut self, input: bool) {
        self.send_i64(if input { 1 } else { 0 })
    }

    #[inline]
    pub fn send_char(&mut self, input: char) {
        self.send_i64(input as i64);
    }

    #[inline]
    pub fn send_string(&mut self, input: &str) {
        input.chars().for_each(|c| self.send_char(c));
        self.send_i64(10);
    }

    #[inline]
    fn receive_next(&mut self) -> Result<Option<i64>, ComputerError> {
        if let Some(peeked) = self.peeked.pop_front() {
            Ok(Some(peeked))
        } else {
            self.run()
        }
    }

    #[inline]
    pub fn expect_i64(&mut self) -> Result<i64, ComputerError> {
        if let Some(value) = self.receive_next()? {
            Ok(value)
        } else {
            Err(ComputerError::PrematureEndOfOutput)
        }
    }

    #[inline]
    pub fn maybe_i64(&mut self) -> Result<Option<i64>, ComputerError> {
        self.receive_next()
    }

    #[inline]
    pub fn expect_bool(&mut self) -> Result<bool, ComputerError> {
        if let Some(value) = self.receive_next()? {
            Ok(value != 0)
        } else {
            Err(ComputerError::PrematureEndOfOutput)
        }
    }

    #[inline]
    pub fn maybe_bool(&mut self) -> Result<Option<bool>, ComputerError> {
        Ok(self.receive_next()?.map(|value| value != 0))
    }

    #[inline]
    pub fn maybe_take_exactly(&mut self, n: usize) -> Result<Option<Vec<i64>>, ComputerError> {
        let result: Vec<i64> = self.as_iter().take(n).try_collect()?;
        if result.len() != n {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }

    fn push_peeked(&mut self, value: i64) {
        self.peeked.push_back(value);
    }

    pub fn maybe_string_or_i64(&mut self) -> Result<Option<Either<i64, String>>, ComputerError> {
        if let Some(string) = self.maybe_string()? {
            Ok(Some(Either::Right(string)))
        } else if !self.peeked.is_empty() {
            Ok(Some(Either::Left(self.expect_i64()?)))
        } else {
            Ok(None)
        }
    }

    pub fn maybe_string(&mut self) -> Result<Option<String>, ComputerError> {
        let mut string = String::new();
        let mut got_string_data = false;
        let mut peeked = None;

        for could_be_char in self.as_iter() {
            let c = could_be_char?;

            if c == 10 {
                got_string_data = true;
                break;
            }

            let mut got_correct_char = false;
            if let Some(ch) = char::from_u32(c as u32) {
                if ch.is_ascii() {
                    got_string_data = true;
                    got_correct_char = true;
                    string.push(ch);
                }
            }

            if !got_correct_char {
                if got_string_data {
                    return Err(ComputerError::NotAValidChar(c));
                } else {
                    peeked = Some(c);
                    break;
                }
            }
        }

        if let Some(peeked) = peeked {
            self.push_peeked(peeked);
            Ok(None)
        } else if !got_string_data {
            Ok(None)
        } else {
            Ok(Some(string))
        }
    }

    pub fn expect_string_(&mut self) -> Result<String, ComputerError> {
        if let Some(string) = self.maybe_string()? {
            Ok(string)
        } else {
            Err(ComputerError::PrematureEndOfOutput)
        }
    }
}

pub struct ComputerFactory {
    data: Vec<i64>,
}

impl ComputerFactory {
    #[inline]
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

    /**
     * Creates an IntCodeComputer.
     * This version must never wait for Input,
     * i.e. the Input must be pushed before it is requested by this IntCodeComputer
     * otherwise it will return an error
     */
    pub fn build(&self) -> IntCodeComputer {
        IntCodeComputer::new(&self.data)
    }
}
