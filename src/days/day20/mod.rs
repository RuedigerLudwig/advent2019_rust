#![allow(dead_code)]

use super::{DayTrait, DayType, RResult};
use crate::common::{
    direction::Direction,
    path_finder::{find_best_path, FingerprintItem, FingerprintSkipper, PathFinder},
    pos2::Pos2,
};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, VecDeque},
    num,
    str::FromStr,
};

const DAY_NUMBER: DayType = 20;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let raw_map: RawMap = input.parse()?;
        let tile_map = raw_map.to_tile_map()?;
        let result = tile_map.find_shortest_path()?;

        Ok(result.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let raw_map: RawMap = input.parse()?;
        let tile_map = raw_map.to_tile_map()?;
        let result = tile_map.find_shortest_recursive_path()?;

        Ok(result.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not a valid description: {0}")]
    ParseError(String),
    #[error("Not an Int")]
    ParseIntError(#[from] num::ParseIntError),
    #[error("Unknown Tile: {0}")]
    UnknownTile(char),
    #[error("Door must have exactly two characters")]
    DoorMustHaveTwoChars,
    #[error("Not all doors have partners")]
    NotAllDoorsHavePartners,
    #[error("Maze has no entrance")]
    MazeHasNoEntrance,
    #[error("No path found")]
    NoPathFound,
}

#[derive(Debug, PartialEq, Eq)]
enum RawTile {
    Inpenetrable,
    Floor,
    DoorName(char),
}

impl TryFrom<char> for RawTile {
    type Error = DayError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' | ' ' => Ok(RawTile::Inpenetrable),
            '.' => Ok(RawTile::Floor),
            'A'..='Z' => Ok(RawTile::DoorName(value)),
            _ => Err(DayError::UnknownTile(value)),
        }
    }
}

#[derive(Debug)]
struct RawMap {
    map: Vec<Vec<RawTile>>,
    width: usize,
    height: usize,
}
impl RawMap {
    fn get(&self, pos: &Pos2<usize>) -> Option<&RawTile> {
        self.map.get(pos.y()).and_then(|row| row.get(pos.x()))
    }

    fn check_direction(
        &self,
        start: Pos2<usize>,
        direction: Direction,
    ) -> Result<Option<(char, char)>, DayError> {
        let Some(step) = start.check_add(direction) else {
            return Ok(None);
        };
        let Some(RawTile::DoorName(door_name_1)) = self.get(&step) else {
            return Ok(None);
        };
        let Some(step) = step.check_add(direction) else {
            return Err(DayError::DoorMustHaveTwoChars);
        };
        let Some(RawTile::DoorName(door_name_2)) = self.get(&step) else {
            return Err(DayError::DoorMustHaveTwoChars);
        };
        match direction {
            Direction::East | Direction::South => Ok(Some((*door_name_1, *door_name_2))),
            Direction::West | Direction::North => Ok(Some((*door_name_2, *door_name_1))),
        }
    }

    fn check_door(&self, maybe_door: Pos2<usize>) -> Result<Tile, DayError> {
        let mut doorsnames: Vec<_> = Direction::iter()
            .map(|dir| self.check_direction(maybe_door, dir))
            .filter_map_ok(|x| x)
            .try_collect()?;

        match doorsnames.len() {
            0 => return Ok(Tile::Floor),
            1 => {}
            _ => return Err(DayError::DoorMustHaveTwoChars),
        }
        let (d1, d2) = doorsnames.pop().unwrap();

        let is_inner = (3..self.width - 3).contains(&maybe_door.x())
            && (3..self.height - 3).contains(&maybe_door.y());
        Ok(Tile::create_door(d1, d2, is_inner))
    }

    pub fn to_tile_map(&self) -> Result<TileMap, DayError> {
        let tiles = self.map[2..self.height - 2]
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let row_len = row.len();
                row[2..row_len.min(self.width - 2)]
                    .iter()
                    .enumerate()
                    .map(|(x, tile)| match tile {
                        RawTile::Inpenetrable | RawTile::DoorName(_) => Ok(Tile::Inpenetrable),
                        RawTile::Floor => self.check_door(Pos2::new(x + 2, y + 2)),
                    })
                    .try_collect()
            })
            .try_collect()?;

        TileMap::new(tiles)
    }
}

impl FromStr for RawMap {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Vec<Vec<_>> = s
            .lines()
            .map(|line| line.chars().map(RawTile::try_from).try_collect())
            .try_collect()?;
        let height = map.len();
        if height == 0 {
            return Err(DayError::ParseError(s.to_owned()));
        }
        let width = map.iter().map(|row| row.len()).max().unwrap();
        Ok(Self { map, width, height })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Default)]
enum Tile {
    #[default]
    Inpenetrable,
    Floor,
    Entrance,
    Exit,
    InnerDoor(char, char),
    OuterDoor(char, char),
}

impl Tile {
    pub fn create_door(d1: char, d2: char, is_inner: bool) -> Tile {
        match (d1, d2, is_inner) {
            ('A', 'A', _) => Tile::Entrance,
            ('Z', 'Z', _) => Tile::Exit,
            (_, _, true) => Tile::InnerDoor(d1, d2),
            (_, _, false) => Tile::OuterDoor(d1, d2),
        }
    }

