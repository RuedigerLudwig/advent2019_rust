#![allow(dead_code)]
use super::pos3::Pos3;
use num_traits::{Num, Signed};
use std::fmt::Display;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Block<T>
where
    T: Num,
{
    lower: Pos3<T>,
    upper: Pos3<T>,
}

impl<T> Block<T>
where
    T: Num + Ord + Copy,
{
    pub fn new(p1: Pos3<T>, p2: Pos3<T>) -> Block<T> {
        Block {
            lower: p1.min_components(p2),
            upper: p1.max_components(p2),
        }
    }

    pub fn single(p1: Pos3<T>) -> Block<T> {
        Block {
            lower: p1,
            upper: p1,
        }
    }

    #[inline]
    fn intersection1(from1: T, to1: T, from2: T, to2: T) -> Option<(T, T)> {
        if to1 >= from2 && to2 >= from1 {
            Some((from1.max(from2), to1.min(to2)))
        } else {
            None
        }
    }

    pub fn within(self, other: Block<T>) -> bool {
        self.intersection(other) == Some(self)
    }

    pub fn intersection(self, other: Block<T>) -> Option<Block<T>> {
        let x = Block::intersection1(
            self.lower.x(),
            self.upper.x(),
            other.lower.x(),
            other.upper.x(),
        )?;
        let y = Block::intersection1(
            self.lower.y(),
            self.upper.y(),
            other.lower.y(),
            other.upper.y(),
        )?;
        let z = Block::intersection1(
            self.lower.z(),
            self.upper.z(),
            other.lower.z(),
            other.upper.z(),
        )?;

        Some(Block::new(
            Pos3::new(x.0, y.0, z.0),
            Pos3::new(x.1, y.1, z.1),
        ))
    }
}

impl<T> Block<T>
where
    T: Num + Signed + Copy,
{
    pub fn on_surface(&self, point: Pos3<T>) -> Option<Pos3<T>> {
        let mut found = false;
        let x = if point.x() == self.lower.x() {
            found = true;
            -T::one()
        } else if point.x() == self.upper.x() {
            found = true;
            T::one()
        } else {
            T::zero()
        };
        let y = if point.y() == self.lower.y() {
            found = true;
            -T::one()
        } else if point.y() == self.upper.y() {
            found = true;
            T::one()
        } else {
            T::zero()
        };
        let z = if point.z() == self.lower.z() {
            found = true;
            -T::one()
        } else if point.z() == self.upper.z() {
            found = true;
            T::one()
        } else {
            T::zero()
        };

        if found {
            Some(Pos3::new(x, y, z))
        } else {
            None
        }
    }
}

impl<T> Block<T>
where
    T: Num + Ord + Copy,
{
    pub fn extend(&self, pos: Pos3<T>) -> Block<T> {
        if self.contains(&pos) {
            return *self;
        }

        Block {
            lower: self.lower.min_components(pos),
            upper: self.upper.max_components(pos),
        }
    }
    pub fn lower(&self) -> Pos3<T> {
        self.lower
    }

    pub fn upper(&self) -> Pos3<T> {
        self.upper
    }

    pub fn contains(&self, pos: &Pos3<T>) -> bool {
        self.lower.x() <= pos.x()
            && pos.x() <= self.upper.x()
            && self.lower.y() <= pos.y()
            && pos.y() <= self.upper.y()
            && self.lower.z() <= pos.z()
            && pos.z() <= self.upper.z()
    }

    pub fn center(&self) -> Pos3<T> {
        let two = T::one() + T::one();
        (self.lower + self.upper) / two
    }
}

impl<T> Add<Pos3<T>> for Block<T>
where
    T: Num + Ord + Copy,
{
    type Output = Self;

    fn add(self, rhs: Pos3<T>) -> Self::Output {
        Block::new(self.lower + rhs, self.upper + rhs)
    }
}

impl<T> Sub<Pos3<T>> for Block<T>
where
    T: Num + Ord + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Pos3<T>) -> Self::Output {
        Block::new(self.lower - rhs, self.upper - rhs)
    }
}

impl<'a, T> Block<T>
where
    T: Num + Ord + 'a + Copy,
{
    pub fn from_iterator<I>(mut iter: I) -> Option<Self>
    where
        I: Iterator<Item = &'a Pos3<T>>,
    {
        let first = *iter.next()?;
        let (upper, lower) = iter.fold((first, first), |(mx, mn), p| {
            (mx.max_components(*p), mn.min_components(*p))
        });

        Some(Block::new(lower, upper))
    }
}

impl<T> Block<T>
where
    T: Num + Ord + Copy,
{
    pub fn from_components(components: &[(T, T)]) -> Option<Self> {
        if components.len() < 3 {
            return None;
        }
        let lower = Pos3::new(components[0].0, components[1].0, components[2].0);
        let upper = Pos3::new(components[0].1, components[1].1, components[2].1);
        Some(Block::new(lower, upper))
    }
}

impl<T> Block<T>
where
    T: Num + Copy,
{
    pub fn len_x(&self) -> T {
        self.upper.x() - self.lower.x() + T::one()
    }

    pub fn len_y(&self) -> T {
        self.upper.y() - self.lower.y() + T::one()
    }

    pub fn len_z(&self) -> T {
        self.upper.z() - self.lower.z() + T::one()
    }
}

impl<T> Block<T>
where
    T: Num + Copy,
{
    #[allow(dead_code)]
    pub fn volume(&self) -> T {
        self.len_x() * self.len_y() * self.len_z()
    }
}

impl<T> Display for Block<T>
where
    T: Num + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}]", self.lower, self.upper)
    }
}
