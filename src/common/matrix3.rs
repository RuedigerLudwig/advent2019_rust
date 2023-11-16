#![allow(dead_code)]
use num_traits::{Num, One, Zero};

use super::pos3::Pos3;
use std::{
    fmt::Display,
    ops::{Add, Index, Mul},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Matrix3<T>([Pos3<T>; 3]);

impl<T> Matrix3<T>
where
    T: Copy,
{
    pub fn from_col_vectors(one: Pos3<T>, two: Pos3<T>, three: Pos3<T>) -> Self {
        Self([one, two, three])
    }

    pub fn from_row_vectors(one: Pos3<T>, two: Pos3<T>, three: Pos3<T>) -> Self {
        let p1 = Pos3::new(one.x(), two.x(), three.x());
        let p2 = Pos3::new(one.y(), two.y(), three.y());
        let p3 = Pos3::new(one.z(), two.z(), three.z());
        Self([p1, p2, p3])
    }

    pub fn transpose(self) -> Self {
        Matrix3::from_row_vectors(self.0[0], self.0[1], self.0[2])
    }
}

impl<T> Mul for Matrix3<T>
where
    T: Num + Copy,
{
    type Output = Matrix3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let p0 = self * rhs[0];
        let p1 = self * rhs[1];
        let p2 = self * rhs[2];
        Self::from_col_vectors(p0, p1, p2)
    }
}

impl<T> Display for Matrix3<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for n in 0..3 {
            write!(f, "{}", self.0[n])?;
        }
        write!(f, "]")
    }
}

impl<T> Mul<Pos3<T>> for Matrix3<T>
where
    T: Num + Copy,
{
    type Output = Pos3<T>;

    fn mul(self, rhs: Pos3<T>) -> Self::Output {
        let mut result = Pos3::zero();
        for col in 0..3 {
            let mut tmp = T::zero();
            for row in 0..3 {
                tmp = tmp + self[row][col] * rhs[row];
            }
            result = result.set(col, tmp);
        }
        result
    }
}

impl<T> Index<usize> for Matrix3<T> {
    type Output = Pos3<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<T> One for Matrix3<T>
where
    T: Num + Copy,
{
    fn one() -> Self {
        Matrix3::from_col_vectors(
            Pos3::new(T::one(), T::zero(), T::zero()),
            Pos3::new(T::zero(), T::one(), T::zero()),
            Pos3::new(T::zero(), T::zero(), T::one()),
        )
    }
}

impl<T> Zero for Matrix3<T>
where
    T: Num + Copy,
{
    fn zero() -> Self {
        Matrix3::from_col_vectors(Pos3::zero(), Pos3::zero(), Pos3::zero())
    }

    fn is_zero(&self) -> bool {
        self.0[0].is_zero() && self.0[1].is_zero() && self.0[2].is_zero()
    }
}

impl<T> Add for Matrix3<T>
where
    T: Num + Copy,
{
    type Output = Matrix3<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_col_vectors(
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul() {
        let matrix =
            Matrix3::from_col_vectors(Pos3::new(1, 2, 3), Pos3::new(4, 5, 6), Pos3::new(7, 8, 9));
        assert_eq!(matrix * Matrix3::one(), matrix);
        assert_eq!(matrix[1][2], 6);

        let rot =
            Matrix3::from_col_vectors(Pos3::new(0, 1, 0), Pos3::new(1, 0, 0), Pos3::new(0, 0, 1));
        let expected =
            Matrix3::from_col_vectors(Pos3::new(4, 5, 6), Pos3::new(1, 2, 3), Pos3::new(7, 8, 9));
        assert_eq!(matrix * rot, expected);

        let m1 =
            Matrix3::from_col_vectors(Pos3::new(4, 4, 7), Pos3::new(4, 6, 5), Pos3::new(6, 1, 8));
        let m2 =
            Matrix3::from_row_vectors(Pos3::new(2, 4, 8), Pos3::new(9, 8, 7), Pos3::new(8, 0, 1));
        let expected = Matrix3::from_col_vectors(
            Pos3::new(92, 70, 123),
            Pos3::new(48, 64, 68),
            Pos3::new(66, 75, 99),
        );
        assert_eq!(m1 * m2, expected);
    }
}