    fn is_door(&self) -> bool {
        match self {
            Tile::Inpenetrable | Tile::Floor => false,
            Tile::Entrance | Tile::Exit | Tile::InnerDoor(_, _) | Tile::OuterDoor(_, _) => true,
        }
    }

    fn is_partner(&self, other: &Tile) -> bool {
        match (self, other) {
            (Tile::Entrance, Tile::Exit) | (Tile::Exit, Tile::Entrance) => true,
            (Tile::InnerDoor(i1, i2), Tile::OuterDoor(o1, o2))
            | (Tile::OuterDoor(o1, o2), Tile::InnerDoor(i1, i2)) => i1 == o1 && i2 == o2,
            _ => false,
        }
    }

    fn is_floor(&self) -> bool {
        self == &Tile::Floor
    }

    fn wrap(&self) -> Tile {
        match self {
            Tile::Inpenetrable | Tile::Floor | Tile::Entrance | Tile::Exit => *self,
            Tile::InnerDoor(d1, d2) => Tile::OuterDoor(*d1, *d2),
            Tile::OuterDoor(d1, d2) => Tile::InnerDoor(*d1, *d2),
        }
    }
}

struct TileMap {
    tiles: Vec<Vec<Tile>>,
}

impl TileMap {
    fn new(tiles: Vec<Vec<Tile>>) -> Result<Self, DayError> {
        let doors = tiles
            .iter()
            .flat_map(|row| row.iter().filter(|tile| tile.is_door()))
            .collect_vec();

        if !doors.contains(&&Tile::Entrance) {
            return Err(DayError::MazeHasNoEntrance);
        }

        let num_doors = doors.len();
        let partnered_doors = doors
            .into_iter()
            .permutations(2)
            .filter_map(|doors| {
                if doors[0].is_partner(doors[1]) {
                    Some(doors[0])
                } else {
                    None
                }
            })
            .collect_vec();
        if partnered_doors.len() != num_doors {
            return Err(DayError::NotAllDoorsHavePartners);
        }

        Ok(Self { tiles })
    }

    fn get(&self, pos: &Pos2<usize>) -> Option<&Tile> {
        self.tiles.get(pos.y()).and_then(|row| row.get(pos.x()))
    }

    fn get_distances_for(&self, start: Pos2<usize>) -> Vec<(Tile, usize)> {
        let mut distances = vec![];
        let mut grid = vec![vec![false; self.tiles[0].len()]; self.tiles.len()];
        grid[start.y()][start.x()] = true;
        let mut queue = VecDeque::new();
        queue.push_back((start, 0));
        while let Some((pos, dist)) = queue.pop_front() {
            for direction in Direction::iter() {
                let Some(next_pos) = pos.check_add(direction) else {
                    continue;
                };
                if grid
                    .get(next_pos.y())
                    .and_then(|row| row.get(next_pos.x()))
                    .copied()
                    .unwrap_or(true)
                {
                    continue;
                }

                let tile = self.get(&next_pos).unwrap_or(&Tile::Inpenetrable);
                if tile.is_door() {
                    grid[next_pos.y()][next_pos.x()] = true;
                    distances.push((*tile, dist + 1));
                } else if tile.is_floor() {
                    grid[next_pos.y()][next_pos.x()] = true;
                    queue.push_back((next_pos, dist + 1))
                }
            }
        }

        distances
    }

    pub fn find_shortest_path(&self) -> Result<usize, DayError> {
        let solver = MapSolver::new(self);
        find_best_path(solver)
            .map(|result| result.steps - 1)
            .ok_or(DayError::NoPathFound)
    }

    pub fn find_shortest_recursive_path(&self) -> Result<usize, DayError> {
        let solver = RecursiveMapSolver::new(self);
        find_best_path(solver)
            .map(|result| result.steps)
            .ok_or(DayError::NoPathFound)
    }
}

struct MapSolver {
    distances: Distances,
}

impl MapSolver {
    pub fn new(map: &TileMap) -> Self {
        Self {
            distances: Distances::new(map),
        }
    }
}

impl PathFinder for MapSolver {
    type Item = MapState;
    type Queue = BinaryHeap<MapState>;
    type Skipper = FingerprintSkipper<MapState>;

    fn get_start_item(&self) -> Self::Item {
        MapState::default()
    }

    #[inline]
    fn is_finished(&self, item: &Self::Item) -> bool {
        item.position == Tile::Exit
    }

    fn get_next_states<'a>(
        &'a self,
        item: &'a Self::Item,
    ) -> impl Iterator<Item = Self::Item> + 'a {
        self.distances
            .reachable_connections(item.position)
            .unwrap()
            .into_iter()
            .filter_map(move |tile| {
                self.distances
                    .get(item.position, tile)
                    .map(|steps| MapState {
                        steps: item.steps + steps + 1,
                        level: item.level,
                        position: tile.wrap(),
                    })
            })
    }
}

struct RecursiveMapSolver {
    distances: Distances,
}

