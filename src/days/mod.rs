mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod template;

pub use template::{read_string, DayTrait, DayType, PartType, RResult, ResultType, UnitResult};

pub mod day_provider {
    use super::*;
    use thiserror::Error;

    const MAX_DAY: DayType = 19;

    pub fn get_day(day_num: DayType) -> core::result::Result<Box<dyn DayTrait>, ProviderError> {
        match day_num {
            1 => Ok(Box::new(day01::Day)),
            2 => Ok(Box::new(day02::Day)),
            3 => Ok(Box::new(day03::Day)),
            4 => Ok(Box::new(day04::Day)),
            5 => Ok(Box::new(day05::Day)),
            6 => Ok(Box::new(day06::Day)),
            7 => Ok(Box::new(day07::Day)),
            8 => Ok(Box::new(day08::Day)),
            9 => Ok(Box::new(day09::Day)),
            10 => Ok(Box::new(day10::Day)),
            11 => Ok(Box::new(day11::Day)),
            12 => Ok(Box::new(day12::Day)),
            13 => Ok(Box::new(day13::Day)),
            14 => Ok(Box::new(day14::Day)),
            15 => Ok(Box::new(day15::Day)),
            16 => Ok(Box::new(day16::Day)),
            17 => Ok(Box::new(day17::Day)),
            18 => Ok(Box::new(day18::Day)),
            19 => Ok(Box::new(day19::Day)),
            _ => Err(ProviderError::InvalidNumber(day_num)),
        }
    }

    pub fn get_all_days() -> impl Iterator<Item = Box<dyn DayTrait>> {
        (1..=MAX_DAY).map(|day_num| get_day(day_num).expect("Must never happen"))
    }

    #[derive(Debug, Error)]
    pub enum ProviderError {
        #[error("Not a valid day number: {0}")]
        InvalidNumber(DayType),
    }
}
