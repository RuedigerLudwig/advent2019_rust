use crate::common::{direction::Direction, pos2::Pos2};

use super::{DayTrait, DayType, RResult};
use itertools::Itertools;
use std::{
    collections::{BinaryHeap, HashSet, VecDeque},
    str::FromStr,
};

const DAY_NUMBER: DayType = 18;

pub struct Day;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let map: Map = input.parse()?;
        let path = map.find_shortest_path()?;
        Ok(path.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let map: Map = input.parse()?;
        let map = map.expand()?;
        let path = map.find_shortest_path()?;
        Ok(path.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Unknown Tile: {0}")]
    UnknownTile(char),
    #[error("A must must aloways be Rectangle")]
    MapMustBeRectangle,
    #[error("An empty map is not allowed")]
    EmptyMapNotAllowed,
    #[error("The map has no entrance")]
    MapHasNoSingleEntrance,
    #[error("No Path found")]
    NoPathFound,
    #[error("Can't expand this Map")]
    CantExpandMap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Tile {
    Wall,
    Floor,
    Entrance(usize),
    Key(char),
    Door(char),
}

impl TryFrom<char> for Tile {
    type Error = DayError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Floor),
            '@' => Ok(Tile::Entrance(0)),
            'a'..='z' => Ok(Tile::Key(value)),
            'A'..='Z' => Ok(Tile::Door(value.to_ascii_lowercase())),
            _ => Err(DayError::UnknownTile(value)),
        }
    }
}

impl Tile {
    pub fn is_floor_like(&self) -> bool {
        matches!(self, Tile::Floor | Tile::Entrance(_))
    }

    pub fn is_poi(&self) -> bool {
        match self {
            Tile::Wall | Tile::Floor => false,
            Tile::Entrance(_) | Tile::Key(_) | Tile::Door(_) => true,
        }
    }

