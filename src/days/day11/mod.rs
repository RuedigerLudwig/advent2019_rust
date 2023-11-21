use crate::{
    common::{area::Area, direction::Direction, pos2::Pos2, turn::Turn},
    int_code::{ComputerError, ComputerFactory, IntCodeComputer},
};

use super::{DayTrait, DayType, RResult};
use std::collections::HashMap;

const DAY_NUMBER: DayType = 11;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let mut robby = Robot::new(input)?;
        robby.run(false)?;
        Ok(robby.get_touched_tiles().into())
    }

    fn part2(&self, input: &str) -> RResult {
        let mut robby = Robot::new(input)?;
        robby.run(true)?;
        Ok(robby.get_picture().into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Computer Error")]
    ComputerError(#[from] ComputerError),
}

struct Robot {
    brain: IntCodeComputer,
    tiles: HashMap<Pos2<i64>, bool>,
}

impl Robot {
    pub fn new(code: &str) -> Result<Self, DayError> {
        Ok(Self {
            brain: ComputerFactory::init(code)?.build(),
            tiles: HashMap::new(),
        })
    }

    pub fn run(&mut self, starting_color: bool) -> Result<(), DayError> {
        let mut pos = Pos2::splat(0);
        let mut facing = Direction::North;
        self.tiles.insert(pos, starting_color);
        self.brain.send_bool(starting_color);
        while let Some(color) = self.brain.maybe_bool()? {
            self.tiles.insert(pos, color);
            let turn_right = self.brain.expect_bool()?;
            facing = facing + if turn_right { Turn::Right } else { Turn::Left };
            pos += facing;
            let color = self.tiles.get(&pos).copied().unwrap_or(false);
            self.brain.send_bool(color);
        }
        Ok(())
    }

    pub fn get_touched_tiles(&self) -> usize {
        self.tiles.len()
    }

    pub fn get_picture(&self) -> Vec<Vec<bool>> {
        let Some(area) = Area::from_iterator(self.tiles.keys()) else {
            return vec![vec![]];
        };
        self.tiles.iter().fold(
            vec![vec![false; area.width() as usize]; area.height() as usize],
            |mut picture, (pos, color)| {
                if *color {
                    picture[(pos.y() - area.bottom()) as usize][(pos.x() - area.left()) as usize] =
                        true
                };
                picture
            },
        )
    }
}
