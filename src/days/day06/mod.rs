use super::{DayTrait, DayType, RResult};
use itertools::Itertools;

const DAY_NUMBER: DayType = 6;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let system = System::try_from(input)?;
        Ok(system.orbits().into())
    }

    fn part2(&self, input: &str) -> RResult {
        let system = System::try_from(input)?;
        Ok(system.path_between(ME, SANTA).into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not a valid description: {0}")]
    ParseError(String),
}

struct Orbit<'a>(&'a str, &'a str);

impl<'a> TryFrom<&'a str> for Orbit<'a> {
    type Error = DayError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some((center, satelite)) = value.split_once(')') {
            Ok(Self(center, satelite))
        } else {
            Err(DayError::ParseError(value.to_owned()))
        }
    }
}

const CENTER: &str = "COM";
const ME: &str = "YOU";
const SANTA: &str = "SAN";

struct System<'a> {
    objects: Vec<&'a str>,
    parent: Vec<Option<usize>>,
}

impl<'a> TryFrom<&'a str> for System<'a> {
    type Error = DayError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (objects, parent) = value.lines().map(Orbit::try_from).fold_ok(
            (vec![CENTER], vec![None]),
            |(mut objects, mut parent), Orbit(center, satelite)| {
                let center_pos = match objects.iter().position(|&item| item == center) {
                    Some(pos) => pos,
                    None => {
                        let pos = objects.len();
                        objects.push(center);
                        parent.push(None);
                        pos
                    }
                };
                let pos = match objects.iter().position(|&item| item == satelite) {
                    Some(pos) => pos,
                    None => {
                        let pos = objects.len();
                        objects.push(satelite);
                        parent.push(None);
                        pos
                    }
                };
                parent[pos] = Some(center_pos);
                (objects, parent)
            },
        )?;
        if parent[0].is_some() {
            return Err(DayError::ParseError(value.to_owned()));
        }
        if parent.iter().skip(1).any(|item| item.is_none()) {
            return Err(DayError::ParseError(value.to_owned()));
        }
        Ok(System { objects, parent })
    }
}

impl System<'_> {
    fn fill_orbits(&self, orbits: &mut [Option<usize>], current: usize) -> usize {
        if let Some(prev) = orbits[current] {
            return prev;
        }
        let my_orbits = self.fill_orbits(orbits, self.parent[current].unwrap()) + 1;
        orbits[current] = Some(my_orbits);
        my_orbits
    }

    fn find_common_orbits(&self, orbits: &mut [Option<usize>], current: usize) -> (usize, usize) {
        if let Some(prev) = orbits[current] {
            return (prev, prev);
        }
        let (common_orbits, my_orbits) =
            self.find_common_orbits(orbits, self.parent[current].unwrap());
        (common_orbits, my_orbits + 1)
    }

    pub fn orbits(&self) -> usize {
        let mut orbits = vec![None; self.objects.len()];
        orbits[0] = Some(0);
        for pos in 1..self.objects.len() {
            self.fill_orbits(&mut orbits, pos);
        }
        orbits.into_iter().flatten().sum()
    }

    pub fn path_between(&self, me: &str, santa: &str) -> usize {
        let mut orbits = vec![None; self.objects.len()];
        orbits[0] = Some(0);
        let santas_pos = self.objects.iter().position(|&i| i == santa).unwrap();
        let santas_orbits = self.fill_orbits(&mut orbits, santas_pos);
        let my_pos = self.objects.iter().position(|&i| i == me).unwrap();
        let (common_orbits, my_orbits) = self.find_common_orbits(&mut orbits, my_pos);
        santas_orbits + my_orbits - 2 * common_orbits - 2
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
        let expected = ResultType::Integer(42);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;
        let expected = ResultType::Integer(4);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn parse() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let system = System::try_from(input.as_str())?;

        assert_eq!(
            system.objects,
            ["COM", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L"]
        );
        assert_eq!(
            system.parent.into_iter().flatten().collect_vec(),
            [0, 1, 2, 3, 4, 1, 6, 3, 4, 9, 10]
        );
        Ok(())
    }

    #[test]
    fn orbits() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let system = System::try_from(input.as_str())?;
        assert_eq!(system.orbits(), 42);

        Ok(())
    }

    #[test]
    fn meet_orbits() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;
        let system = System::try_from(input.as_str())?;
        assert_eq!(system.path_between(ME, SANTA), 4);

        Ok(())
    }
}
