use crate::{
    common::pos2::Pos2,
    int_code::{ComputerError, ComputerFactory, IntCodeComputer, Pointer},
};

use super::{DayTrait, DayType, RResult};
use std::collections::HashMap;

const DAY_NUMBER: DayType = 13;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let brain = ComputerFactory::init(input)?.build();
        let game = Game::run(brain)?;
        Ok(game.blocks().into())
    }

    fn part2(&self, input: &str) -> RResult {
        let mut brain = ComputerFactory::init(input)?.build();
        brain.manipulate_memory(Pointer::new(0), 2);
        let result = Game::run(brain)?;
        Ok(result.score()?.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Computer error: {0}")]
    ComputerError(#[from] ComputerError),
    #[error("Unknown tile: [{0}")]
    UnknownTile(i64),
    #[error("There are still {0} blocks left")]
    StillBlocksLeft(usize),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Tile {
    #[default]
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i64> for Tile {
    type Error = DayError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::Paddle),
            4 => Ok(Tile::Ball),
            _ => Err(DayError::UnknownTile(value)),
        }
    }
}

struct Game {
    blocks: usize,
    score: i64,
}

const SCORE: (i64, i64) = (-1, 0);

impl Game {
    pub fn run(mut brain: IntCodeComputer) -> Result<Self, DayError> {
        let mut tiles = HashMap::new();
        let mut blocks = 0;
        let mut score = 0;
        let mut paddle_pos = None;
        while let Some(v) = brain.maybe_take_exactly(3)? {
            let [x, y, payload] = v[..] else {
                unreachable!();
            };
            if (x, y) == SCORE {
                score = payload;
                continue;
            }

            let tile = Tile::try_from(payload)?;
            match tile {
                Tile::Block => blocks += 1,
                Tile::Paddle => paddle_pos = Some(x),
                Tile::Ball => match paddle_pos {
                    Some(paddle_pos) if x > paddle_pos => brain.send_i64(1),
                    Some(_) => brain.send_i64(-1),
                    None => brain.send_i64(0),
                },
                _ => {}
            }

            let prev_tile = tiles.insert(Pos2::new(x, y), tile).unwrap_or_default();
            if matches!(prev_tile, Tile::Block) {
                blocks -= 1;
            }
        }

        Ok(Self { blocks, score })
    }

    pub fn blocks(&self) -> usize {
        self.blocks
    }

    pub fn score(&self) -> Result<i64, DayError> {
        if self.blocks != 0 {
            Err(DayError::StillBlocksLeft(self.blocks))
        } else {
            Ok(self.score)
        }
    }
}
