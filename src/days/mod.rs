mod day01;
mod day02;
mod template;

pub use template::{read_string, DayTrait, DayType, PartType, RResult, ResultType, UnitResult};

pub mod day_provider {
    use super::*;
    use thiserror::Error;

    const MAX_DAY: DayType = 2;

    pub fn get_day(day_num: DayType) -> core::result::Result<Box<dyn DayTrait>, ProviderError> {
        match day_num {
            1 => Ok(Box::new(day01::Day)),
            2 => Ok(Box::new(day02::Day)),
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
