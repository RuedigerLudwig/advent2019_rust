use super::{DayTrait, DayType, RResult};
use crate::{
    common::{direction::Direction, pos2::Pos2, turn::Turn},
    int_code::{ComputerError, ComputerFactory, IntCodeComputer},
};
use itertools::Itertools;
use std::{fmt::Display, num, ops::Add, str::FromStr};

const DAY_NUMBER: DayType = 17;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let mut reader = AsciiReader::new(input)?;
        let picture: RobotPicture = reader.get_image()?.parse()?;
        Ok(picture.crossing_sum().into())
    }

    fn part2(&self, input: &str) -> RResult {
        let mut reader = AsciiReader::new(input)?;
        let picture: RobotPicture = reader.get_image()?.parse()?;
        let path = picture.gather()?;
        println!("{}", path);
        path.break_path(19)?;
        Ok(().into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not an Int")]
    ParseIntError(#[from] num::ParseIntError),
    #[error("Computer error: {0}")]
    ComputerError(#[from] ComputerError),
    #[error("Illegal tile: {0}")]
    IllegalTile(char),
    #[error("Empty Pictures are not allowed")]
    NoEmptyPicture,
    #[error("Pictures must be rectangular")]
    PictureMustBeRectangular,
    #[error("Not exactly one robot")]
    NotExactlyOneRobot,
    #[error("Steps for path must not be zero")]
    StepsMustNotBeZero,
    #[error("Illegal Turn: {0}")]
    NotAllowedTurn(Turn),
    #[error("Empty path is not allowed")]
    EmptyPathNotAllowed,
    #[error("Could not split path")]
    CouldNotSplitPath,
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Empty,
    Scaffold,
    Robot(Direction),
    Tumbling,
}

impl TryFrom<char> for Tile {
    type Error = DayError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Scaffold),
            '>' => Ok(Tile::Robot(Direction::East)),
            '^' => Ok(Tile::Robot(Direction::North)),
            '<' => Ok(Tile::Robot(Direction::West)),
            'v' => Ok(Tile::Robot(Direction::South)),
            'X' => Ok(Tile::Tumbling),

            _ => Err(DayError::IllegalTile(value)),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ' ',
                Tile::Scaffold => '#',
                Tile::Robot(Direction::East) => '>',
                Tile::Robot(Direction::North) => '^',
                Tile::Robot(Direction::West) => '<',
                Tile::Robot(Direction::South) => 'v',
                Tile::Tumbling => 'X',
            }
        )
    }
}

struct RobotPicture {
    pixels: Vec<Vec<Tile>>,
    robot: Pos2<usize>,
    direction: Direction,
}

impl Display for RobotPicture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.pixels.iter() {
            for tile in row.iter() {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl FromStr for RobotPicture {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pixels = s
            .trim()
            .lines()
            .map(|row| row.chars().map(|tile| tile.try_into()).try_collect())
            .try_collect()?;
        Self::new(pixels)
    }
}

impl RobotPicture {
    pub fn new(mut pixels: Vec<Vec<Tile>>) -> Result<Self, DayError> {
        if pixels.is_empty() || pixels[0].is_empty() {
            return Err(DayError::NoEmptyPicture);
        }
        if !pixels.iter().map(|row| row.len()).all_equal() {
            return Err(DayError::PictureMustBeRectangular);
        }
        let (robot, direction) = Self::find_robot(&pixels)?;
        pixels[robot.y()][robot.x()] = Tile::Scaffold;
        Ok(Self {
            pixels,
            robot,
            direction,
        })
    }

    pub fn find_robot(pixels: &[Vec<Tile>]) -> Result<(Pos2<usize>, Direction), DayError> {
        pixels
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    if let Tile::Robot(direction) = tile {
                        Some((Pos2::new(x, y), *direction))
                    } else {
                        None
                    }
                })
            })
            .exactly_one()
            .map_err(|_| DayError::NotExactlyOneRobot)
    }

    fn get_tile(&self, x: usize, y: usize) -> Option<&Tile> {
        self.pixels.get(y).and_then(|row| row.get(x))
    }

    pub fn crossings(&self) -> impl Iterator<Item = Pos2<usize>> + '_ {
        (1..self.pixels.len()).flat_map(move |y| {
            (1..self.pixels[0].len()).filter_map(move |x| {
                if self.pixels[y][x] == Tile::Scaffold
                    && matches!(self.get_tile(x - 1, y), Some(Tile::Scaffold))
                    && matches!(self.get_tile(x, y - 1), Some(Tile::Scaffold))
                    && matches!(self.get_tile(x + 1, y), Some(Tile::Scaffold))
                    && matches!(self.get_tile(x, y + 1), Some(Tile::Scaffold))
                {
                    Some(Pos2::new(x, y))
                } else {
                    None
                }
            })
        })
    }

    pub fn crossing_sum(&self) -> usize {
        self.crossings().map(|pos| pos.x() * pos.y()).sum()
    }

    fn check_turn(&self, pos: Pos2<usize>, next_direction: Direction) -> bool {
        if let Some(next_pos) = pos.check_add(next_direction) {
            if matches!(
                self.get_tile(next_pos.x(), next_pos.y()),
                Some(Tile::Scaffold)
            ) {
                return true;
            }
        }
        false
    }

    fn get_next_turn(&self, pos: Pos2<usize>, facing: Direction) -> Option<Turn> {
        if self.check_turn(pos, facing + Turn::Left) {
            Some(Turn::Left)
        } else if self.check_turn(pos, facing + Turn::Right) {
            Some(Turn::Right)
        } else {
            None
        }
    }

    fn next_step(&self, pos: Pos2<usize>, facing: Direction) -> Option<Pos2<usize>> {
        if let Some(next_pos) = pos.check_add(facing) {
            if matches!(
                self.get_tile(next_pos.x(), next_pos.y()),
                Some(Tile::Scaffold)
            ) {
                return Some(next_pos);
            }
        }
        None
    }

    pub fn gather(&self) -> Result<Path, DayError> {
        let mut facing = self.direction;
        let mut pos = self.robot;
        let mut path = Path::new();

        while let Some(turn) = self.get_next_turn(pos, facing) {
            facing = facing + turn;
            let mut steps = 0;
            while let Some(next_pos) = self.next_step(pos, facing) {
                pos = next_pos;
                steps += 1;
            }
            path.add(turn, steps)?;
        }

        if path.is_empty() {
            return Err(DayError::EmptyPathNotAllowed);
        }
        Ok(path)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Element {
    Left(usize),
    Right(usize),
}

impl Element {
    pub fn string_len(&self) -> usize {
        match self {
            Element::Left(size) | Element::Right(size) => {
                if size >= &10 {
                    4
                } else {
                    3
                }
            }
        }
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Element::Left(size) => write!(f, "L,{}", size),
            Element::Right(size) => write!(f, "R,{}", size),
        }
    }
}

