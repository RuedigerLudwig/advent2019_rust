use super::{DayTrait, DayType, RResult};
use crate::int_code::{ComputerError, ComputerFactory, IntCodeComputer};
use itertools::{Either, Itertools};
use std::fmt::Display;

const DAY_NUMBER: DayType = 21;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let mut droid = SpringDroid::create(input, 'D', "WALK")?;

        let instructions = [
            (Instruction::Not, Read::Distance('A'), Write::Jump),
            (Instruction::Not, Read::Distance('C'), Write::Temp),
            (Instruction::Or, Read::Temp, Write::Jump),
            (Instruction::And, Read::Distance('D'), Write::Jump),
        ];

        let result = droid.run_instructions(&instructions, false)?;
        Ok(result.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let mut droid = SpringDroid::create(input, 'I', "RUN")?;

        let instructions = [
            (Instruction::Not, Read::Distance('B'), Write::Temp),
            (Instruction::Not, Read::Distance('C'), Write::Jump),
            (Instruction::Or, Read::Temp, Write::Jump),
            (Instruction::And, Read::Distance('D'), Write::Jump),
            (Instruction::And, Read::Distance('H'), Write::Jump),
            (Instruction::Not, Read::Distance('A'), Write::Temp),
            (Instruction::Or, Read::Temp, Write::Jump),
        ];

        let result = droid.run_instructions(&instructions, false)?;
        Ok(result.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Computer error: {0}")]
    ComputerError(#[from] ComputerError),
    #[error("Incorrect Result")]
    IncorrectResult,
    #[error("Incorrect Distance: {0}")]
    IncorrectDistance(char),
}

struct SpringDroid<'a> {
    brain: IntCodeComputer,
    allowed_distance: char,
    start_verb: &'a str,
}

impl<'a> SpringDroid<'a> {
    fn create(code: &str, allowed_distance: char, start_verb: &'a str) -> Result<Self, DayError> {
        let brain = ComputerFactory::init(code)?.build();
        Ok(Self {
            brain,
            allowed_distance,
            start_verb,
        })
    }

    fn send_instructions(
        &mut self,
        instruction: Instruction,
        read: Read,
        write: Write,
    ) -> Result<(), DayError> {
        if let Read::Distance(c) = read {
            if !c.is_ascii_uppercase() || c > self.allowed_distance {
                return Err(DayError::IncorrectDistance(c));
            }
        }

        self.brain
            .send_string(&format!("{instruction} {read} {write}"));
        Ok(())
    }

    fn start_program(&mut self) -> Result<Either<i64, Vec<String>>, DayError> {
        self.brain.send_string(self.start_verb);

        let mut messages = vec![];
        while let Some(line) = self.brain.maybe_string_or_i64()? {
            match line {
                Either::Left(value) => return Ok(Either::Left(value)),
                Either::Right(line) => messages.push(line),
            }
        }

        Ok(Either::Right(messages))
    }

    fn run_instructions(
        &mut self,
        instructions: &[(Instruction, Read, Write)],
        print_error: bool,
    ) -> Result<i64, DayError> {
        for (instruction, read, write) in instructions {
            self.send_instructions(*instruction, *read, *write)?;
        }

        match self.start_program()? {
            Either::Left(value) => Ok(value),
            Either::Right(messages) => {
                if print_error {
                    println!("{}", messages.into_iter().join("\n"));
                }
                Err(DayError::IncorrectResult)
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Read {
    Distance(char),
    Temp,
}

impl Display for Read {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Distance(dist) => *dist,
                Self::Temp => 'T',
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Write {
    Temp,
    Jump,
}

impl Display for Write {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Temp => 'T',
                Self::Jump => 'J',
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    And,
    Or,
    Not,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::And => "AND",
                Self::Or => "OR",
                Self::Not => "NOT",
            }
        )
    }
}
