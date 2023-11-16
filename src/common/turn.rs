use super::direction::Direction;
use std::{fmt::Display, ops::Add};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    Forward,
    Left,
    Back,
    Right,
}

use Turn::*;

impl Turn {
    pub fn from_i64(value: i64) -> Turn {
        match value % 4 {
            0 => Turn::Forward,
            1 => Turn::Left,
            2 => Turn::Back,
            3 => Turn::Right,
            _ => unreachable!(),
        }
    }

    pub fn turn_left(&self) -> Turn {
        match *self {
            Left => Back,
            Back => Right,
            Right => Forward,
            Forward => Left,
        }
    }

    pub fn turn_right(&self) -> Turn {
        match *self {
            Left => Forward,
            Back => Left,
            Right => Back,
            Forward => Right,
        }
    }

    pub fn turn_back(&self) -> Turn {
        match *self {
            Left => Right,
            Back => Forward,
            Right => Left,
            Forward => Back,
        }
    }

    pub fn mirror(&self) -> Turn {
        match *self {
            Left => Right,
            Back => Back,
            Right => Left,
            Forward => Forward,
        }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Turn::Left => write!(f, "Left"),
            Turn::Right => write!(f, "Right"),
            Turn::Forward => write!(f, "Forward"),
            Turn::Back => write!(f, "Back"),
        }
    }
}

impl Add for Turn {
    type Output = Turn;

    fn add(self, rhs: Turn) -> Self::Output {
        match rhs {
            Left => self.turn_left(),
            Back => self.turn_back(),
            Right => self.turn_right(),
            Forward => self,
        }
    }
}

impl Add for &Turn {
    type Output = Turn;

    fn add(self, rhs: &Turn) -> Self::Output {
        Turn::add(*self, *rhs)
    }
}

impl Add<&Turn> for Turn {
    type Output = Turn;

    fn add(self, rhs: &Turn) -> Self::Output {
        Turn::add(self, *rhs)
    }
}

impl Add<Turn> for &Turn {
    type Output = Turn;

    fn add(self, rhs: Turn) -> Self::Output {
        Turn::add(*self, rhs)
    }
}

impl Add<Direction> for Turn {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output {
        rhs.turn(self)
    }
}

impl Add<Direction> for &Turn {
    type Output = Direction;

    fn add(self, rhs: Direction) -> Self::Output {
        rhs.turn(*self)
    }
}

impl Add<&Direction> for Turn {
    type Output = Direction;

    fn add(self, rhs: &Direction) -> Self::Output {
        rhs.turn(self)
    }
}

impl Add<&Direction> for &Turn {
    type Output = Direction;

    fn add(self, rhs: &Direction) -> Self::Output {
        rhs.turn(*self)
    }
}