struct PathFinder<'a> {
    orig: &'a Path,
    sub: Vec<(Path>,
}

impl<'a> PathFinder<'a> {
    pub fn new(orig: &'a Path) -> Self {
        PathFinder { orig, sub: vec![] }
    }

    pub fn new_start(&self) -> Self {

    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Path {
    path: Vec<Element>,
}

impl Path {
    fn new() -> Self {
        Self { path: vec![] }
    }

    pub fn add(&mut self, turn: Turn, steps: usize) -> Result<(), DayError> {
        if steps == 0 {
            return Err(DayError::StepsMustNotBeZero);
        }
        match turn {
            Turn::Left => self.path.push(Element::Left(steps)),
            Turn::Right => self.path.push(Element::Right(steps)),
            _ => return Err(DayError::NotAllowedTurn(turn)),
        };
        Ok(())
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }

    pub fn string_len(&self) -> usize {
        if self.path.is_empty() {
            return 0;
        }
        self.path
            .iter()
            .map(|element| element.string_len())
            .fold(self.path.len() - 1, Add::add)
    }

    pub fn pop_one(&mut self) {
        self.path.pop();
    }

    pub fn find_max_subpath(&self, start_at: usize, max_len: usize) -> Option<Path> {
        let mut sub = Path::new();
        let mut current = start_at;
        while let Some(element) = self.path.get(current) {
            sub.path.push(element.clone());
            if sub.string_len() > max_len {
                sub.pop_one();
                break;
            }
            current += 1;
        }
        if sub.is_empty() {
            None
        } else {
            Some(sub)
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.path.len()
    }

    pub fn find_repeats(&self, sub: &Path) -> Vec<usize> {
        if sub.len() > self.len() {
            return vec![];
        }
        (0..(self.len() - sub.len() + 1))
            .filter(|start| {
                let equal = sub
                    .path
                    .iter()
                    .zip(self.path[*start..].iter())
                    .all(|(a, b)| a == b);
                if equal {
                    println!("{}", start)
                }
                equal
            })
            .collect_vec()
    }

    pub fn break_path(&self, max_len: usize) -> Result<Vec<Path>, DayError> {
        let path_finder = PathFinder::new(self);
        let mut used_up = vec![false; self.path.len()];
        let Some(mut sub) = self.find_max_subpath(String::from("A"), 12, max_len) else {
            return Err(DayError::CouldNotSplitPath);
        };
        loop {
            let findings = self.find_repeats(&sub);
            if findings.len() > 1 {
                println!("{}", sub);
                println!("{:?}", findings);
                break;
            }
            sub.pop_one();
            if sub.is_empty() {
                return Err(DayError::CouldNotSplitPath);
            }
        }

        todo!()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = self
            .path
            .iter()
            .map(|element| format!("{}", element))
            .join(",");
        write!(f, "{}", path)
    }
}

struct AsciiReader {
    brain: IntCodeComputer,
}

impl AsciiReader {
    pub fn new(code: &str) -> Result<Self, DayError> {
        let brain = ComputerFactory::init(code)?.build();
        Ok(Self { brain })
    }

    pub fn get_image(&mut self) -> Result<String, DayError> {
        Ok(self.brain.expect_string()?)
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
        let expected = ResultType::Integer(76);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "input.txt")?;
        let expected = ResultType::Nothing;
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn analyze() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let picture: RobotPicture = input.parse()?;

        assert_eq!(picture.robot, Pos2::new(10, 6));
        assert_eq!(picture.direction, Direction::North);
        assert_eq!(picture.crossing_sum(), 76);

        Ok(())
    }
}
