use super::{DayTrait, DayType, RResult};
use std::num;

const DAY_NUMBER: DayType = 1;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        Ok(day_impl::get_simple_fuel(input)?.into())
    }

    fn part2(&self, input: &str) -> RResult {
        Ok(day_impl::get_complex_fuel(input)?.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not an Int")]
    ParseIntError(#[from] num::ParseIntError),
}

mod day_impl {
    use super::DayError;
    use itertools::Itertools;
    use std::ops::Add;

    #[inline]
    fn calc(mass: u64) -> u64 {
        mass / 3 - 2
    }

    pub fn get_simple_fuel(input: &str) -> Result<u64, DayError> {
        get_fuel(input, calc)
    }

    pub fn get_complex_fuel(input: &str) -> Result<u64, DayError> {
        let func = |mass| {
            itertools::unfold(mass, |mass| {
                if *mass < 9 {
                    None
                } else {
                    *mass = calc(*mass);
                    Some(*mass)
                }
            })
            .sum()
        };
        get_fuel(input, func)
    }

    fn get_fuel<F>(input: &str, func: F) -> Result<u64, DayError>
    where
        F: FnMut(u64) -> u64,
    {
        Ok(input
            .lines()
            .map(|line| line.parse::<u64>())
            .map_ok(func)
            .fold_ok(0, Add::add)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::{read_string, ResultType, UnitResult};

    #[test]
    fn test_part1() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let expected = ResultType::Integer(33583);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let expected = ResultType::Integer(50346);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }
}
