#![allow(dead_code)]
use std::{fs, io};

use itertools::Itertools;

#[allow(dead_code)]
#[derive(Debug, Default, PartialEq, Eq)]
pub enum ResultType {
    #[default]
    Nothing,
    Integer(i64),
    String(String),
    Lines(Vec<String>),
}

pub type RResult = anyhow::Result<ResultType>;
pub type UnitResult = anyhow::Result<()>;

impl From<&str> for ResultType {
    #[inline]
    fn from(value: &str) -> Self {
        ResultType::String(value.to_owned())
    }
}

impl From<String> for ResultType {
    #[inline]
    fn from(value: String) -> Self {
        ResultType::String(value)
    }
}

impl From<Vec<String>> for ResultType {
    #[inline]
    fn from(value: Vec<String>) -> Self {
        ResultType::Lines(value)
    }
}

impl From<Vec<Vec<bool>>> for ResultType {
    #[inline]
    fn from(lines: Vec<Vec<bool>>) -> Self {
        let lines = lines
            .into_iter()
            .map(|row| row.into_iter().map(|p| if p { '█' } else { ' ' }).join(""))
            .collect_vec();
        ResultType::Lines(lines)
    }
}

impl From<i32> for ResultType {
    #[inline]
    fn from(value: i32) -> Self {
        ResultType::Integer(value as i64)
    }
}

impl From<u32> for ResultType {
    #[inline]
    fn from(value: u32) -> Self {
        ResultType::Integer(value as i64)
    }
}

impl From<u64> for ResultType {
    #[inline]
    fn from(value: u64) -> Self {
        assert!(value < i64::MAX as u64);
        ResultType::Integer(value as i64)
    }
}

impl From<i64> for ResultType {
    #[inline]
    fn from(value: i64) -> Self {
        ResultType::Integer(value)
    }
}

impl From<usize> for ResultType {
    #[inline]
    fn from(value: usize) -> Self {
        assert!(value < i64::MAX as usize);
        ResultType::Integer(value as i64)
    }
}

impl From<()> for ResultType {
    fn from(_value: ()) -> Self {
        ResultType::Nothing
    }
}

pub type DayType = u8;
pub type PartType = u8;

pub trait DayTrait {
    fn get_day_number(&self) -> DayType;
    fn part1(&self, input: &str) -> RResult;
    fn part2(&self, input: &str) -> RResult;
}

fn format_path(day_num: DayType, file: &str) -> String {
    format!("data/day{day_num:02}/{file}")
}

pub fn read_string(day_num: DayType, file: &str) -> io::Result<String> {
    fs::read_to_string(format_path(day_num, file))
}