impl RecursiveMapSolver {
    pub fn new(map: &TileMap) -> Self {
        Self {
            distances: Distances::new(map),
        }
    }
}

impl PathFinder for RecursiveMapSolver {
    type Item = MapState;
    type Queue = BinaryHeap<MapState>;
    type Skipper = FingerprintSkipper<Self::Item>;

    fn get_start_item(&self) -> Self::Item {
        MapState::default()
    }

    #[inline]
    fn is_finished(&self, item: &Self::Item) -> bool {
        item.position == Tile::Exit
    }

    fn get_next_states<'a>(
        &'a self,
        item: &'a Self::Item,
    ) -> impl Iterator<Item = Self::Item> + 'a {
        self.distances
            .reachable_connections(item.position)
            .unwrap()
            .into_iter()
            .filter_map(move |target| {
                self.distances
                    .get(item.position, target)
                    .and_then(|steps| item.walk_to(target, steps))
            })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct MapState {
    steps: usize,
    level: usize,
    position: Tile,
}

impl FingerprintItem for MapState {
    type Fingerprint = (Tile, usize);

    fn get_fingerprint(&self) -> Self::Fingerprint {
        (self.position, self.level)
    }
}

impl MapState {
    pub fn walk_to(&self, target: Tile, steps: usize) -> Option<Self> {
        match target {
            Tile::InnerDoor(_, _) => Some(Self {
                steps: self.steps + steps + 1,
                level: self.level + 1,
                position: target.wrap(),
            }),
            Tile::OuterDoor(_, _) => {
                if self.level == 0 {
                    None
                } else {
                    Some(Self {
                        steps: self.steps + steps + 1,
                        level: self.level - 1,
                        position: target.wrap(),
                    })
                }
            }
            Tile::Exit => {
                if self.level == 0 {
                    Some(Self {
                        steps: self.steps + steps,
                        level: 0,
                        position: Tile::Exit,
                    })
                } else {
                    None
                }
            }
            Tile::Inpenetrable | Tile::Floor | Tile::Entrance => None,
        }
    }
}

impl PartialOrd for MapState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MapState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.steps.cmp(&self.steps) {
            std::cmp::Ordering::Equal => {}
            cmp => return cmp,
        }
        self.level.cmp(&other.level)
    }
}

impl Default for MapState {
    fn default() -> Self {
        Self {
            steps: 0,
            level: 0,
            position: Tile::Entrance,
        }
    }
}

struct Distances {
    doors: Vec<Tile>,
    dist: Vec<Vec<Option<usize>>>,
}

impl Distances {
    pub fn new(map: &TileMap) -> Self {
        let positions = map
            .tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    if tile.is_door() {
                        Some((*tile, Pos2::new(x, y)))
                    } else {
                        None
                    }
                })
            })
            .collect_vec();

        let dist = (1..positions.len()).map(|l| vec![None; l]).collect_vec();

        let doors = positions.iter().map(|(tile, _)| *tile).sorted().collect();

        let mut me = Self { doors, dist };

        for (from, pos) in positions {
            let distances = map.get_distances_for(pos);
            for (to, distance) in distances {
                me.set(from, to, distance);
            }
        }

        me
    }

    #[inline]
    fn set(&mut self, from: Tile, to: Tile, distance: usize) {
        if let (Some(from), Some(to)) = (self.tile_index(from), self.tile_index(to)) {
            self.set_by_idx(from, to, distance);
        }
    }

    #[inline]
    fn set_by_idx(&mut self, from: usize, to: usize, distance: usize) {
        assert!(from != to);
        self.dist[from.max(to) - 1][from.min(to)] = Some(distance);
    }

    #[inline]
    pub fn get(&self, from: Tile, to: Tile) -> Option<usize> {
        self.get_by_idx(self.tile_index(from)?, self.tile_index(to)?)
    }

    #[inline]
    pub fn get_by_idx(&self, from: usize, to: usize) -> Option<usize> {
        if from == to {
            None
        } else {
            self.dist[from.max(to) - 1][from.min(to)]
        }
    }

    fn tile_index(&self, tile: Tile) -> Option<usize> {
        self.doors.iter().position(|t| t == &tile)
    }

    pub fn reachable_connections(&self, tile: Tile) -> Option<Vec<Tile>> {
        let Some(idx) = self.tile_index(tile) else {
            return None;
        };
        Some(
            self.doors
                .iter()
                .enumerate()
                .filter(|(pos, _)| pos != &idx)
                .filter(|(_, tile)| !matches!(tile, Tile::Entrance))
                .filter_map(|(pos, tile)| {
                    let connection = if pos < idx {
                        self.dist[idx - 1][pos]
                    } else {
                        self.dist[pos - 1][idx]
                    };
                    connection.map(|_| *tile)
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::{read_string, ResultType, UnitResult};

    #[test]
    fn test_part1() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;
        let expected = ResultType::Integer(58);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example03.txt")?;
        let expected = ResultType::Integer(396);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn parse() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let raw_map: RawMap = input.parse()?;
        let tile_map = raw_map.to_tile_map()?;

        assert_eq!(tile_map.get(&Pos2::new(7, 0)), Some(&Tile::Entrance));

        Ok(())
    }
}
