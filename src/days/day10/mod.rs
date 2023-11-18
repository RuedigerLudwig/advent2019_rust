use super::{DayTrait, DayType, RResult};
use crate::common::pos2::Pos2;
use itertools::Itertools;
use std::{cell::RefCell, num, str::FromStr};

const DAY_NUMBER: DayType = 10;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let field: AsteroidField = input.parse()?;
        let station = field.place_station();
        Ok(station.visible_asteroids().into())
    }

    fn part2(&self, input: &str) -> RResult {
        let field: AsteroidField = input.parse()?;
        let mut station = field.place_station();
        let last_asteroid = station.shoot_number_asteroids(200)?;
        Ok((last_asteroid.x() * 100 + last_asteroid.y()).into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not an Int")]
    ParseIntError(#[from] num::ParseIntError),
    #[error("Empty fields are not allowed")]
    NoEmptyField,
    #[error("Not enough asteroids")]
    NotEnoughAsteroids,
    #[error("What do you want me to do?")]
    NothingToDo,
}

#[derive(Debug)]
struct AsteroidField {
    objects: Vec<Pos2<i64>>,
}

impl AsteroidField {
    fn new(raw: Vec<Vec<bool>>) -> Result<AsteroidField, DayError> {
        let objects = raw
            .into_iter()
            .enumerate()
            .map(|(y, line)| (y as i64, line))
            .flat_map(|(y, line)| {
                line.into_iter()
                    .enumerate()
                    .filter(|(_, a)| *a)
                    .map(move |(x, _)| Pos2::new(x as i64, y))
            })
            .collect_vec();

        if objects.is_empty() {
            return Err(DayError::NoEmptyField);
        }
        Ok(Self { objects })
    }

    fn count_seen_at(&self, station: Pos2<i64>) -> usize {
        self.objects
            .iter()
            .filter_map(|&pos| (pos - station).normalize().map(|(pos, _)| pos).ok())
            .unique()
            .count()
    }

    fn best_place_for_station(&self) -> Pos2<i64> {
        self.objects
            .iter()
            .map(|&pos| (pos, self.count_seen_at(pos)))
            .max_by_key(|&(_, count)| count)
            .map(|(pos, _)| pos)
            .unwrap()
    }

    pub fn place_station(self) -> Station {
        let position = self.best_place_for_station();
        Station::new(self, position)
    }
}

impl FromStr for AsteroidField {
    type Err = DayError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let asteroids = input
            .lines()
            .map(|line| line.chars().map(|a| a == '#').collect_vec())
            .collect_vec();
        AsteroidField::new(asteroids)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct AngleOrderNormal(Pos2<i64>);

impl AngleOrderNormal {
    pub fn quarter(&self) -> usize {
        match (self.0.x().signum(), self.0.y().signum()) {
            (0, -1) | (1, -1) => 1,
            (1, 0) | (1, 1) => 2,
            (0, 1) | (-1, 1) => 3,
            (-1, 0) | (-1, -1) => 4,
            (0, 0) => 0,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for AngleOrderNormal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AngleOrderNormal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let quarter = self.quarter();
        match quarter.cmp(&other.quarter()) {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match quarter {
            1 => (self.0.x() * -other.0.y()).cmp(&(other.0.x() * -self.0.y())),
            2 => (other.0.x() * self.0.y()).cmp(&(self.0.x() * other.0.y())),
            3 => (-self.0.x() * other.0.y()).cmp(&(-other.0.x() * self.0.y())),
            4 => (-other.0.x() * -self.0.y()).cmp(&(-self.0.x() * -other.0.y())),
            0 => std::cmp::Ordering::Equal,
            _ => unreachable!(),
        }
    }
}

struct AsteroidPosition {
    normal: AngleOrderNormal,
    factor: i64,
}

impl AsteroidPosition {
    pub fn new(pos: Pos2<i64>) -> Option<AsteroidPosition> {
        pos.normalize().ok().map(|(pos, factor)| Self {
            normal: AngleOrderNormal(pos),
            factor,
        })
    }

    #[inline]
    pub fn position(&self) -> Pos2<i64> {
        self.normal.0 * self.factor
    }
}

struct Station {
    asteroids: Vec<RefCell<Vec<AsteroidPosition>>>,
    position: Pos2<i64>,
}

impl Station {
    #[inline]
    pub fn visible_asteroids(&self) -> usize {
        self.asteroids
            .iter()
            .filter(|lineup| !lineup.borrow().is_empty())
            .count()
    }

    pub fn new(field: AsteroidField, station: Pos2<i64>) -> Self {
        let asteroids = field
            .objects
            .into_iter()
            .filter_map(|pos| AsteroidPosition::new(pos - station))
            .sorted_by_key(|pos| pos.normal)
            .group_by(|a| a.normal)
            .into_iter()
            .map(|(_, group)| {
                RefCell::new(
                    group
                        .into_iter()
                        .sorted_by_key(|pos| -pos.factor)
                        .collect_vec(),
                )
            })
            .collect_vec();
        Self {
            asteroids,
            position: station,
        }
    }

    pub fn shoot_number_asteroids(&mut self, number: usize) -> Result<Pos2<i64>, DayError> {
        if number == 0 {
            return Err(DayError::NothingToDo);
        }
        match self.shooting().nth(number - 1) {
            Some(last_asteroid) => Ok(last_asteroid),
            None => Err(DayError::NotEnoughAsteroids),
        }
    }

    pub fn shooting(&mut self) -> impl Iterator<Item = Pos2<i64>> + '_ {
        struct ShootingIterator<'b> {
            canon: &'b Station,
            iter: std::slice::Iter<'b, RefCell<Vec<AsteroidPosition>>>,
        }

        impl<'a> Iterator for ShootingIterator<'a> {
            type Item = Pos2<i64>;

            fn next(&mut self) -> Option<Self::Item> {
                let mut repeated = false;
                loop {
                    if let Some(lineup) = self.iter.next() {
                        let mut lineup = lineup.borrow_mut();
                        if let Some(asteroid) = lineup.pop() {
                            return Some(asteroid.position() + self.canon.position);
                        }
                    } else if repeated {
                        return None;
                    } else {
                        self.iter = self.canon.asteroids.iter();
                        repeated = true;
                    }
                }
            }
        }
        ShootingIterator {
            canon: self,
            iter: self.asteroids.iter(),
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
        let expected = ResultType::Integer(210);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let expected = ResultType::Integer(802);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn count() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let field: AsteroidField = input.parse()?;
        assert_eq!(field.count_seen_at(Pos2::new(11, 13)), 210);
        assert_eq!(field.best_place_for_station(), Pos2::new(11, 13));
        Ok(())
    }

    #[test]
    fn shooting_some() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;
        let field: AsteroidField = input.parse()?;
        let mut cannon = Station::new(field, Pos2::new(8, 3));
        assert_eq!(
            cannon.shooting().take(5).collect_vec(),
            vec![
                Pos2::new(8, 1),
                Pos2::new(9, 0),
                Pos2::new(9, 1),
                Pos2::new(10, 0),
                Pos2::new(9, 2)
            ]
        );

        Ok(())
    }

    #[test]
    fn shooting_many() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let field: AsteroidField = input.parse()?;
        let mut cannon = field.place_station();
        assert_eq!(cannon.shooting().nth(199), Some(Pos2::new(8, 2)));

        Ok(())
    }
}
