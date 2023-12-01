use super::{DayTrait, DayType, RResult};
use crate::{
    common::pos2::Pos2,
    int_code::{ComputerError, ComputerFactory, IntCodeComputer},
};

const DAY_NUMBER: DayType = 19;

pub struct Day;

const SHIP_SIZE: usize = 100;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let mut tractor = TractorBrain::new(input)?;
        let pulled = tractor.count_pulled(50)?;
        Ok(pulled.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let mut tractor = TractorBrain::new(input)?;
        let (x, y) = tractor.find_closest(SHIP_SIZE)?;
        Ok((x * 10_000 + y).into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Computer error: {0}")]
    ComputerError(#[from] ComputerError),
}

struct TractorBrain {
    brain: IntCodeComputer,
}

impl TractorBrain {
    pub fn new(code: &str) -> Result<Self, DayError> {
        let brain = ComputerFactory::init(code)?.build();
        Ok(Self { brain })
    }

    pub fn read_point(&mut self, x: usize, y: usize) -> Result<bool, DayError> {
        self.brain.send_i64(x as i64);
        self.brain.send_i64(y as i64);
        let result = self.brain.expect_bool()?;
        self.brain.reset();
        Ok(result)
    }

    #[allow(clippy::mut_range_bound)]
    pub fn count_pulled(&mut self, max_distance: usize) -> Result<usize, DayError> {
        let mut min_x = 0;
        let mut pulled = 0;
        for y in 0..max_distance {
            let mut found_any = false;
            for x in min_x..max_distance {
                if self.read_point(x, y)? {
                    if !found_any {
                        min_x = x;
                        found_any = true;
                    }
                    pulled += 1;
                } else if found_any {
                    break;
                }
            }
        }
        Ok(pulled)
    }

    fn find_first_pulled(
        &mut self,
        start_x: usize,
        y: usize,
        from_left: bool,
    ) -> Result<usize, DayError> {
        let mut x = start_x;
        let expected = !self.read_point(x, y)?;
        loop {
            let next_x = if expected == from_left { x + 1 } else { x - 1 };
            let point = self.read_point(next_x, y)?;
            if self.read_point(next_x, y)? == expected {
                if point {
                    return Ok(next_x);
                } else {
                    return Ok(x);
                }
            }
            x = next_x;
        }
    }

    fn grow(&mut self, size: usize) -> Result<(usize, usize), DayError> {
        let mut y = 0;
        let mut x = 0;

        loop {
            x = self.find_first_pulled(x, y + size - 1, true)?;
            if self.read_point(x + size - 1, y)? {
                return Ok((x, y));
            } else if y == 0 {
                y = size;
            } else {
                y *= 2;
            }
        }
    }

    fn binary_search(
        &mut self,
        x: usize,
        y: usize,
        size: usize,
    ) -> Result<(usize, usize), DayError> {
        let mut max = Pos2::new(x, y);
        let mut min = max / 2;

        while min.y() < max.y() {
            let middle = (min + max) / 2;
            let left_x = self.find_first_pulled(middle.x(), middle.y() + size - 1, true)?;
            let middle = middle.set_x(left_x);
            if middle == min || middle == max {
                break;
            }
            let right_x = self.find_first_pulled(middle.x() + size - 1, middle.y(), false)?;
            if left_x + size - 1 > right_x {
                min = middle;
            } else {
                max = middle;
            }
        }
        Ok((max.x(), max.y()))
    }

    pub fn find_closest(&mut self, size: usize) -> Result<(usize, usize), DayError> {
        let (x, y) = self.grow(size)?;
        let (x, y) = self.binary_search(x, y, size)?;
        Ok((x, y))
    }
}
