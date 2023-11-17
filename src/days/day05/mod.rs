use crate::int_code::IntCodeComputer;

use super::{DayTrait, DayType, RResult};

const DAY_NUMBER: DayType = 5;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let mut computer: IntCodeComputer = input.parse()?;
        computer.push_input(1);
        let mut result = 0;
        while let Some(output) = computer.run()? {
            result = output;
        }
        Ok(result.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let mut computer: IntCodeComputer = input.parse()?;
        computer.push_input(5);
        let mut result = 0;
        while let Some(output) = computer.run()? {
            result = output;
        }
        Ok(result.into())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        days::UnitResult,
        int_code::{IntCodeComputer, Pointer},
    };

    #[test]
    fn param_mode() -> UnitResult {
        let input = "1101,100,-1,4,0";
        let mut computer: IntCodeComputer = input.parse()?;

        computer.run()?;
        assert_eq!(computer.get_value_at(Pointer::new(4))?, 99);

        Ok(())
    }

    #[test]
    fn complex() -> UnitResult {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

        let mut computer: IntCodeComputer = input.parse()?;
        computer.push_input(7);
        let result = computer.run()?;
        assert_eq!(result, Some(999));

        let mut computer: IntCodeComputer = input.parse()?;
        computer.push_input(8);
        let result = computer.run()?;
        assert_eq!(result, Some(1000));

        let mut computer: IntCodeComputer = input.parse()?;
        computer.push_input(9);
        let result = computer.run()?;
        assert_eq!(result, Some(1001));

        Ok(())
    }
}
