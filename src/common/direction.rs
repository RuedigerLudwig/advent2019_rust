use super::{pos2::Pos2, turn::Turn};
use num_traits::{Num, Signed};
use std::{fmt::Display, ops::Add};
use Direction::*;
use Turn::*;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub enum Direction {
    #[default]
    East = 0,
    North = 1,
    West = 2,
    South = 3,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Direction> {
        [
            Direction::East,
            Direction::North,
            Direction::West,
            Direction::South,
        ]
        .into_iter()
    }

    pub fn is_perpendicular(&self, other: &Direction) -> bool {
        match *self {
            East => *other != East && *other != West,
            North => *other != North && *other != South,
            West => *other != East && *other != West,
            South => *other != North && *other != South,
        }
    }

    pub fn get_turn(&self, toward: Direction) -> Turn {
        if *self == toward {
            Forward
        } else if toward == self.turn_left() {
            Left
        } else if toward == self.turn_right() {
            Right
        } else {
            Back
        }
    }

    pub fn turn(&self, turn: Turn) -> Direction {
        match turn {
            Left => self.turn_left(),
            Right => self.turn_right(),
            Back => self.turn_back(),
            Forward => *self,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match *self {
            East => South,
            North => East,
            West => North,
            South => West,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match *self {
            East => North,
            North => West,
            West => South,
            South => East,
        }
    }

    pub fn turn_back(&self) -> Direction {
        match *self {
            East => West,
            North => South,
            West => East,
            South => North,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Direction::East => write!(f, "East"),
            Direction::North => write!(f, "North"),
            Direction::West => write!(f, "West"),
            Direction::South => write!(f, "South"),
        }
    }
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value % 4 {
            0 => Direction::East,
            1 => Direction::North,
            2 => Direction::West,
            3 => Direction::South,
            _ => unreachable!(),
        }
    }
}

impl<T> From<Direction> for Pos2<T>
where
    T: Num + Signed,
{
    fn from(value: Direction) -> Self {
        match value {
            East => Pos2::new(T::one(), T::zero()),
            North => Pos2::new(T::zero(), -T::one()),
            West => Pos2::new(-T::one(), T::zero()),
            South => Pos2::new(T::zero(), T::one()),
        }
    }
}

impl Add<Direction> for Pos2<i32> {
    type Output = Pos2<i32>;

    fn add(self, rhs: Direction) -> Self::Output {
        let rhs: Pos2<i32> = rhs.into();
        self + rhs
    }
}

impl Add<Turn> for Direction {
    type Output = Self;

    fn add(self, rhs: Turn) -> Self {
        self.turn(rhs)
    }
}
