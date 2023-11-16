#![allow(dead_code)]
use super::pos3::Pos3;
use std::{
    fmt::Display,
    ops::{Mul, Neg},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnitVector(Pos3<i8>);

pub const X: UnitVector = UnitVector(Pos3::new(1, 0, 0));
pub const NEG_X: UnitVector = UnitVector(Pos3::new(-1, 0, 0));
pub const Y: UnitVector = UnitVector(Pos3::new(0, 1, 0));
pub const NEG_Y: UnitVector = UnitVector(Pos3::new(0, -1, 0));
pub const Z: UnitVector = UnitVector(Pos3::new(0, 0, 1));
pub const NEG_Z: UnitVector = UnitVector(Pos3::new(0, 0, -1));

impl UnitVector {
    pub fn try_new(vector: Pos3<i8>) -> Option<Self> {
        vector.is_unit().then_some(UnitVector(vector))
    }

    pub fn x(self) -> i8 {
        self.0.x()
    }

    pub fn y(self) -> i8 {
        self.0.y()
    }

    pub fn z(self) -> i8 {
        self.0.z()
    }
}

impl Display for UnitVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<UnitVector> for Pos3<T>
where
    T: From<i8>,
{
    fn from(value: UnitVector) -> Self {
        Pos3::new(value.x().into(), value.y().into(), value.z().into())
    }
}

impl Mul for UnitVector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        UnitVector(self.0.cross(rhs.0))
    }
}

impl Mul<bool> for UnitVector {
    type Output = Self;

    fn mul(self, rhs: bool) -> Self::Output {
        if rhs {
            self
        } else {
            Self(self.0 * -1)
        }
    }
}

impl Neg for UnitVector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        UnitVector(-self.0)
    }
}
