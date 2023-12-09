use std::collections::BinaryHeap;

pub trait PathQueue<I> {
    fn create() -> Self;
    fn push(&mut self, item: I);
    fn pop(&mut self) -> Option<I>;
}

impl<I> PathQueue<I> for BinaryHeap<I>
where
    I: Ord + Eq,
{
    fn push(&mut self, item: I) {
        self.push(item)
    }

    fn pop(&mut self) -> Option<I> {
        self.pop()
    }

    fn create() -> Self {
        BinaryHeap::new()
    }
}

impl<I> PathQueue<I> for Vec<I> {
    fn push(&mut self, item: I) {
        self.push(item)
    }

    fn pop(&mut self) -> Option<I> {
        self.pop()
    }

    fn create() -> Self {
        Vec::new()
    }
}
