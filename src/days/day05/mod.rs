use super::{DayTrait, DayType, RResult};
use crate::int_code::ComputerFactory;

const DAY_NUMBER: DayType = 5;

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
        computer.push_input(5);
        let mut result = 0;
        for output in computer.run_blocking() {
            result = output?;
        }
        Ok(result.into())
    }
}

#[cfg(test)]
mod test {
    use crate::{
        days::UnitResult,
        int_code::{ComputerFactory, Pointer},
    };

    #[test]
    fn param_mode() -> UnitResult {
        let input = "1101,100,-1,4,0";
        let factory = ComputerFactory::init(input)?;
        let mut computer = factory.build();

        computer.run()?;
        assert_eq!(computer.get_value_at(Pointer::new(4)), 99);

        Ok(())
    }

    #[test]
    fn complex() -> UnitResult {
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let factory = ComputerFactory::init(input)?;

        let mut computer = factory.build();
        computer.push_input(7);
        let result = computer.run_blocking().next().unwrap()?;
        assert_eq!(result, 999);

        let mut computer = factory.build();
        computer.push_input(8);
        let result = computer.run_blocking().next().unwrap()?;
        assert_eq!(result, 1000);

        let mut computer = factory.build();
        computer.push_input(9);
        let result = computer.run_blocking().next().unwrap()?;
        assert_eq!(result, 1001);

        Ok(())
    }
}
