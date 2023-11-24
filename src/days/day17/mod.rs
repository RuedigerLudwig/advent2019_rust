use super::{DayTrait, DayType, RResult};
use crate::{
    common::{direction::Direction, pos2::Pos2, turn::Turn},
    int_code::{ComputerError, ComputerFactory, IntCodeComputer, Pointer},
};
use itertools::Itertools;
use std::{fmt::Display, num, ops::Add, str::FromStr};

const DAY_NUMBER: DayType = 17;
const MAX_LEN: usize = 20;
const MAX_DEPTH: usize = 3;
const SHOW_OUTPUT: bool = false;

fn maybe_print(output: &str) {
    if SHOW_OUTPUT {
        println!("{}", output);
    }
}

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let mut reader = AsciiBrain::new(input)?;
        let picture: RobotPicture = reader.get_image()?.parse()?;
        Ok(picture.crossing_sum().into())
    }

    fn part2(&self, input: &str) -> RResult {
        let mut ascii_brain = AsciiBrain::new(input)?;
        let picture: RobotPicture = ascii_brain.get_image()?.parse()?;
        let path = picture.determine_path()?;
        let parts = path.break_up_path()?;
        let result = ascii_brain.feed_input(parts)?;
        Ok(result.into())
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
    #[error("No Path Found")]
    NoPathFound,
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

    pub fn determine_path(&self) -> Result<Path, DayError> {
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

#[derive(Debug)]
struct PathFinder<'a> {
    orig: &'a Path,
    sub: Vec<(Path, Vec<usize>)>,
    free_positions: Vec<bool>,
}

impl Display for PathFinder<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (sub, pos) in self.sub.iter() {
            writeln!(f, "{}: {:?}", sub, pos)?;
        }
        writeln!(f, "{:?}", self.free_positions)
    }
}

impl<'a> PathFinder<'a> {
    pub fn min_output_len(&self) -> usize {
        let items: usize = self.sub.iter().map(|(_, pos)| pos.len()).sum();
        if items == 0 {
            0
        } else {
            items * 2 - 1
        }
    }

    pub fn new(orig: &'a Path) -> Self {
        PathFinder {
            orig,
            sub: vec![],
            free_positions: vec![true; orig.len()],
        }
    }

    fn first_free_position(&self) -> Option<usize> {
        self.free_positions.iter().position(|free| *free)
    }

    fn add_sub(&self, new_sub: Path, positions: Vec<usize>) -> Option<Self> {
        if self.sub.len() >= MAX_DEPTH {
            return None;
        }
        let mut free_positions = self.free_positions.clone();
        for start in positions.iter() {
            let end = start + new_sub.len();
            if !free_positions[*start..end].iter().all(|item| *item) {
                return None;
            }
            free_positions[*start..end]
                .iter_mut()
                .for_each(|item| *item = false);
        }
        let mut sub = self.sub.clone();
        sub.push((new_sub, positions));
        let candidate = Self {
            orig: self.orig,
            sub,
            free_positions,
        };
        if candidate.min_output_len() < MAX_LEN {
            Some(candidate)
        } else {
            None
        }
    }

    pub fn is_finished(&self) -> bool {
        self.free_positions.iter().all(|free| !free)
    }

    fn add_repeats(&self, sub: Path) -> Vec<Self> {
        let repeats = self.orig.find_repeats(&sub);

        repeats
            .into_iter()
            .powerset()
            .filter_map(|positions| self.add_sub(sub.clone(), positions))
            .collect_vec()
    }

    fn check_reduce(mut self) -> Vec<Self> {
        let (curr, _) = self.sub.pop().unwrap();
        let Some(next_sub) = curr.reduce_by_one() else {
            return vec![];
        };
        self.add_repeats(next_sub)
    }

    pub fn next_sub(self) -> Vec<Self> {
        if let Some((_, pos)) = self.sub.last() {
            if pos.is_empty() {
                return self.check_reduce();
            }
        }

        let Some(first_free) = self.first_free_position() else {
            return vec![];
        };
        let Some(sub) = self.orig.find_max_subpath(first_free) else {
            return vec![];
        };
        self.add_repeats(sub)
    }

    fn get_order(&self) -> String {
        self.sub
            .iter()
            .enumerate()
            .fold(
                vec![None; self.orig.len()],
                |mut lst, (idx, (_, positions))| {
                    positions.iter().for_each(|start| {
                        lst[*start] = Some(idx);
                    });
                    lst
                },
            )
            .into_iter()
            .flatten()
            .map(|c| (c as u8 + b'A') as char)
            .join(",")
    }

    fn get_strings(&self) -> Vec<String> {
        if !self.is_finished() {
            vec![]
        } else {
            std::iter::once(self.get_order())
                .chain(self.sub.iter().map(|(sub, _)| format!("{}", sub)))
                .collect_vec()
        }
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

    pub fn reduce_by_one(&self) -> Option<Self> {
        if self.len() > 1 {
            let mut path = self.path.clone();
            path.pop();
            Some(Self { path })
        } else {
            None
        }
    }

    pub fn find_max_subpath(&self, start_at: usize) -> Option<Path> {
        let mut sub = Path::new();
        let mut current = start_at;
        while let Some(element) = self.path.get(current) {
            sub.path.push(*element);
            if sub.string_len() > MAX_LEN {
                return sub.reduce_by_one();
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
                sub.path
                    .iter()
                    .zip(self.path[*start..].iter())
                    .all(|(a, b)| a == b)
            })
            .collect_vec()
    }

    pub fn break_up_path(&self) -> Result<Vec<String>, DayError> {
        let pf = PathFinder::new(self);
        let mut queue = vec![pf];
        while let Some(current) = queue.pop() {
            if current.is_finished() {
                return Ok(current.get_strings());
            }
            queue.append(&mut current.next_sub())
        }
        Err(DayError::NoPathFound)
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

struct AsciiBrain {
    brain: IntCodeComputer,
}

impl AsciiBrain {
    pub fn new(code: &str) -> Result<Self, DayError> {
        let brain = ComputerFactory::init(code)?.build();
        Ok(Self { brain })
    }

    pub fn get_image(&mut self) -> Result<String, DayError> {
        Ok(std::iter::from_fn(|| self.brain.maybe_string().transpose())
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .join("\n"))
    }

    fn receive_and_send(&mut self, to_send: &str) -> Result<(), DayError> {
        maybe_print(&self.brain.expect_string_()?);
        maybe_print(to_send);
        self.brain.send_string(to_send);
        Ok(())
    }

    fn animate(&mut self) -> Result<(), DayError> {
        self.receive_and_send("n")?;
        maybe_print(&self.get_image()?);

        Ok(())
    }

    pub fn feed_input(&mut self, input: Vec<String>) -> Result<i64, DayError> {
        self.brain.manipulate_memory(Pointer::new(0), 2);

        for line in input {
            self.receive_and_send(&line)?;
        }
        self.animate()?;

        Ok(self.brain.expect_i64()?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::{read_string, UnitResult};

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
