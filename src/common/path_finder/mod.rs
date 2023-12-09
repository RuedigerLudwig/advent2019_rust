#![allow(dead_code)]

mod item_skipper;
mod path_queue;

use self::item_skipper::NoneSkipper;
pub use self::path_queue::PathQueue;
pub use item_skipper::{FingerprintItem, FingerprintSkipper, ItemSkipper};

pub trait PathFinder {
    type Item;
    type Queue: PathQueue<Self::Item>;
    type Skipper: ItemSkipper<Item = Self::Item> = NoneSkipper<Self::Item>;

    fn get_start_item(&self) -> Self::Item;

    fn is_finished(&self, item: &Self::Item) -> bool;

    fn get_next_states<'a>(&'a self, item: &'a Self::Item)
        -> impl Iterator<Item = Self::Item> + 'a;
}

pub fn find_best_path<P: PathFinder>(path_finder: P) -> Option<P::Item> {
    let mut skipper = P::Skipper::init();

    let mut queue = P::Queue::create();
    queue.push(path_finder.get_start_item());

    while let Some(item) = queue.pop() {
        if path_finder.is_finished(&item) {
            return Some(item);
        }

        if skipper.skip_item(&item) {
            continue;
        }

        for next_item in path_finder.get_next_states(&item) {
            queue.push(next_item)
        }
    }

    None
}
