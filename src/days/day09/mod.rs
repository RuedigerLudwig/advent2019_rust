use crate::int_code::ComputerFactory;

use super::{DayTrait, DayType, RResult};
use std::num;

const DAY_NUMBER: DayType = 9;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let factory = ComputerFactory::init(input)?;
        let mut computer = factory.build();
        computer.push_input(1);
        let mut result = 0;
        for output in computer.run_blocking() {
            result = output?;
        }
        Ok(result.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let factory = ComputerFactory::init(input)?;
        let mut computer = factory.build();
        computer.push_input(2);
        let mut result = 0;
        for output in computer.run_blocking() {
            result = output?;
        }
        Ok(result.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not an Int")]
    ParseIntError(#[from] num::ParseIntError),
}

#[cfg(test)]
mod test {
    use crate::{days::UnitResult, int_code::ComputerFactory};
    use itertools::Itertools;

    #[test]
    fn copy() -> UnitResult {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let factory = ComputerFactory::new(input.clone());

        let mut computer = factory.build();
        let result: Vec<_> = computer.run_blocking().try_collect()?;
        assert_eq!(result, input);
        Ok(())
    }
}
