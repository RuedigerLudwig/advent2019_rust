use super::{DayTrait, DayType, RResult};
use itertools::Itertools;
use std::{
    cell::Cell,
    collections::{hash_map::Entry, HashMap},
    num,
};

const DAY_NUMBER: DayType = 14;

pub struct Day;

const FUEL: &str = "FUEL";
const ORE: &str = "ORE";
const FREE_ORE: usize = 1_000_000_000_000;

impl DayTrait for Day {
    fn get_day_number(&self) -> DayType {
        DAY_NUMBER
    }

    fn part1(&self, input: &str) -> RResult {
        let recipe: Recipe = input.try_into()?;
        let amount = recipe.ore_per_fuel(1)?;
        Ok(amount.into())
    }

    fn part2(&self, input: &str) -> RResult {
        let recipe: Recipe = input.try_into()?;
        let amount = recipe.fuel_from_ore(FREE_ORE)?;
        Ok(amount.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum DayError {
    #[error("Not a valid description: {0}")]
    ParseError(String),
    #[error("Not an Int")]
    ParseIntError(#[from] num::ParseIntError),
    #[error("Could not resolve ore")]
    CouldNotResolveOre,
    #[error("No fuel in recipe")]
    NoFuelInRecipe,
    #[error("Unknown Ingredient: {0}")]
    UnknownIngredient(String),
}

#[derive(Debug)]
struct Reaction<'a> {
    name: &'a str,
    produced_amount: usize,
    ingredients: Vec<(&'a str, usize)>,
    level: Cell<Option<usize>>,
}

impl<'a> TryFrom<&'a str> for Reaction<'a> {
    type Error = DayError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        fn get_one(item: &str) -> Result<(&str, usize), self::DayError> {
            let Some((amount, ingredient)) = item.trim().split_once(' ') else {
                return Err(DayError::ParseError(item.to_owned()));
            };
            Ok((ingredient, amount.parse()?))
        }

        let Some((ingredients, result)) = value.split_once("=>") else {
            return Err(DayError::ParseError(value.to_owned()));
        };

        let (name, produced_amount) = get_one(result)?;
        let ingredients = ingredients.split(',').map(get_one).try_collect()?;

        Ok(Self {
            name,
            produced_amount,
            ingredients,
            level: Cell::new(None),
        })
    }
}

impl Reaction<'_> {
    pub fn level(&self) -> Option<usize> {
        self.level.get()
    }

    fn set_level(&self, level: usize) {
        self.level.set(Some(level));
    }
}

struct SortedHashMap<K, V>(HashMap<K, V>);

impl<K, V> SortedHashMap<K, V> {
    #[inline]
    pub fn new() -> Self {
        Self(HashMap::new())
    }
}

impl<K: std::hash::Hash + Eq + PartialEq, V> SortedHashMap<K, V> {
    #[inline]
    pub fn push(&mut self, k: K, v: V) -> Option<V> {
        self.0.insert(k, v)
    }

    #[inline]
    pub fn entry(&mut self, k: K) -> Entry<'_, K, V> {
        self.0.entry(k)
    }
}

impl<K: std::hash::Hash + Ord + Clone, V> SortedHashMap<K, V> {
    #[inline]
    pub fn pop_value(&mut self) -> Option<V> {
        let max = self.0.keys().max();
        if let Some(max) = max {
            self.0.remove(&max.clone())
        } else {
            None
        }
    }
}

struct InternalReactions<'a> {
    name: &'a str,
    index: usize,
    produced_amount: usize,
    level: usize,
    ingredients: Vec<(usize, usize)>,
}

impl<'a> InternalReactions<'a> {
    fn ore(name: &'a str, index: usize) -> Self {
        Self {
            name,
            index,
            produced_amount: 1,
            level: 1,
            ingredients: vec![],
        }
    }

    fn new(reaction: &Reaction<'a>, index: usize, names: &[&'a str]) -> Result<Self, DayError> {
        let ingredients = reaction
            .ingredients
            .iter()
            .map(|(ingredient, amount)| {
                if let Some(idx) = names.iter().position(|name| name == ingredient) {
                    Ok((idx, *amount))
                } else {
                    Err(DayError::UnknownIngredient(String::from(*ingredient)))
                }
            })
            .try_collect()?;
        let level = reaction.level().unwrap_or_default();

        Ok(Self {
            name: reaction.name,
            index,
            produced_amount: reaction.produced_amount,
            level,
            ingredients,
        })
    }
}

struct Recipe<'a> {
    reactions: Vec<InternalReactions<'a>>,
}

impl<'a> TryFrom<&'a str> for Recipe<'a> {
    type Error = DayError;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let reactions = value.lines().map(|line| line.try_into()).try_collect()?;
        Self::new(reactions)
    }
}

