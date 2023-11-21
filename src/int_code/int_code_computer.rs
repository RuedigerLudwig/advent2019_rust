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
        struct BlockingRunner<'a>(&'a mut IntCodeComputer);

        impl<'a> BlockingRunner<'a> {
            #[inline]
            pub fn new(computer: &'a mut IntCodeComputer) -> BlockingRunner<'a> {
                Self(computer)
            }
        }

        impl<'a> Iterator for BlockingRunner<'a> {
            type Item = Result<i64, ComputerError>;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.run().transpose()
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
    pub fn expect_i64(&mut self) -> Result<i64, ComputerError> {
        if let Some(value) = self.run()? {
            Ok(value)
        } else {
            Err(ComputerError::PrematureEndOfOutput)
        }
    }

    #[inline]
    pub fn maybe_i64(&mut self) -> Result<Option<i64>, ComputerError> {
        self.run()
    }

    #[inline]
    pub fn expect_bool(&mut self) -> Result<bool, ComputerError> {
        if let Some(value) = self.run()? {
            Ok(value != 0)
        } else {
            Err(ComputerError::PrematureEndOfOutput)
        }
    }

    #[inline]
    pub fn maybe_bool(&mut self) -> Result<Option<bool>, ComputerError> {
        Ok(self.run()?.map(|value| value != 0))
    }

    #[inline]
    pub fn maybe_take_exacltly(&mut self, n: usize) -> Result<Option<Vec<i64>>, ComputerError> {
        let result: Vec<i64> = self.as_iter().take(n).try_collect()?;
        if result.len() != n {
            Ok(None)
        } else {
            Ok(Some(result))
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
