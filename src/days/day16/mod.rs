use super::{DayTrait, DayType, RResult};
use itertools::Itertools;
use std::str::FromStr;

const DAY_NUMBER: DayType = 16;

pub struct Day;

type Number = i32;

const BASE: [Number; 4] = [0, 1, 0, -1];
const PHASES: usize = 100;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let fft: Fft = input.parse()?;
        let fft = fft.rounds(PHASES);
        Ok(fft.as_usize(8).into())
    }

    fn part2(&self, input: &str) -> RResult {
        let fft: Fft = input.parse()?;
        let skip = fft.as_usize(7);
        let fft = fft.complex_rounds(PHASES, 10_000, skip);
        Ok(fft.as_usize(8).into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not a digit: {0}")]
    NotAtDigit(char),
}

#[derive(Debug, PartialEq, Eq)]
struct Fft(Vec<Number>);

impl FromStr for Fft {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Fft(s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as Number)
                    .ok_or(DayError::NotAtDigit(c))
            })
            .try_collect()?))
    }
}

impl Fft {
    pub fn rounds(self, times: usize) -> Self {
        self.complex_rounds(times, 1, 0)
    }

    fn complex_rounds(self, times: usize, self_repeat: usize, skip: usize) -> Self {
        let len = self.0.len() * self_repeat;
        let mut data = self
            .0
            .iter()
            .copied()
            .cycle()
            .take(len)
            .skip(skip)
            .collect_vec();

        let real_quick_start = len.div_ceil(2);
        let quick_start_index = if real_quick_start > skip {
            real_quick_start - skip
        } else {
            0
        };
        let end_index = data.len();

        for _ in 0..times {
            for index in 0..quick_start_index {
                let phase = index + skip + 1;

                let first_start = phase - 1;
                let mut start = index;
                let mut end =
                    (start + skip - first_start + 1).next_multiple_of(phase) - skip + first_start;

                let mut digit_sum = 0;
                while start < end_index {
                    let idx = ((start + skip + 1) / phase) % BASE.len();
                    if BASE[idx] != 0 {
                        digit_sum += BASE[idx] * data[start..end].iter().sum::<Number>();
                    }
                    start = end;
                    end = (end + phase).min(end_index);
                }

                data[index] = digit_sum.abs() % 10;
            }
            for index in (quick_start_index..end_index - 1).rev() {
                data[index] = (data[index] + data[index + 1]) % 10;
            }
        }
        Self(data)
    }

    pub fn as_usize(&self, digits: usize) -> usize {
        self.0
            .iter()
            .take(digits)
            .fold(0, |sum, digit| sum * 10 + *digit as usize)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::{read_string, ResultType, UnitResult};

    #[test]
    fn test_part1() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;
        let expected = ResultType::Integer(24176176);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example03.txt")?;
        let expected = ResultType::Integer(84462026);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn example1() -> UnitResult {
        let input = "12345678";

        let fft: Fft = input.parse()?;
        let fft = fft.rounds(1);
        assert_eq!(fft.as_usize(8), 48226158);

        let fft: Fft = input.parse()?;
        let fft = fft.rounds(2);
        assert_eq!(fft.as_usize(8), 34040438);

        Ok(())
    }

    #[test]
    fn skip1() -> UnitResult {
        let input = "12345678";

        let fft: Fft = input.parse()?;
        let fft_1 = fft.complex_rounds(1, 1, 1);
        assert_eq!(fft_1.as_usize(8), 8226158);

        let fft: Fft = input.parse()?;
        let fft_2 = fft.complex_rounds(2, 1, 1);
        assert_eq!(fft_2.as_usize(8), 4040438);

        Ok(())
    }

    #[test]
    fn example2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;

        let fft: Fft = input.parse()?;
        let fft = fft.rounds(1);
        assert_eq!(fft.as_usize(8), 24706861);

        Ok(())
    }
}
