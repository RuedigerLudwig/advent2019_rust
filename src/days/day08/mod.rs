use super::{DayTrait, DayType, RResult};
use itertools::Itertools;

const DAY_NUMBER: DayType = 8;

pub struct Day;

const COLS: usize = 25;
const ROWS: usize = 6;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let picture = Picture::parse(input, COLS, ROWS)?;
        Ok(picture.count_numbers().into())
    }

    fn part2(&self, input: &str) -> RResult {
        let picture = Picture::parse(input, COLS, ROWS)?;
        Ok(picture.decode()?.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not a valid description: {0}")]
    ParseError(String),
    #[error("no complete picture Found")]
    NoCompletePictureFound,
}

struct Picture {
    layers: Vec<Vec<u8>>,
    cols: usize,
    rows: usize,
}

impl Picture {
    fn parse(input: &str, cols: usize, rows: usize) -> Result<Self, DayError> {
        if input.chars().any(|c| !('0'..='2').contains(&c)) {
            return Err(DayError::ParseError(input.to_owned()));
        }
        let layers = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .chunks(cols * rows)
            .into_iter()
            .map(|layer| layer.collect_vec())
            .collect_vec();
        Ok(Self { layers, cols, rows })
    }

    pub fn count_numbers(&self) -> usize {
        let (_, ones, twos) = self
            .layers
            .iter()
            .map(|layer| {
                let counts = layer.iter().counts();
                (
                    counts.get(&0).copied().unwrap_or_default(),
                    counts.get(&1).copied().unwrap_or_default(),
                    counts.get(&2).copied().unwrap_or_default(),
                )
            })
            .min_by_key(|(zeros, _, _)| *zeros)
            .unwrap();
        ones * twos
    }

    pub fn decode(&self) -> Result<Vec<Vec<bool>>, DayError> {
        let picture =
            self.layers
                .iter()
                .rev()
                .fold(vec![None; self.cols * self.rows], |mut lower, upper| {
                    for (l, u) in lower.iter_mut().zip(upper) {
                        match u {
                            0 => *l = Some(0),
                            1 => *l = Some(1),
                            _ => {}
                        }
                    }
                    lower
                });

        if picture.iter().any(|p| p.is_none()) {
            return Err(DayError::NoCompletePictureFound);
        }

        Ok(picture
            .into_iter()
            .chunks(self.cols)
            .into_iter()
            .map(|p| p.map(|p| p == Some(1)).collect_vec())
            .collect_vec())
    }
}
