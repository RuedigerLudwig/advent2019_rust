use super::{DayTrait, DayType, RResult};
use crate::common::math::lcm;
use crate::common::pos3::Pos3;
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::Sum;
use std::ops::{Add, Sub};
use std::{num, str::FromStr};

const DAY_NUMBER: DayType = 12;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let (ticks, system) = System::parse(input)?;
        let system = system.tick(ticks);
        Ok(system.energy().into())
    }

    fn part2(&self, input: &str) -> RResult {
        let (_, system) = System::parse(input)?;
        Ok(system.test_repeat().into())
    }
}

trait Sign {
    fn sign(self) -> Self;
}

trait Moonish {
    type Item: Sub<Output = Self::Item> + Add<Output = Self::Item> + Sign + Sum + Copy;

    fn location(&self) -> Self::Item;
    fn velocity(&self) -> Self::Item;
    fn create(location: Self::Item, velocity: Self::Item) -> Self;

    fn tick(data: Vec<Self>) -> Vec<Self>
    where
        Self: Sized,
    {
        let delta = data
            .iter()
            .permutations(2)
            .map(|x| (x[1].location() - x[0].location()).sign())
            .chunks(data.len() - 1)
            .into_iter()
            .map(|delta| delta.sum::<Self::Item>())
            .collect_vec();

        data.into_iter()
            .zip(delta)
            .map(|(moon, delta)| {
                let velocity = moon.velocity() + delta;
                Self::create(moon.location() + velocity, velocity)
            })
            .collect_vec()
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not a valid description: {0}")]
    ParseError(String),
    #[error("Not an Int")]
    ParseIntError(#[from] num::ParseIntError),
}

#[derive(Debug, PartialEq, Eq)]
struct Moon {
    location: Pos3<i64>,
    velocity: Pos3<i64>,
}

impl Sign for Pos3<i64> {
    fn sign(self) -> Self {
        self.signum()
    }
}

impl Moonish for Moon {
    type Item = Pos3<i64>;

    #[inline]
    fn create(location: Self::Item, velocity: Self::Item) -> Self {
        Self { location, velocity }
    }

    #[inline]
    fn location(&self) -> Self::Item {
        self.location
    }

    #[inline]
    fn velocity(&self) -> Self::Item {
        self.velocity
    }
}

impl FromStr for Moon {
    type Err = DayError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn components(input: &str) -> Option<(&str, &str, &str)> {
            let input = input.strip_prefix('<')?;
            let input = input.strip_suffix('>')?;
            let input = input.split(',').collect_vec();
            if input.len() != 3 {
                None
            } else {
                Some((
                    input[0].trim().strip_prefix("x=")?,
                    input[1].trim().strip_prefix("y=")?,
                    input[2].trim().strip_prefix("z=")?,
                ))
            }
        }

        let (x, y, z) = components(input).ok_or(DayError::ParseError(input.to_owned()))?;
        Ok(Self {
            location: Pos3::new(x.parse()?, y.parse()?, z.parse()?),
            velocity: Pos3::default(),
        })
    }
}

impl Moon {
    #[inline]
    pub fn potential(&self) -> i64 {
        self.location.iter().map(|v| v.abs()).sum()
    }

    #[inline]
    pub fn kinetic(&self) -> i64 {
        self.velocity.iter().map(|v| v.abs()).sum()
    }

    #[inline]
    pub fn energy(&self) -> i64 {
        self.potential() * self.kinetic()
    }
}

struct System {
    moons: Vec<Moon>,
}

impl System {
    fn parse(input: &str) -> Result<(usize, Self), DayError> {
        let Some((ticks, system)) = input.split_once('\n') else {
            return Err(DayError::ParseError(input.to_owned()));
        };

        let ticks = ticks.parse()?;
        Ok((
            ticks,
            Self {
                moons: system.lines().map(|line| line.parse()).try_collect()?,
            },
        ))
    }

    pub fn energy(&self) -> i64 {
        self.moons.iter().map(|moon| moon.energy()).sum()
    }

    pub fn tick(self, times: usize) -> Self {
        let mut data = self.moons;
        for _ in 0..times {
            data = Moon::tick(data)
        }
        Self { moons: data }
    }

    fn repeat_one(&self, index: usize) -> usize {
        let mut data = self
            .moons
            .iter()
            .map(|moon| (moon.location()[index], moon.velocity()[index]))
            .collect_vec();
        let mut seen = HashMap::new();
        seen.insert(data.clone(), 0);
        for round in 1.. {
            data = Moonish::tick(data);
            if let Some(prev) = seen.get(&data) {
                return round - *prev;
            }
            seen.insert(data.clone(), round);
        }
        unreachable!()
    }

    pub fn test_repeat(self) -> usize {
        (0..3).map(|num| self.repeat_one(num)).reduce(lcm).unwrap()
    }
}

impl Sign for i64 {
    #[inline]
    fn sign(self) -> Self {
        self.signum()
    }
}

impl Moonish for (i64, i64) {
    type Item = i64;

    #[inline]
    fn location(&self) -> Self::Item {
        self.0
    }

    #[inline]
    fn velocity(&self) -> Self::Item {
        self.1
    }

    #[inline]
    fn create(location: Self::Item, velocity: Self::Item) -> Self {
        (location, velocity)
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
        let expected = ResultType::Integer(1940);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;
        let expected = ResultType::Integer(4686774924);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn parse() -> UnitResult {
        let input = "<x=-1, y=0, z=2>";
        let moon: Moon = input.parse()?;
        let expected = Moon {
            location: Pos3::new(-1, 0, 2),
            velocity: Pos3::new(0, 0, 0),
        };
        assert_eq!(moon, expected);
        Ok(())
    }

    #[test]
    fn add_delta() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let (_, system) = System::parse(&input)?;
        let system = system.tick(1);

        assert_eq!(
            system.moons.iter().map(|moon| moon.velocity).collect_vec(),
            vec![
                Pos3::new(3, -1, -1),
                Pos3::new(1, 3, 3),
                Pos3::new(-3, 1, -3),
                Pos3::new(-1, -3, 1),
            ]
        );

        assert_eq!(
            system.moons.iter().map(|moon| moon.location).collect_vec(),
            vec![
                Pos3::new(2, -1, 1),
                Pos3::new(3, -7, -4),
                Pos3::new(1, -7, 5),
                Pos3::new(2, 2, 0),
            ]
        );

        Ok(())
    }

    #[test]
    fn energy() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let (ticks, system) = System::parse(&input)?;
        let system = system.tick(ticks);

        assert_eq!(system.energy(), 179);

        Ok(())
    }

    #[test]
    fn repeat() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let (_, system) = System::parse(&input)?;

        assert_eq!(system.test_repeat(), 2772);

        Ok(())
    }

    #[test]
    fn repeat_long() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;
        let (_, system) = System::parse(&input)?;

        assert_eq!(system.test_repeat(), 4686774924);

        Ok(())
    }
}
