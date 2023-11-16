#![allow(dead_code)]
use num_traits::{Euclid, Num};
use rand::rngs::ThreadRng;
use rand::Rng;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MathError {
    #[error("We can not calculate so close to the ceiling")]
    TooHigh,

    #[error("Need positive modulo")]
    NeedPositiveModulo,

    #[error("Need non negative exponent")]
    NeedNonNegativeExponent,
}

fn non_zero_gcd<T>(mut a: T, mut b: T) -> T
where
    T: Num + Ord + Copy,
{
    while !b.is_zero() {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

pub fn gcd<T>(a: T, b: T) -> Option<T>
where
    T: Num + Ord + Copy,
{
    assert!(a >= T::zero());
    assert!(b >= T::zero());

    if a.is_zero() {
        if b.is_zero() {
            None
        } else {
            Some(b)
        }
    } else {
        Some(non_zero_gcd(a, b))
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Num + Ord + Copy,
{
    if a.is_zero() || b.is_zero() {
        T::zero()
    } else {
        a * b / non_zero_gcd(a, b)
    }
}

pub fn modulus_inv<T>(num: T, modulo: T) -> Option<T>
where
    T: Num + Euclid + Copy,
{
    let num = num.rem_euclid(&modulo);
    let mut s = (T::zero(), T::one());
    let mut r = (modulo, num);
    while !r.0.is_zero() {
        let q = r.1 / r.0;
        r = (r.1 - q * r.0, r.0);
        s = (s.1 - q * s.0, s.0);
    }
    if !r.1.is_one() {
        None
    } else {
        Some(s.1.rem_euclid(&modulo))
    }
}

fn quick_select<T: Ord + Copy>(lst: &mut [T], index: usize, mut rng: ThreadRng) -> T {
    match lst.len() {
        0 => unreachable!(),
        1 => {
            assert!(index == 0);
            lst[0]
        }
        2 => match index {
            0 => lst[0].min(lst[1]),
            1 => lst[0].max(lst[1]),
            _ => unreachable!(),
        },
        n => {
            let pivot = lst[rng.gen_range(0..n)];
            let lesser = lst.iter_mut().partition_in_place(|&i| i <= pivot);
            if lesser > index {
                quick_select(&mut lst[..lesser], index, rng)
            } else {
                quick_select(&mut lst[lesser..], index - lesser, rng)
            }
        }
    }
}

pub fn select<T: Ord + Copy>(lst: &mut [T], index: usize) -> T {
    assert!(index < lst.len());
    let rng = rand::thread_rng();
    quick_select(lst, index, rng)
}

#[inline]
pub fn median<T: Ord + Copy>(lst: &mut [T]) -> T {
    select(lst, lst.len() / 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn some_simple_gcd() {
        assert_eq!(Some(5), gcd(10, 15));
        assert_eq!(Some(7), gcd(21, 49));
        assert_eq!(Some(1), gcd(13, 17));
    }

    #[test]
    fn some_simple_lcm() {
        assert_eq!(18, lcm(6, 9));
        assert_eq!(20, lcm(5, 4));
    }

    #[test]
    fn test_inverse_modulo() {
        let num = 3;
        let modulo = 10;
        let inv = modulus_inv(num, modulo);

        assert_eq!(inv, Some(7));
    }

    #[test]
    fn test_median() {
        let mut input = vec![9, 1, 0, 2, 3, 4, 6, 8, 7, 10, 5];
        let expected = 5;
        assert_eq!(median(&mut input), expected);
    }
}
