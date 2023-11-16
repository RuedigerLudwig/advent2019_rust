#![allow(dead_code)]
use std::fmt::Display;

pub fn join<T: Display>(lst: &[T], sep: &str) -> String {
    lst.iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}

pub fn zip2<A, B>(o1: Option<A>, o2: Option<B>) -> Option<(A, B)> {
    o1.zip(o2)
}

pub fn zip3<A, B, C>(o1: Option<A>, o2: Option<B>, o3: Option<C>) -> Option<(A, B, C)> {
    match (o1, o2, o3) {
        (Some(a), Some(b), Some(c)) => Some((a, b, c)),
        _ => None,
    }
}

pub fn zip4<A, B, C, D>(
    o1: Option<A>,
    o2: Option<B>,
    o3: Option<C>,
    o4: Option<D>,
) -> Option<(A, B, C, D)> {
    match (o1, o2, o3, o4) {
        (Some(a), Some(b), Some(c), Some(d)) => Some((a, b, c, d)),
        _ => None,
    }
}

pub fn zip5<A, B, C, D, E>(
    o1: Option<A>,
    o2: Option<B>,
    o3: Option<C>,
    o4: Option<D>,
    o5: Option<E>,
) -> Option<(A, B, C, D, E)> {
    match (o1, o2, o3, o4, o5) {
        (Some(a), Some(b), Some(c), Some(d), Some(e)) => Some((a, b, c, d, e)),
        _ => None,
    }
}

pub fn zip6<A, B, C, D, E, F>(
    o1: Option<A>,
    o2: Option<B>,
    o3: Option<C>,
    o4: Option<D>,
    o5: Option<E>,
    o6: Option<F>,
) -> Option<(A, B, C, D, E, F)> {
    match (o1, o2, o3, o4, o5, o6) {
        (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f)) => Some((a, b, c, d, e, f)),
        _ => None,
    }
}

pub fn zip7<A, B, C, D, E, F, G>(
    o1: Option<A>,
    o2: Option<B>,
    o3: Option<C>,
    o4: Option<D>,
    o5: Option<E>,
    o6: Option<F>,
    o7: Option<G>,
) -> Option<(A, B, C, D, E, F, G)> {
    match (o1, o2, o3, o4, o5, o6, o7) {
        (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g)) => {
            Some((a, b, c, d, e, f, g))
        }
        _ => None,
    }
}
