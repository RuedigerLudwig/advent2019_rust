#![allow(dead_code)]
use num_traits::{Num, PrimInt, Signed, Zero};
use std::fmt;
use std::ops::{Add, Div, Index, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pos3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Pos3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Pos3<T> {
        Pos3 { x, y, z }
    }

    #[inline]
    pub fn get_x(&self) -> &T {
        &self.x
    }

    #[inline]
    pub fn get_y(&self) -> &T {
        &self.y
    }

    #[inline]
    pub fn get_z(&self) -> &T {
        &self.z
    }
}

impl<T: Signed + PrimInt> Pos3<T> {
    pub fn is_unit(&self) -> bool {
        self.abs() == T::one()
    }
}

impl<T: Signed> Pos3<T> {
    pub fn signum(&self) -> Pos3<T> {
        Pos3::new(self.x.signum(), self.y.signum(), self.z.signum())
    }
}

impl<T: Copy + Default> From<&[T]> for Pos3<T> {
    fn from(value: &[T]) -> Self {
        match value.len() {
            0 => Pos3::new(T::default(), T::default(), T::default()),
            1 => Pos3::new(value[0], T::default(), T::default()),
            2 => Pos3::new(value[0], value[1], T::default()),
            _ => Pos3::new(value[0], value[1], value[2]),
        }
    }
}

impl<T: Copy> From<[T; 3]> for Pos3<T> {
    fn from(value: [T; 3]) -> Self {
        Pos3::new(value[0], value[1], value[2])
    }
}

impl<T> From<(T, T, T)> for Pos3<T> {
    fn from(value: (T, T, T)) -> Self {
        Pos3::new(value.0, value.1, value.2)
    }
}

impl<T> Pos3<T>
where
    T: Copy,
{
    #[inline]
    pub fn splat(v: T) -> Pos3<T> {
        Pos3::new(v, v, v)
    }

    #[inline]
    pub fn x(&self) -> T {
        self.x
    }

    #[inline]
    pub fn y(&self) -> T {
        self.y
    }

    #[inline]
    pub fn z(&self) -> T {
        self.z
    }
}

impl<T> Pos3<T>
where
    T: Ord + Copy,
{
    pub fn max_components(self, other: Pos3<T>) -> Self {
        Pos3::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    pub fn min_components(self, other: Pos3<T>) -> Self {
        Pos3::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }
}

impl<T> Zero for Pos3<T>
where
    T: Num + Zero + Copy,
{
    fn zero() -> Self {
        Pos3::splat(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}

impl<T> Pos3<T>
where
    T: Signed,
{
    pub fn abs(self) -> T {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl<T> fmt::Display for Pos3<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T, P: Into<Pos3<T>>> Add<P> for Pos3<T>
where
    T: Num + Copy,
{
    type Output = Self;
    fn add(self, rhs: P) -> Self::Output {
        let rhs = rhs.into();
        Pos3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T> std::iter::Sum for Pos3<T>
where
    T: Num + Copy,
{
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(Add::add).unwrap_or(Pos3::zero())
    }
}

impl<T, P: Into<Pos3<T>>> Sub<P> for Pos3<T>
where
    T: Num + Copy,
{
    type Output = Pos3<T>;
    fn sub(self, rhs: P) -> Self::Output {
        let rhs = rhs.into();
        Pos3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T> Pos3<T>
where
    T: Num + Copy,
{
    pub fn component_mul(self, rhs: Self) -> Self {
        Pos3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl<T> Mul<T> for Pos3<T>
where
    T: Num + Copy,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Pos3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T> Div<T> for Pos3<T>
where
    T: Num + Copy,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        Pos3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T> Neg for Pos3<T>
where
    T: Signed + Copy,
{
    type Output = Pos3<T>;

    fn neg(self) -> Self::Output {
        Pos3::new(-self.x(), -self.y(), -self.z())
    }
}

impl<T> Pos3<T>
where
    T: Num + Copy,
{
    pub fn cross(self, rhs: Pos3<T>) -> Self {
        Pos3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl<T> Pos3<T> {
    #[inline]
    pub fn set_x(self, x: T) -> Self {
        Pos3::new(x, self.y, self.z)
    }
    #[inline]
    pub fn set_y(self, y: T) -> Self {
        Pos3::new(self.x, y, self.z)
    }
    #[inline]
    pub fn set_z(self, z: T) -> Self {
        Pos3::new(self.x, self.y, z)
    }
    pub fn set(self, idx: usize, value: T) -> Self {
        assert!(idx < 3);
        match idx {
            0 => self.set_x(value),
            1 => self.set_y(value),
            2 => self.set_z(value),
            _ => unreachable!(),
        }
    }
}

impl<T> Index<usize> for Pos3<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        assert!(idx < 3);
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}

impl<T> Pos3<T>
where
    T: Copy,
{
    pub fn iter(self) -> PosIterator<T> {
        PosIterator::new(self)
    }
}

pub struct PosIterator<T> {
    pos: Pos3<T>,
    idx: Option<usize>,
}

impl<T> PosIterator<T> {
    pub fn new(pos: Pos3<T>) -> Self {
        Self { pos, idx: Some(0) }
    }
}

impl<T: Copy> Iterator for PosIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(idx) = self.idx else {
            return None;
        };
        self.idx = (idx < 2).then_some(idx + 1);
        Some(self.pos[idx])
    }
}
