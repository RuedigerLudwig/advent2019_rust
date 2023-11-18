#![allow(dead_code)]

use super::direction::Direction;
use super::{abs::Abs, math::gcd};
use num_traits::{CheckedAdd, CheckedSub, Float, Num, NumCast, Signed, Zero};
use std::fmt;
use std::ops::{Add, AddAssign, Div, Index, Mul, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos2<T> {
    x: T,
    y: T,
}

impl<T> Pos2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Pos2<T> {
        Pos2 { x, y }
    }

    #[inline]
    pub fn get_x(&self) -> &T {
        &self.x
    }

    #[inline]
    pub fn get_y(&self) -> &T {
        &self.y
    }
}

impl<T> From<[T; 2]> for Pos2<T> {
    fn from(value: [T; 2]) -> Self {
        let [x, y] = value;
        Pos2::new(x, y)
    }
}

impl<T> From<(T, T)> for Pos2<T> {
    fn from(value: (T, T)) -> Self {
        Pos2::new(value.0, value.1)
    }
}

impl<T> Pos2<T>
where
    T: Copy,
{
    #[inline]
    pub fn splat(v: T) -> Pos2<T> {
        Pos2::new(v, v)
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}

impl<T> Index<usize> for Pos2<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        assert!(idx < 2);
        match idx {
            0 => &self.x,
            1 => &self.y,
            _ => unreachable!(),
        }
    }
}

impl<T> Pos2<T> {
    #[inline]
    pub fn set_x(self, x: T) -> Self {
        Pos2::new(x, self.y)
    }

    #[inline]
    pub fn set_y(self, y: T) -> Self {
        Pos2::new(self.x, y)
    }

    pub fn set(self, idx: usize, value: T) -> Self {
        assert!(idx < 2);
        match idx {
            0 => self.set_x(value),
            1 => self.set_y(value),
            _ => unreachable!(),
        }
    }
}

impl<T> Pos2<T>
where
    T: Num + Copy,
{
    pub fn times_matrix(self, col1: Self, col2: Self) -> Self {
        Self::new(
            self.x * col1.x + self.y * col2.x,
            self.x * col1.y + self.y * col2.y,
        )
    }
}

impl<T> Pos2<T>
where
    T: Num + Ord + Copy,
{
    pub fn normalize(self) -> Result<(Pos2<T>, T), Pos2<T>> {
        if self.x.is_zero() && self.y.is_zero() {
            Err(self)
        } else {
            let x = if self.x >= T::zero() {
                self.x
            } else {
                T::zero() - self.x
            };
            let y = if self.y >= T::zero() {
                self.y
            } else {
                T::zero() - self.y
            };
            gcd(x, y)
                .map(|ggt| (Pos2::new(self.x / ggt, self.y / ggt), ggt))
                .ok_or(self)
        }
    }
}

impl<T> Pos2<T>
where
    T: Float,
{
    pub fn normal(self) -> Result<(Pos2<T>, T), Pos2<T>> {
        let length = self.length();
        if length == T::zero() {
            Err(self)
        } else {
            Ok((self / length, length))
        }
    }
}

impl<T> Pos2<T>
where
    T: Num + NumCast,
{
    pub fn angle(&self) -> Option<f64> {
        if let (Some(x), Some(y)) = (self.x.to_f64(), self.y.to_f64()) {
            Some(y.atan2(x))
        } else {
            None
        }
    }

    pub fn angle2(&self) -> Option<f64> {
        if let (Some(x), Some(y)) = (self.x.to_f64(), self.y.to_f64()) {
            Some((-x.atan2(-y) + std::f64::consts::PI).rem_euclid(2.0 * std::f64::consts::PI))
        } else {
            None
        }
    }
}

impl<T> Pos2<T>
where
    T: Ord + Copy,
{
    pub fn max_components(self, other: Pos2<T>) -> Self {
        Pos2::new(self.x.max(other.x), self.y.max(other.y))
    }

    pub fn min_components(self, other: Pos2<T>) -> Self {
        Pos2::new(self.x.min(other.x), self.y.min(other.y))
    }
}

impl<T> Pos2<T>
where
    T: Num + Abs,
{
    pub fn abs(self) -> T {
        self.x.abs() + self.y.abs()
    }
}

impl<T> Pos2<T>
where
    T: Float,
{
    pub fn length(self) -> T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T> fmt::Display for Pos2<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T> Zero for Pos2<T>
where
    T: Num + Zero + Copy,
{
    fn zero() -> Self {
        Pos2::splat(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T> Add for Pos2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Add<(T, T)> for Pos2<T>
where
    T: Num + Copy,
{
    type Output = Self;
    fn add(self, rhs: (T, T)) -> Self::Output {
        Pos2::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl<T, P: Into<Pos2<T>>> AddAssign<P> for Pos2<T>
where
    T: AddAssign<T> + Copy,
{
    fn add_assign(&mut self, rhs: P) {
        let rhs = rhs.into();
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T, P: Into<Pos2<T>>> Sub<P> for Pos2<T>
where
    T: Num + Copy,
{
    type Output = Pos2<T>;
    fn sub(self, rhs: P) -> Self::Output {
        let rhs = rhs.into();
        Pos2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T, P: Into<Pos2<T>>> SubAssign<P> for Pos2<T>
where
    T: SubAssign<T> + Copy,
{
    fn sub_assign(&mut self, rhs: P) {
        let rhs = rhs.into();
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> Mul<T> for Pos2<T>
where
    T: Num + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Pos2::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> Div<T> for Pos2<T>
where
    T: Num + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Pos2::new(self.x / rhs, self.y / rhs)
    }
}

impl<T> Neg for Pos2<T>
where
    T: Signed + Copy,
{
    type Output = Pos2<T>;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl<T> Pos2<T>
where
    T: Num + Abs + Copy,
{
    pub fn taxicab_between(self, other: Pos2<T>) -> T {
        self.x.abs_beween(&other.x) + self.y.abs_beween(&other.y)
    }
}

impl<T> Pos2<T>
where
    T: Num + Copy + CheckedAdd + CheckedSub,
{
    pub fn check_add(self, direction: Direction) -> Option<Self> {
        match direction {
            Direction::East => self.x.checked_add(&T::one()).map(|x| Pos2::new(x, self.y)),
            Direction::North => self.y.checked_sub(&T::one()).map(|y| Pos2::new(self.x, y)),
            Direction::West => self.x.checked_sub(&T::one()).map(|x| Pos2::new(x, self.y)),
            Direction::South => self.y.checked_add(&T::one()).map(|y| Pos2::new(self.x, y)),
        }
    }
}
