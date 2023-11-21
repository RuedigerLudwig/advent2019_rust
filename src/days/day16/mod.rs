use super::{DayTrait, DayType, RResult};
use itertools::Itertools;
use std::str::FromStr;

const DAY_NUMBER: DayType = 16;

pub struct Day;

type Number = i32;

const BASE: [Number; 4] = [1, 0, -1, 0];
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
    #[inline]
    fn calc_pattern(phase: usize, pos: usize) -> Number {
        BASE[((pos + 1) / phase + (BASE.len() - 1)) % BASE.len()]
    }

    #[inline]
    fn phase_start(phase: usize, skip: usize) -> (usize, usize) {
        assert!(phase > 0);
        let start_index = phase - 1;
        if skip < start_index {
            (start_index, start_index)
        } else {
            let block_length = BASE.len() * phase;
            let adjusted_start = skip - start_index;
            let adjusted_multiple = adjusted_start.next_multiple_of(block_length);
            let real_start = adjusted_multiple + start_index;
            if adjusted_start == adjusted_multiple || real_start < block_length {
                (real_start, real_start)
            } else {
                (real_start - block_length, real_start)
            }
        }
    }

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

        for _ in 0..times {
            data = ((skip + 1)..=len)
                .map(|phase| {
                    let (first_block, run_start) = Self::phase_start(phase, skip);
                    let start_sum: Number = if first_block != run_start && skip + phase > run_start
                    {
                        data[(first_block - skip)..(run_start - skip - phase)]
                            .iter()
                            .enumerate()
                            .map(|(pos, value)| *value * Self::calc_pattern(phase, pos + skip))
                            .sum()
                    } else {
                        0
                    };

                    let mut rest_sum = 0;
                    let mut offset = run_start - skip;
                    let mut idx = 0;
                    while offset < len - skip {
                        if BASE[idx] != 0 {
                            rest_sum +=
                                BASE[idx] * data[offset..].iter().take(phase).sum::<Number>();
                        }
                        offset += phase;
                        idx += 1;
                        if idx >= BASE.len() {
                            idx = 0;
                        }
                    }

                    (start_sum + rest_sum).abs() % 10
                })
                .collect_vec();
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
        let input = read_string(day.get_day_number(), "input.txt")?;
        let expected = ResultType::Integer(84462026);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn pattern_for_phase() {
        assert_eq!(
            (0..)
                .map(|pos| Fft::calc_pattern(1, pos))
                .take(10)
                .collect_vec(),
            vec![1, 0, -1, 0, 1, 0, -1, 0, 1, 0]
        );
        assert_eq!(
            (0..)
                .map(|pos| Fft::calc_pattern(2, pos))
                .take(10)
                .collect_vec(),
            vec![0, 1, 1, 0, 0, -1, -1, 0, 0, 1]
        );
        assert_eq!(
            (0..)
                .map(|pos| Fft::calc_pattern(3, pos))
                .take(10)
                .collect_vec(),
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1]
        );
    }
    #[test]
    fn start_pattern() {
        assert_eq!(Fft::phase_start(1, 0), (0, 0));
        assert_eq!(Fft::phase_start(1, 1), (0, 4));
        assert_eq!(Fft::phase_start(1, 7), (4, 8));
        assert_eq!(Fft::phase_start(1, 8), (8, 8));
        assert_eq!(Fft::phase_start(1, 9), (8, 12));

        assert_eq!(Fft::phase_start(2, 0), (1, 1));
        assert_eq!(Fft::phase_start(2, 1), (1, 1));
        assert_eq!(Fft::phase_start(2, 8), (1, 9));
        assert_eq!(Fft::phase_start(2, 9), (9, 9));
        assert_eq!(Fft::phase_start(2, 10), (9, 17));
    }

    #[test]
    fn example1() -> UnitResult {
        let input = "12345678";
        let fft: Fft = input.parse()?;

        let fft = fft.rounds(1);
        assert_eq!(fft.as_usize(8), 48226158);

        let fft = fft.rounds(1);
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
}
