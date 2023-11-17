use super::{DayTrait, DayType, RResult};
use crate::common::{direction::Direction, pos2::Pos2};
use itertools::Itertools;
use std::{collections::HashMap, num, str::FromStr};

const DAY_NUMBER: DayType = 3;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let (wire1, wire2) = Wire::parse_two(input)?;
        let crossings = wire1.crossings(&wire2);
        let min = crossings
            .into_iter()
            .map(|(point, _)| point.abs())
            .min()
            .ok_or(DayError::NoCrossings)?;
        Ok(min.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let (wire1, wire2) = Wire::parse_two(input)?;
        let crossings = wire1.crossings(&wire2);
        let min = crossings
            .into_iter()
            .map(|(_, steps)| steps)
            .min()
            .ok_or(DayError::NoCrossings)?;
        Ok(min.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not a valid description: {0}")]
    ParseError(String),
    #[error("Not an Int")]
    ParseIntError(#[from] num::ParseIntError),
    #[error("No Crossings")]
    NoCrossings,
}

struct Wire {
    sections: Vec<(Direction, i64)>,
}

impl FromStr for Wire {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            sections: s
                .split(',')
                .map(|inst| {
                    if inst.len() < 2 {
                        return Err(DayError::ParseError(inst.to_owned()));
                    }
                    let (dir, dist) = inst.split_at(1);
                    let dist = dist.parse()?;
                    match dir {
                        "R" => Ok((Direction::East, dist)),
                        "U" => Ok((Direction::North, dist)),
                        "L" => Ok((Direction::West, dist)),
                        "D" => Ok((Direction::South, dist)),
                        _ => Err(DayError::ParseError(inst.to_owned())),
                    }
                })
                .try_collect()?,
        })
    }
}
impl Wire {
    pub fn coords(&self) -> HashMap<Pos2<i64>, usize> {
        self.sections
            .iter()
            .fold(
                (HashMap::new(), Pos2::default(), 0),
                |(mut coords, mut pos, mut steps), &(direction, dist)| {
                    for _ in 1..=dist {
                        pos += direction;
                        steps += 1;
                        coords.entry(pos).or_insert(steps);
                    }
                    (coords, pos, steps)
                },
            )
            .0
    }

    pub fn crossings(&self, other: &Wire) -> Vec<(Pos2<i64>, usize)> {
        let other = other.coords();
        self.coords()
            .iter()
            .filter_map(|(coord, steps1)| other.get(coord).map(|steps2| (*coord, steps1 + steps2)))
            .collect()
    }

    fn parse_two(input: &str) -> Result<(Wire, Wire), DayError> {
        let mut wires: Vec<_> = input.lines().map(|line| line.parse()).try_collect()?;
        if wires.len() < 2 {
            Err(DayError::ParseError(input.to_owned()))
        } else {
            Ok((wires.remove(0), wires.remove(0)))
        }
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
        let expected = ResultType::Integer(159);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let expected = ResultType::Integer(610);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    pub fn parse() -> UnitResult {
        let input = "R8,U5,L5,D3";
        let wire: Wire = input.parse()?;
        assert_eq!(
            wire.sections,
            vec![
                (Direction::East, 8),
                (Direction::North, 5),
                (Direction::West, 5),
                (Direction::South, 3),
            ]
        );
        Ok(())
    }

    #[test]
    pub fn crossings() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;
        let (wire1, wire2) = Wire::parse_two(&input)?;

        let crossings = wire1.crossings(&wire2);
        assert_eq!(
            crossings
                .into_iter()
                .sorted_by_key(|(p, _)| p.abs())
                .collect_vec(),
            [(Pos2::new(3, -3), 40), (Pos2::new(6, -5), 30)]
        );

        Ok(())
    }
}
