#![allow(dead_code)]
use super::pos2::Pos2;
use num_traits::{Num, One, Signed, Zero};
use std::{
    fmt::Display,
    ops::{Add, Index, Mul},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix2<T>([Pos2<T>; 2]);

impl<T> Matrix2<T>
where
    T: Copy,
{
    pub fn from_data(c00: T, c01: T, c10: T, c11: T) -> Self {
        Self([Pos2::new(c00, c01), Pos2::new(c10, c11)])
    }

    pub fn from_col_vectors(one: Pos2<T>, two: Pos2<T>) -> Self {
        Self([one, two])
    }

    pub fn from_row_vectors(one: Pos2<T>, two: Pos2<T>) -> Self {
        let p1 = Pos2::new(one.x(), two.x());
        let p2 = Pos2::new(one.y(), two.y());
        Self([p1, p2])
    }

    pub fn transpose(self) -> Self {
        Matrix2::from_row_vectors(self.0[0], self.0[1])
    }
}

impl<T> Matrix2<T>
where
    T: Copy + Num,
{
    pub fn det(&self) -> T {
        self.0[0].x() * self.0[1].y() - self.0[0].y() * self.0[1].x()
    }
}

impl<T> Matrix2<T>
where
    T: Copy + Num + Signed,
{
    pub fn inverse(self) -> Result<Self, Self> {
        let det = self.det();
        if det.is_zero() {
            Err(self)
        } else {
            Ok(Self::from_data(
                self.0[1].y() / det,
                -self.0[0].y() / det,
                -self.0[1].x() / det,
                self.0[0].x() / det,
            ))
        }
    }
}

impl<T> Mul for Matrix2<T>
where
    T: Num + Copy,
{
    type Output = Matrix2<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let p0 = self * rhs[0];
        let p1 = self * rhs[1];
        Self::from_col_vectors(p0, p1)
    }
}

impl<T> Display for Matrix2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for n in 0..2 {
            write!(f, "{}", self.0[n])?;
        }
        write!(f, "]")
    }
}

impl<T> Mul<Pos2<T>> for Matrix2<T>
where
    T: Num + Copy,
{
    type Output = Pos2<T>;

    fn mul(self, rhs: Pos2<T>) -> Self::Output {
        let mut result = Pos2::zero();
        for col in 0..2 {
            let mut tmp = T::zero();
            for row in 0..2 {
                tmp = tmp + self[row][col] * rhs[row];
            }
            result = result.set(col, tmp);
        }
        result
    }
}

impl<T> Index<usize> for Matrix2<T> {
    type Output = Pos2<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> One for Matrix2<T>
where
    T: Num + Copy,
{
    fn one() -> Self {
        Matrix2::from_col_vectors(
            Pos2::new(T::one(), T::zero()),
            Pos2::new(T::zero(), T::one()),
        )
    }
}

impl<T> Zero for Matrix2<T>
where
    T: Num + Copy,
{
    fn zero() -> Self {
        Matrix2::from_col_vectors(Pos2::zero(), Pos2::zero())
    }

    fn is_zero(&self) -> bool {
        self.0[0].is_zero() && self.0[1].is_zero() && self.0[2].is_zero()
    }
}

impl<T> Add for Matrix2<T>
where
    T: Num + Copy,
{
    type Output = Matrix2<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_col_vectors(self.0[0] + rhs.0[0], self.0[1] + rhs.0[1])
    }
}
