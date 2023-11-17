use crate::int_code::{ComputerError, ComputerFactory};

use super::{DayTrait, DayType, RResult};
use itertools::Itertools;
use std::num;

const DAY_NUMBER: DayType = 7;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let amplifier = Amplifier::create(input)?;
        let result = amplifier.max_once()?;
        Ok(result.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let amplifier = Amplifier::create(input)?;
        let result = amplifier.max_recursive()?;
        Ok(result.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not an Int")]
    ParseIntError(#[from] num::ParseIntError),
    #[error("Computer Error")]
    ComputerError(#[from] ComputerError),
}

struct Amplifier {
    factory: ComputerFactory,
}

impl Amplifier {
    pub fn create(input: &str) -> Result<Self, DayError> {
        let factory = ComputerFactory::init(input)?;
        Ok(Self { factory })
    }

    fn max_once(&self) -> Result<i64, DayError> {
        self.max_result(0..5, |phase| self.run(&phase))
    }

    fn max_recursive(&self) -> Result<i64, DayError> {
        self.max_result(5..10, |phase| self.run_recursive(&phase))
    }

    fn max_result<F>(&self, phase_values: std::ops::Range<i64>, func: F) -> Result<i64, DayError>
    where
        F: FnMut(Vec<i64>) -> Result<i64, DayError>,
    {
        let len = (phase_values.end - phase_values.start) as usize;
        phase_values
            .permutations(len)
            .map(func)
            .fold_ok(i64::MIN, |v, x| v.max(x))
    }

    pub fn run(&self, phase_value: &[i64]) -> Result<i64, DayError> {
        let mut value = 0;
        for &phase in phase_value {
            let mut computer = self.factory.build();
            computer.push_input(phase);
            computer.push_input(value);
            if let Some(next_value) = computer.next() {
                value = next_value?;
            }
        }
        Ok(value)
    }

    pub fn run_recursive(&self, phase_value: &[i64]) -> Result<i64, DayError> {
        let mut computers = phase_value
            .iter()
            .zip(self.factory.iter())
            .map(|(phase, mut computer)| {
                computer.push_input(*phase);
                computer
            })
            .collect_vec();

        let mut value = 0;
        loop {
            for computer in computers.iter_mut() {
                computer.push_input(value);
                if let Some(next_value) = computer.next() {
                    value = next_value?;
                } else {
                    return Ok(value);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::{read_string, ResultType, UnitResult};

    #[test]
    fn test_part1() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example03.txt")?;
        let expected = ResultType::Integer(65210);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example04.txt")?;
        let expected = ResultType::Integer(139629729);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn run_once() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let amplifier = Amplifier::create(&input)?;
        let result = amplifier.run(&[4, 3, 2, 1, 0])?;
        assert_eq!(result, 43210);

        let input = read_string(day.get_day_number(), "example02.txt")?;
        let amplifier = Amplifier::create(&input)?;
        let result = amplifier.run(&[0, 1, 2, 3, 4])?;
        assert_eq!(result, 54321);

        let input = read_string(day.get_day_number(), "example03.txt")?;
        let amplifier = Amplifier::create(&input)?;
        let result = amplifier.run(&[1, 0, 4, 3, 2])?;
        assert_eq!(result, 65210);

        Ok(())
    }

    #[test]
    fn max_once() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let amplifier = Amplifier::create(&input)?;
        let result = amplifier.max_once()?;
        assert_eq!(result, 43210);
        Ok(())
    }

    #[test]
    fn run_recursive() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example04.txt")?;
        let amplifier = Amplifier::create(&input)?;
        let result = amplifier.run_recursive(&[9, 8, 7, 6, 5])?;
        assert_eq!(result, 139629729);

        let input = read_string(day.get_day_number(), "example05.txt")?;
        let amplifier = Amplifier::create(&input)?;
        let result = amplifier.run_recursive(&[9, 7, 8, 5, 6])?;
        assert_eq!(result, 18216);

        Ok(())
    }

    #[test]
    fn max_recursive() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example04.txt")?;
        let amplifier = Amplifier::create(&input)?;
        let result = amplifier.max_recursive()?;
        assert_eq!(result, 139629729);

        Ok(())
    }
}
