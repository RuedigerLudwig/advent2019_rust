use super::{DayTrait, DayType, RResult};
use std::num;

const DAY_NUMBER: DayType = 4;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        Ok(day_impl::check_range(input, day_impl::check_password)?.into())
    }

    fn part2(&self, input: &str) -> RResult {
        Ok(day_impl::check_range(input, day_impl::check_better_password)?.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not a valid description: {0}")]
    ParseError(String),
    #[error("Not an Int")]
    ParseIntError(#[from] num::ParseIntError),
}

mod day_impl {
    use itertools::{self, FoldWhile, Itertools};

    use super::DayError;

    pub fn check_range<F>(input: &str, func: F) -> Result<usize, DayError>
    where
        F: Fn(u64) -> bool,
    {
        let Some((fst, snd)) = input.split_once('-') else {
            return Err(DayError::ParseError(input.to_owned()));
        };
        let fst = fst.parse()?;
        let snd = snd.parse()?;
        Ok((fst..=snd).filter(|&num| func(num)).count())
    }

    pub fn extract_digits(number: u64) -> impl Iterator<Item = u64> {
        itertools::unfold(number, |number| {
            if *number > 0 {
                let digit = *number % 10;
                *number /= 10;
                Some(digit)
            } else {
                None
            }
        })
    }

    pub fn check_password(number: u64) -> bool {
        let check = extract_digits(number).fold_while(
            (None, false),
            |(last, double): (Option<u64>, bool), digit| -> FoldWhile<(Option<_>, bool)> {
                if let Some(last) = last {
                    match last.cmp(&digit) {
                        std::cmp::Ordering::Less => FoldWhile::Done((None, false)),
                        std::cmp::Ordering::Equal => FoldWhile::Continue((Some(digit), true)),
                        std::cmp::Ordering::Greater => FoldWhile::Continue((Some(digit), double)),
                    }
                } else {
                    FoldWhile::Continue((Some(digit), false))
                }
            },
        );
        matches!(check, FoldWhile::Continue((_, true)))
    }

    pub fn check_better_password(number: u64) -> bool {
        let check = extract_digits(number)
            .group_by(|&id| id)
            .into_iter()
            .fold_while(
                (None, false),
                |(last, double): (Option<u64>, bool),
                 (digit, group)|
                 -> FoldWhile<(Option<_>, bool)> {
                    match last {
                        Some(last) if last < digit => FoldWhile::Done((None, false)),
                        _ => FoldWhile::Continue((Some(digit), double || group.count() == 2)),
                    }
                },
            );
        matches!(check, FoldWhile::Continue((_, true)))
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
        let expected = ResultType::Integer(2);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let expected = ResultType::Integer(1);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn check_password() {
        assert!(day_impl::check_password(122345));
        assert!(day_impl::check_password(111123));
        assert!(!day_impl::check_password(135679));
        assert!(day_impl::check_password(111111));
        assert!(!day_impl::check_password(223450));
        assert!(!day_impl::check_password(123789));
    }

    #[test]
    fn check_better_password() {
        assert!(day_impl::check_better_password(122345));
        assert!(!day_impl::check_better_password(111123));
        assert!(!day_impl::check_better_password(135679));
        assert!(!day_impl::check_better_password(111111));
        assert!(!day_impl::check_better_password(223450));
        assert!(!day_impl::check_better_password(123789));
        assert!(day_impl::check_better_password(111122));
    }
}
