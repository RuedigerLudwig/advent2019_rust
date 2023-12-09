use std::{collections::HashSet, marker::PhantomData};

pub trait ItemSkipper {
    type Item;

    fn init() -> Self;
    fn skip_item(&mut self, item: &Self::Item) -> bool;
}

pub trait FingerprintItem {
    type Fingerprint: std::hash::Hash + Eq;
    fn get_fingerprint(&self) -> Self::Fingerprint;
}

pub struct FingerprintSkipper<F>
where
    F: FingerprintItem,
{
    fingerprints: HashSet<F::Fingerprint>,
}

impl<F: FingerprintItem> ItemSkipper for FingerprintSkipper<F> {
    type Item = F;

    fn init() -> Self {
        Self {
            fingerprints: HashSet::new(),
        }
    }

    fn skip_item(&mut self, item: &Self::Item) -> bool {
        !self.fingerprints.insert(item.get_fingerprint())
    }
}

pub struct NoneSkipper<F> {
    _pd: PhantomData<F>,
}

impl<F> ItemSkipper for NoneSkipper<F> {
    type Item = F;

    #[inline]
    fn init() -> Self {
        Self { _pd: PhantomData }
    }

    #[inline]
    fn skip_item(&mut self, _item: &Self::Item) -> bool {
        false
    }
}