impl<'a> Recipe<'a> {
    pub fn new(reactions: Vec<Reaction<'a>>) -> Result<Self, DayError> {
        let names = std::iter::once(ORE)
            .chain(reactions.iter().map(|reaction| reaction.name))
            .collect_vec();

        if !names.contains(&FUEL) {
            return Err(DayError::NoFuelInRecipe);
        };
        let _ = Self::get_level_of(&reactions, FUEL);

        let reactions = std::iter::once(Ok(InternalReactions::ore(ORE, 0)))
            .chain(
                reactions
                    .iter()
                    .enumerate()
                    .map(|(pos, r)| InternalReactions::new(r, pos + 1, &names)),
            )
            .try_collect()?;
        Ok(Self { reactions })
    }
}

impl Recipe<'_> {
    fn get(&self, ingredient: &str) -> Option<&InternalReactions> {
        self.reactions.iter().find(|r| r.name == ingredient)
    }

    fn get_level_of(reactions: &[Reaction], ingredient: &str) -> usize {
        let Some(reaction) = reactions.iter().find(|r| r.name == ingredient) else {
            return 1;
        };
        if let Some(level) = reaction.level() {
            return level;
        }
        let level = 1 + reaction
            .ingredients
            .iter()
            .map(|(name, _)| Self::get_level_of(reactions, name))
            .max()
            .expect("max should never be empty at this point");

        reaction.set_level(level);

        level
    }

    pub fn ore_per_fuel(&self, amount: usize) -> Result<usize, DayError> {
        let fuel_reaction = self.get(FUEL).unwrap();
        let mut unfulfilled = SortedHashMap::new();
        unfulfilled.push(
            (fuel_reaction.level, fuel_reaction.index),
            (amount, fuel_reaction.index),
        );

        while let Some((required_amount, index)) = unfulfilled.pop_value() {
            if index == 0 {
                return Ok(required_amount);
            }
            let reaction = &self.reactions[index];
            let batches = required_amount.div_ceil(reaction.produced_amount);
            for (ingredient_idx, needed_amount) in reaction.ingredients.iter().copied() {
                let ingredient = &self.reactions[ingredient_idx];
                unfulfilled
                    .entry((ingredient.level, ingredient_idx))
                    .and_modify(|(amount, _)| *amount += needed_amount * batches)
                    .or_insert((needed_amount * batches, ingredient_idx));
            }
        }

        Err(DayError::CouldNotResolveOre)
    }

    fn fuel_from_ore(&self, free_ore: usize) -> Result<usize, DayError> {
        let ore_per_fuel = self.ore_per_fuel(1)?;
        let start = free_ore / ore_per_fuel;
        let mut current = start;
        let mut too_large = loop {
            let ore = self.ore_per_fuel(current)?;
            if ore > free_ore {
                break current;
            }
            current += start;
        };
        let mut too_small = too_large - start;
        while too_large > too_small + 1 {
            let current = (too_large + too_small) / 2;
            let ore = self.ore_per_fuel(current)?;
            if ore > free_ore {
                too_large = current;
            } else {
                too_small = current;
            }
        }
        Ok(too_small)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::{read_string, ResultType, UnitResult};

    #[test]
    fn test_part1() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example05.txt")?;
        let expected = ResultType::Integer(2210736);
        let result = day.part1(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn test_part2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example05.txt")?;
        let expected = ResultType::Integer(460664);
        let result = day.part2(&input)?;
        assert_eq!(result, expected);

        Ok(())
    }

    #[test]
    fn parse() -> UnitResult {
        let input = "7 A, 1 B => 1 C";
        let reaction: Reaction = input.try_into()?;

        assert_eq!(reaction.name, "C");
        assert_eq!(reaction.produced_amount, 1);

        Ok(())
    }

    #[test]
    fn example1() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example01.txt")?;
        let recipe: Recipe = input.as_str().try_into()?;

        let expected = 31;
        let ore = recipe.ore_per_fuel(1)?;

        assert_eq!(ore, expected);

        Ok(())
    }

    #[test]
    fn example2() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example02.txt")?;
        let recipe: Recipe = input.as_str().try_into()?;

        let expected = 165;
        let ore = recipe.ore_per_fuel(1)?;

        assert_eq!(ore, expected);

        Ok(())
    }

    #[test]
    fn example3() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example03.txt")?;
        let recipe: Recipe = input.as_str().try_into()?;

        let ore = recipe.ore_per_fuel(1)?;
        assert_eq!(ore, 13312);

        let fuel = recipe.fuel_from_ore(FREE_ORE)?;
        assert_eq!(fuel, 82892753);

        Ok(())
    }

    #[test]
    fn example4() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example04.txt")?;
        let recipe: Recipe = input.as_str().try_into()?;

        let ore = recipe.ore_per_fuel(1)?;
        assert_eq!(ore, 180697);

        let fuel = recipe.fuel_from_ore(FREE_ORE)?;
        assert_eq!(fuel, 5586022);

        Ok(())
    }

    #[test]
    fn example5() -> UnitResult {
        let day = Day {};
        let input = read_string(day.get_day_number(), "example05.txt")?;
        let recipe: Recipe = input.as_str().try_into()?;

        let ore = recipe.ore_per_fuel(1)?;
        assert_eq!(ore, 2210736);

        let fuel = recipe.fuel_from_ore(FREE_ORE)?;
        assert_eq!(fuel, 460664);

        Ok(())
    }
}
