use super::{DayTrait, DayType, RResult};
use crate::int_code::{ComputerError, ComputerFactory};

const DAY_NUMBER: DayType = 15;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let mut brain = ComputerFactory::init(input)?.build_blocking();
        let maze = maze::Maze::new(&mut brain)?;

        Ok(maze.steps()?.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let mut brain = ComputerFactory::init(input)?.build_blocking();
        let maze = maze::Maze::new(&mut brain)?;

        Ok(maze.oxygenize()?.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Computer error: {0}")]
    ComputerError(#[from] ComputerError),
    #[error("Unknown tile: {0}")]
    UnknownTile(i64),
    #[error("Illegal backstep")]
    IllegalBackstep,
    #[error("No Oxygen found")]
    NoOxygenFound,
    #[error("More than one Oxygen found")]
    MoreThanOneOxygenFond,
}

mod maze {
    use super::DayError;
    use crate::{
        common::{area::Area, direction::Direction, pos2::Pos2},
        int_code::BlockingIntCodeRunner,
    };
    use std::collections::{hash_map::Entry, HashMap};

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    enum Tile {
        Empty,
        Wall,
        Oxygen,
    }

    impl Tile {
        #[inline]
        pub fn can_walk(&self) -> bool {
            !matches!(self, Tile::Wall)
        }
    }

    impl TryFrom<i64> for Tile {
        type Error = DayError;

        fn try_from(value: i64) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Tile::Wall),
                1 => Ok(Tile::Empty),
                2 => Ok(Tile::Oxygen),
                _ => Err(DayError::UnknownTile(value)),
            }
        }
    }

    type Coordinate = Pos2<i32>;

    #[derive(Debug)]
    pub struct Maze {
        tiles: HashMap<Coordinate, Tile>,
        oxygen: Option<Coordinate>,
    }

    impl Maze {
        pub fn new(brain: &mut BlockingIntCodeRunner) -> Result<Self, DayError> {
            let mut maze = Self {
                tiles: HashMap::new(),
                oxygen: None,
            };
            maze.explore(brain)?;
            Ok(maze)
        }

        fn direction_to_command(dir: Direction) -> i64 {
            match dir {
                Direction::East => 4,
                Direction::North => 1,
                Direction::West => 3,
                Direction::South => 2,
            }
        }

        #[allow(dead_code)]
        fn print_maze(&self) {
            let Some(area) = Area::from_iterator(self.tiles.keys()) else {
                println!("fizzle");
                return;
            };
            for y in area.bottom()..=area.top() {
                for x in area.left()..=area.right() {
                    if x == 0 && y == 0 {
                        print!("X");
                        continue;
                    }
                    let tile = self.tiles.get(&Pos2::new(x, y));
                    match tile {
                        Some(Tile::Empty) => print!("."),
                        Some(Tile::Wall) => print!("#"),
                        Some(Tile::Oxygen) => print!("X"),
                        None => print!(" "),
                    }
                }
                println!();
            }
        }

        fn explore(&mut self, brain: &mut BlockingIntCodeRunner) -> Result<(), DayError> {
            let mut path = vec![Direction::East];
            let mut pos = Pos2::default();
            self.tiles.insert(pos, Tile::Empty);
            while let Some(facing) = path.pop() {
                let next_pos = pos + facing;
                let mut do_walk = false;
                if let Entry::Vacant(location) = self.tiles.entry(next_pos) {
                    brain.send_i64(Self::direction_to_command(facing));
                    let tile: Tile = brain.expect_i64()?.try_into()?;
                    location.insert(tile);
                    if tile == Tile::Oxygen {
                        if self.oxygen.is_some() {
                            return Err(DayError::MoreThanOneOxygenFond);
                        }
                        self.oxygen = Some(next_pos);
                    }
                    do_walk = tile.can_walk();
                }

                if do_walk {
                    pos = next_pos;
                    path.push(facing);
                    path.push(Direction::East);
                } else {
                    match facing.turn_left() {
                        Direction::East => {
                            while let Some(prev) = path.pop() {
                                let back = prev.turn_back();
                                pos += back;
                                brain.send_i64(Self::direction_to_command(back));
                                let tile: Tile = brain.expect_i64()?.try_into()?;
                                if !tile.can_walk() {
                                    return Err(DayError::IllegalBackstep);
                                }

                                let next = prev.turn_left();
                                if next != Direction::East {
                                    path.push(next);
                                    break;
                                }
                            }
                        }
                        next_facing => path.push(next_facing),
                    }
                }
            }
            Ok(())
        }

        pub fn steps(&self) -> Result<usize, DayError> {
            if let Some(oxygen) = self.oxygen {
                let times = self.march_tiles(Pos2::default())?;
                Ok(times.get(&oxygen).copied().unwrap())
            } else {
                Err(DayError::NoOxygenFound)
            }
        }

        pub fn oxygenize(&self) -> Result<usize, DayError> {
            if let Some(oxygen) = self.oxygen {
                let times = self.march_tiles(oxygen)?;
                Ok(times.values().max().copied().unwrap())
            } else {
                Err(DayError::NoOxygenFound)
            }
        }

        fn march_tiles(&self, start: Coordinate) -> Result<HashMap<Coordinate, usize>, DayError> {
            let mut times = HashMap::new();
            times.insert(start, 0);
            let mut path = vec![Direction::East];
            let mut pos = start;
            while let Some(facing) = path.pop() {
                let next_pos = pos + facing;

                let tile = self.tiles.get(&next_pos).copied().unwrap_or(Tile::Wall);

                let move_along = tile.can_walk()
                    && times
                        .get(&next_pos)
                        .map(|&steps| steps > path.len())
                        .unwrap_or(true);

                if move_along {
                    times.insert(next_pos, path.len() + 1);
                    pos = next_pos;
                    path.push(facing);
                    path.push(Direction::East);
                } else {
                    match facing.turn_left() {
                        Direction::East => {
                            while let Some(prev) = path.pop() {
                                let back = prev.turn_back();
                                pos += back;

                                let next = prev.turn_left();
                                if next != Direction::East {
                                    path.push(next);
                                    break;
                                }
                            }
                        }
                        next_facing => path.push(next_facing),
                    }
                }
            }
            Ok(times)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::days::{read_string, ResultType, UnitResult};

    #[test]
    pub fn test_part1() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "input.txt")?;
        let expected = ResultType::Nothing;
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }
}
