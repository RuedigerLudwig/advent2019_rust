#![allow(dead_code)]
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Idx(usize);

impl Idx {
    #[inline]
    pub fn new(value: usize) -> Self {
        Idx(value)
    }

    #[inline]
    pub fn get(&self) -> usize {
        self.0
    }

    #[inline]
    pub fn abs_diff(self, other: Idx) -> Idx {
        if self > other {
            Idx(self.0 - other.0)
        } else {
            Idx(other.0 - self.0)
        }
    }
}

impl From<usize> for Idx {
    #[inline]
    fn from(value: usize) -> Self {
        Idx(value)
    }
}

impl<I> Add<I> for Idx
where
    I: Into<Idx>,
{
    type Output = Idx;

    #[inline]
    fn add(self, rhs: I) -> Self::Output {
        Idx(self.0 + rhs.into().0)
    }
}

impl<I> AddAssign<I> for Idx
where
    I: Into<Idx>,
{
    #[inline]
    fn add_assign(&mut self, rhs: I) {
        self.0 += rhs.into().0
    }
}

impl<I> Sub<I> for Idx
where
    I: Into<Idx>,
{
    type Output = Idx;

    #[inline]
    fn sub(self, rhs: I) -> Self::Output {
        Idx(self.0 - rhs.into().0)
    }
}

impl<I> SubAssign<I> for Idx
where
    I: Into<Idx>,
{
    #[inline]
    fn sub_assign(&mut self, rhs: I) {
        self.0 -= rhs.into().0
    }
}

impl<I> Mul<I> for Idx
where
    I: Into<Idx>,
{
    type Output = Idx;

    #[inline]
    fn mul(self, rhs: I) -> Self::Output {
        Idx(self.0 * rhs.into().0)
    }
}

impl<I> MulAssign<I> for Idx
where
    I: Into<Idx>,
{
    #[inline]
    fn mul_assign(&mut self, rhs: I) {
        self.0 *= rhs.into().0
    }
}

impl<I> Div<I> for Idx
where
    I: Into<Idx>,
{
    type Output = Idx;

    #[inline]
    fn div(self, rhs: I) -> Self::Output {
        Idx(self.0 / rhs.into().0)
    }
}

impl<I> DivAssign<I> for Idx
where
    I: Into<Idx>,
{
    #[inline]
    fn div_assign(&mut self, rhs: I) {
        self.0 /= rhs.into().0
    }
}

pub struct Stepper {
    current: Option<Idx>,
    target: Idx,
}

impl Stepper {
    pub fn new<I, J>(start: I, target: J) -> Self
    where
        I: Into<Idx>,
        J: Into<Idx>,
    {
        Stepper {
            current: Some(start.into()),
            target: target.into(),
        }
    }
}

impl Iterator for Stepper {
    type Item = Idx;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(current) = self.current else {
            return None;
        };
        match current.cmp(&self.target) {
            std::cmp::Ordering::Equal => self.current = None,
            std::cmp::Ordering::Less => self.current = Some(current + 1),
            std::cmp::Ordering::Greater => self.current = Some(current - 1),
        }
        Some(current)
    }
}