    #[allow(dead_code)]
    fn as_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::Entrance(num) => match num {
                0 => '@',
                1..=4 => ['1', '2', '3', '4'][*num - 1],
                _ => unreachable!(),
            },
            Tile::Key(key) => *key,
            Tile::Door(door) => door.to_ascii_uppercase(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Connection {
    Unknown,
    Direct(usize),
    Indirect(usize, String),
}

impl Connection {
    pub fn value(&self) -> Option<usize> {
        match self {
            Connection::Unknown => None,
            Connection::Direct(val) => Some(*val),
            Connection::Indirect(val, _) => Some(*val),
        }
    }

    pub fn is_set(&self) -> bool {
        !matches!(self, Connection::Unknown)
    }

    fn get_doors(&self) -> String {
        match self {
            Connection::Unknown | Connection::Direct(_) => String::from(""),
            Connection::Indirect(_, doors) => doors.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Distances {
    poi: Vec<Tile>,
    dist: Vec<Vec<Connection>>,
}

impl Distances {
    pub fn new(map: &Map) -> Self {
        let positions = map.gather_poi();
        let dist = (1..positions.len())
            .map(|l| vec![Connection::Unknown; l])
            .collect_vec();

        let poi = positions.iter().map(|(tile, _)| *tile).sorted().collect();

        let mut me = Self { poi, dist };

        for (from, pos) in positions {
            let distances = map.get_distances_for(pos);
            for (to, distance) in distances {
                me.set(from, to, Connection::Direct(distance));
            }
        }
        me.fill_indirect_connections();

        let first_door = me
            .poi
            .iter()
            .position(|t| matches!(t, Tile::Door(_)))
            .unwrap();
        me.poi = me.poi[0..first_door].to_vec();
        me.dist = me.dist[0..first_door - 1].to_vec();

        me
    }

    #[inline]
    fn set(&mut self, from: Tile, to: Tile, distance: Connection) {
        if let (Some(from), Some(to)) = (self.tile_index(from), self.tile_index(to)) {
            self.set_by_idx(from, to, distance);
        }
    }

    #[inline]
    fn set_by_idx(&mut self, from: usize, to: usize, distance: Connection) {
        assert!(from != to);
        self.dist[from.max(to) - 1][from.min(to)] = distance;
    }

    #[inline]
    pub fn get(&self, from: Tile, to: Tile) -> Connection {
        let (Some(from), Some(to)) = (self.tile_index(from), self.tile_index(to)) else {
            return Connection::Unknown;
        };
        self.get_by_idx(from, to)
    }

    #[inline]
    pub fn get_by_idx(&self, from: usize, to: usize) -> Connection {
        if from == to {
            Connection::Unknown
        } else {
            self.dist[from.max(to) - 1][from.min(to)].clone()
        }
    }

    fn tile_index(&self, tile: Tile) -> Option<usize> {
        self.poi.iter().position(|t| t == &tile)
    }

    fn fill_indirect_connections(&mut self) {
        let mut made_change = true;
        while made_change {
            made_change = false;
            for idx in 0..self.poi.len() {
                let tile = self.poi[idx];
                for idx2 in 0..self.poi.len() - 1 {
                    let con2 = &self.get_by_idx(idx, idx2);
                    if let Some(val1) = con2.value() {
                        let doors2 = con2.get_doors();
                        for idx3 in idx2 + 1..self.poi.len() {
                            let con3 = &self.get_by_idx(idx, idx3);
                            if !self.get_by_idx(idx2, idx3).is_set()
                                && let Some(val2) = con3.value()
                            {
                                let doors3 = con3.get_doors();
                                let mut doors = doors2.clone();
                                doors.push_str(&doors3);

                                if let Tile::Door(door_key) = tile {
                                    doors.push(door_key);
                                }
                                doors = doors.chars().sorted().collect();
                                self.set_by_idx(
                                    idx2,
                                    idx3,
                                    Connection::Indirect(val1 + val2, doors),
                                );
                                made_change = true;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn reachable_connections(&self, tile: Tile, keyring: &str) -> Option<Vec<Tile>> {
        let Some(idx) = self.tile_index(tile) else {
            return None;
        };
        Some(
            self.poi
                .iter()
                .enumerate()
                .filter(|(pos, _)| pos != &idx)
                .filter(|(_, tile)| match tile {
                    Tile::Key(key_name) => !keyring.contains(*key_name),
                    _ => false,
                })
                .map(|(pos, tile)| {
                    let connection = if pos < idx {
                        &self.dist[idx - 1][pos]
                    } else {
                        &self.dist[pos - 1][idx]
                    };
                    (tile, connection)
                })
                .filter_map(|(tile, connection)| match connection {
                    Connection::Unknown => None,
                    Connection::Direct(_) => Some(*tile),
                    Connection::Indirect(_, doors) => {
                        if doors.chars().all(|door_name| keyring.contains(door_name)) {
                            Some(*tile)
                        } else {
                            None
                        }
                    }
                })
                .sorted()
                .collect(),
        )
    }

    fn count_keys(&self) -> usize {
        self.poi
            .iter()
            .filter(|tile| matches!(tile, Tile::Key(_)))
            .count()
    }
}

#[derive(Debug, Clone)]
struct Player {
    position: Tile,
    reachable: Vec<Tile>,
}

impl Player {
    pub fn init(entrance: Tile, distances: &Distances) -> Result<Self, DayError> {
        let Some(reachable) = distances.reachable_connections(entrance, "") else {
            return Err(DayError::MapHasNoSingleEntrance);
        };

        Ok(Player {
            position: entrance,
            reachable,
        })
    }
}

#[derive(Debug, Clone)]
struct State<'a> {
    distances: &'a Distances,
    player: Vec<Player>,
    keyring: String,
    missing_keys: usize,
    steps: usize,
}

impl Eq for State<'_> {}

impl PartialEq for State<'_> {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.cmp(other), std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.steps.cmp(&other.steps).reverse() {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.missing_keys.cmp(&other.missing_keys).reverse() {
            std::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.keyring.cmp(&other.keyring)
    }
}

impl<'a> State<'a> {
    pub fn new_single(distances: &'a Distances) -> Result<Self, DayError> {
        let keyring = String::new();
        let missing_keys = distances.count_keys();

        let player = vec![Player::init(Tile::Entrance(0), distances)?];

        Ok(Self {
            distances,
            player,
            missing_keys,
            keyring,
            steps: 0,
        })
    }

    pub fn new_multi(distances: &'a Distances) -> Result<Self, DayError> {
        let keyring = String::new();
        let missing_keys = distances.count_keys();

        let player = (1..=4)
            .map(|num| Player::init(Tile::Entrance(num), distances))
            .try_collect()?;

        Ok(Self {
            distances,
            player,
            missing_keys,
            keyring,
            steps: 0,
        })
    }

    pub fn is_finished(&self) -> bool {
        self.missing_keys == 0
    }

    pub fn add_key(&self, key: Tile) -> Option<String> {
        if let Tile::Key(key_name) = key {
            if !self.keyring.contains(key_name) {
                let mut keyring = self.keyring.clone();
                keyring.push(key_name);
                keyring = keyring.chars().sorted().collect();
                return Some(keyring);
            }
        }
        None
    }

    pub fn move_to(&self, target: Tile) -> Option<Self> {
        let Some((idx, current)) = self
            .player
            .iter()
            .enumerate()
            .find(|(_, p)| p.reachable.contains(&target))
        else {
            return None;
        };

        let Some(keyring) = self.add_key(target) else {
            return None;
        };

        let steps = self.steps + self.distances.get(current.position, target).value()?;

        let player = self
            .player
            .iter()
            .enumerate()
            .map(|(pos, player)| {
                if pos != idx {
                    let reachable = self
                        .distances
                        .reachable_connections(player.position, &keyring)
                        .unwrap();
                    Player {
                        position: player.position,
                        reachable,
                    }
                } else {
                    let reachable = self
                        .distances
                        .reachable_connections(target, &keyring)
                        .unwrap();
                    Player {
                        position: target,
                        reachable,
                    }
                }
            })
            .collect();

        Some(State {
            distances: self.distances,
            player,
            keyring,
            missing_keys: self.missing_keys - 1,
            steps,
        })
    }

    pub fn fingerprint(&self) -> (Vec<Tile>, String) {
        (
            self.player.iter().map(|p| p.position).collect(),
            self.keyring.clone(),
        )
    }

    fn reachable(&self) -> impl Iterator<Item = &Tile> + '_ {
        self.player.iter().flat_map(|p| p.reachable.iter())
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    is_expanded: bool,
}

impl FromStr for Map {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(
            s.lines()
                .map(|row| row.chars().map(|tile| tile.try_into()).try_collect())
                .try_collect()?,
        )
    }
}

impl Map {
    pub fn new(tiles: Vec<Vec<Tile>>) -> Result<Self, DayError> {
        if tiles.is_empty() || tiles[0].is_empty() {
            return Err(DayError::EmptyMapNotAllowed);
        }
        if !tiles.iter().map(|row| row.len()).all_equal() {
            return Err(DayError::MapMustBeRectangle);
        }
        Ok(Self {
            tiles,
            is_expanded: false,
        })
    }

    pub fn expand(mut self) -> Result<Self, DayError> {
        let entrance = self.find_single_entrance()?;
        if !(0..self.tiles.len() - 1).contains(&entrance.y())
            || !(0..self.tiles[0].len() - 1).contains(&entrance.x())
        {
            return Err(DayError::CantExpandMap);
        }
        for dy in 0..=2 {
            for dx in 0..=2 {
                if !self.tiles[entrance.y() + dy - 1][entrance.x() + dx - 1].is_floor_like() {
                    return Err(DayError::CantExpandMap);
                }
            }
        }

        self.tiles[entrance.y() - 1][entrance.x() - 1] = Tile::Entrance(1);
        self.tiles[entrance.y() - 1][entrance.x()] = Tile::Wall;
        self.tiles[entrance.y() - 1][entrance.x() + 1] = Tile::Entrance(2);
        self.tiles[entrance.y()][entrance.x() - 1] = Tile::Wall;
        self.tiles[entrance.y()][entrance.x()] = Tile::Wall;
        self.tiles[entrance.y()][entrance.x() + 1] = Tile::Wall;
        self.tiles[entrance.y() + 1][entrance.x() - 1] = Tile::Entrance(3);
        self.tiles[entrance.y() + 1][entrance.x()] = Tile::Wall;
        self.tiles[entrance.y() + 1][entrance.x() + 1] = Tile::Entrance(4);
        self.is_expanded = true;

        Ok(self)
    }

    fn find_single_entrance(&self) -> Result<Pos2<usize>, DayError> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    if matches!(tile, Tile::Entrance(0)) {
                        Some(Pos2::new(x, y))
                    } else {
                        None
                    }
                })
            })
            .exactly_one()
            .map_err(|_| DayError::MapHasNoSingleEntrance)
    }

    pub fn gather_poi(&self) -> Vec<(Tile, Pos2<usize>)> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    if tile.is_poi() {
                        Some((*tile, Pos2::new(x, y)))
                    } else {
                        None
                    }
                })
            })
            .collect_vec()
    }

    pub fn get_tile(&self, pos: Pos2<usize>) -> Tile {
        self.tiles
            .get(pos.y())
            .and_then(|row| row.get(pos.x()))
            .copied()
            .unwrap_or(Tile::Wall)
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

                let tile = self.get_tile(next_pos);
                if tile.is_poi() {
                    grid[next_pos.y()][next_pos.x()] = true;
                    distances.push((tile, dist + 1));
                } else if tile.is_floor_like() {
                    grid[next_pos.y()][next_pos.x()] = true;
                    queue.push_back((next_pos, dist + 1))
                }
            }
        }

        distances
    }

    pub fn find_shortest_path(&self) -> Result<usize, DayError> {
        let distances = Distances::new(self);
        let state = if self.is_expanded {
            State::new_multi(&distances)?
        } else {
            State::new_single(&distances)?
        };
        let mut seen = HashSet::new();
        let mut queue = BinaryHeap::new();
        queue.push(state);
        while let Some(current) = queue.pop() {
            if current.is_finished() {
                return Ok(current.steps);
            }
            let fingerprint = current.fingerprint();
            if seen.contains(&fingerprint) {
                continue;
            }
            seen.insert(fingerprint);
            for tile in current.reachable() {
                if let Some(next) = current.move_to(*tile) {
                    queue.push(next);
                }
            }
        }

        Err(DayError::NoPathFound)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::{read_string, ResultType, UnitResult};

    #[test]
    fn test_part1() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example03.txt")?;
        let expected = ResultType::Integer(81);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example05.txt")?;
        let expected = ResultType::Integer(72);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn parse() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;

        let map: Map = input.parse()?;
        assert_eq!(
            map.gather_poi(),
            vec![
                (Tile::Key('b'), Pos2::new(1, 1)),
                (Tile::Door('a'), Pos2::new(3, 1)),
                (Tile::Entrance(0), Pos2::new(5, 1)),
                (Tile::Key('a'), Pos2::new(7, 1)),
            ]
        );

        Ok(())
    }

    #[test]
    fn distances_and_move() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let map: Map = input.parse()?;

        let distances = Distances::new(&map);
        assert_eq!(
            distances,
            Distances {
                poi: vec![Tile::Entrance(0), Tile::Key('a'), Tile::Key('b'),],
                dist: vec![
                    vec![Connection::Direct(2)],
                    vec![
                        Connection::Indirect(4, String::from("a")),
                        Connection::Indirect(6, String::from("a"))
                    ],
                ]
            }
        );
        assert_eq!(
            distances
                .reachable_connections(Tile::Entrance(0), "")
                .unwrap(),
            [Tile::Key('a')]
        );

        let player = State::new_single(&distances)?;

        let player = player.move_to(Tile::Key('a')).unwrap();
        assert_eq!(player.steps, 2);
        assert_eq!(player.reachable().copied().collect_vec(), [Tile::Key('b')]);
        assert_eq!(player.keyring, String::from("a"));

        let player = player.move_to(Tile::Key('b')).unwrap();
        assert!(player.is_finished());
        assert_eq!(player.steps, 8);
        assert_eq!(player.reachable().copied().collect_vec(), []);
        assert_eq!(player.keyring, String::from("ab"));

        Ok(())
    }

    #[test]
    fn shortest_example01() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let map: Map = input.parse()?;

        let path = map.find_shortest_path()?;
        assert_eq!(path, 8);

        Ok(())
    }

    #[test]
    fn shortest_example02() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;
        let map: Map = input.parse()?;

        let path = map.find_shortest_path()?;
        assert_eq!(path, 136);

        Ok(())
    }

    #[test]
    fn shortest_example03() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example03.txt")?;
        let map: Map = input.parse()?;

        let path = map.find_shortest_path()?;
        assert_eq!(path, 81);

        Ok(())
    }

    #[test]
    fn distances_and_move_expended() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example04.txt")?;
        let map: Map = input.parse()?;
        let map = map.expand()?;

        let distances = Distances::new(&map);
        assert_eq!(
            distances,
            Distances {
                poi: vec![
                    Tile::Entrance(1),
                    Tile::Entrance(2),
                    Tile::Entrance(3),
                    Tile::Entrance(4),
                    Tile::Key('a'),
                    Tile::Key('b'),
                    Tile::Key('c'),
                    Tile::Key('d'),
                ],
                dist: vec![
                    vec![Connection::Unknown],
                    vec![Connection::Unknown, Connection::Unknown],
                    vec![
                        Connection::Unknown,
                        Connection::Unknown,
                        Connection::Unknown
                    ],
                    vec![
                        Connection::Direct(2),
                        Connection::Unknown,
                        Connection::Unknown,
                        Connection::Unknown
                    ],
                    vec![
                        Connection::Unknown,
                        Connection::Unknown,
                        Connection::Unknown,
                        Connection::Indirect(2, String::from("a")),
                        Connection::Unknown,
                    ],
                    vec![
                        Connection::Unknown,
                        Connection::Unknown,
                        Connection::Indirect(2, String::from("b")),
                        Connection::Unknown,
                        Connection::Unknown,
                        Connection::Unknown,
                    ],
                    vec![
                        Connection::Unknown,
                        Connection::Indirect(2, String::from("c")),
                        Connection::Unknown,
                        Connection::Unknown,
                        Connection::Unknown,
                        Connection::Unknown,
                        Connection::Unknown,
                    ]
                ]
            }
        );
        assert_eq!(
            distances
                .reachable_connections(Tile::Entrance(1), "")
                .unwrap(),
            [Tile::Key('a')]
        );
        assert_eq!(
            distances
                .reachable_connections(Tile::Entrance(2), "")
                .unwrap(),
            []
        );

        assert_eq!(
            distances
                .reachable_connections(Tile::Entrance(2), "c")
                .unwrap(),
            [Tile::Key('d')]
        );

        let state = State::new_multi(&distances)?;

        let state = state.move_to(Tile::Key('a')).unwrap();
        assert_eq!(state.steps, 2);
        assert_eq!(state.reachable().copied().collect_vec(), [Tile::Key('b')]);
        assert_eq!(state.keyring, String::from("a"));

        let state = state.move_to(Tile::Key('b')).unwrap();
        assert_eq!(state.steps, 4);
        assert_eq!(state.reachable().copied().collect_vec(), [Tile::Key('c')]);
        assert_eq!(state.keyring, String::from("ab"));

        let state = state.move_to(Tile::Key('c')).unwrap();
        assert_eq!(state.steps, 6);
        assert_eq!(state.reachable().copied().collect_vec(), [Tile::Key('d')]);
        assert_eq!(state.keyring, String::from("abc"));

        let state = state.move_to(Tile::Key('d')).unwrap();
        assert!(state.is_finished());
        assert_eq!(state.steps, 8);
        assert_eq!(state.reachable().copied().collect_vec(), []);
        assert_eq!(state.keyring, String::from("abcd"));

        Ok(())
    }
}
