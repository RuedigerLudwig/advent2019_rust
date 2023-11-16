use super::{DayTrait, DayType, RResult};
use crate::int_code::{IntCodeComputer, Pointer};

const DAY_NUMBER: DayType = 2;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let mut computer: IntCodeComputer = input.parse()?;
        computer.set_address(Pointer::new(1), 12)?;
        computer.set_address(Pointer::new(2), 2)?;
        computer.run()?;
        Ok(computer.get_address(Pointer::new(0))?.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let target = 19690720;
        for noun in 0..100 {
            for verb in 0..100 {
                let mut computer: IntCodeComputer = input.parse()?;
                computer.set_address(Pointer::new(1), noun)?;
                computer.set_address(Pointer::new(2), verb)?;
                computer.run()?;
                if computer.get_address(Pointer::new(0))? == target {
                    return Ok((noun * 100 + verb).into());
                }
            }
        }
        unreachable!()
    }
}
