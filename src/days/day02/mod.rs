use super::{DayTrait, DayType, RResult};
use crate::int_code::{ComputerFactory, Pointer};

const DAY_NUMBER: DayType = 2;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let factory = ComputerFactory::init(input)?;
        let mut computer = factory.build();

        computer.set_address(Pointer::new(1), 12)?;
        computer.set_address(Pointer::new(2), 2)?;
        computer.run()?;
        Ok(computer.get_value_at(Pointer::new(0))?.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let factory = ComputerFactory::init(input)?;
        let target = 19690720;
        for noun in 0..100 {
            for verb in 0..100 {
                let mut computer = factory.build();
                computer.set_address(Pointer::new(1), noun)?;
                computer.set_address(Pointer::new(2), verb)?;
                computer.run()?;
                if computer.get_value_at(Pointer::new(0))? == target {
                    return Ok((noun * 100 + verb).into());
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        days::UnitResult,
        int_code::{ComputerFactory, Pointer},
    };

    #[test]
    fn simple() -> UnitResult {
        let input = "1,9,10,3,2,3,11,0,99,30,40,50";

        let factory = ComputerFactory::init(input)?;
        let mut computer = factory.build();

        computer.run()?;

        assert_eq!(computer.get_value_at(Pointer::new(0))?, 3500);

        Ok(())
    }
}
