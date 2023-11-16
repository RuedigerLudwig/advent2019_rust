#![allow(dead_code)]
use super::pos2::Pos2;
use num_traits::{Num, NumAssignOps};
use std::fmt::Display;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Area<T>
where
    T: Num,
{
    lower_left: Pos2<T>,
    upper_right: Pos2<T>,
}

impl<T> Area<T>
where
    T: Num + Ord + Copy,
{
    pub fn new(p1: Pos2<T>, p2: Pos2<T>) -> Area<T> {
        Area {
            lower_left: p1.min_components(p2),
            upper_right: p1.max_components(p2),
        }
    }

    pub fn from_points(x1: T, y1: T, x2: T, y2: T) -> Area<T> {
        Area {
            lower_left: Pos2::new(x1.min(x2), y1.min(y2)),
            upper_right: Pos2::new(x1.max(x2), y1.max(y2)),
        }
    }
}

impl<T> Area<T>
where
    T: Num + Ord + Copy,
{
    pub fn extend(&self, pos: Pos2<T>) -> Area<T> {
        if self.contains(pos) {
            return *self;
        }

        Area {
            lower_left: self.lower_left.min_components(pos),
            upper_right: self.upper_right.max_components(pos),
        }
    }
    pub fn lower_left(&self) -> Pos2<T> {
        self.lower_left
    }

    pub fn upper_right(&self) -> Pos2<T> {
        self.upper_right
    }

    pub fn top(&self) -> T {
        self.upper_right.y()
    }

    pub fn right(&self) -> T {
        self.upper_right.x()
    }

    pub fn bottom(&self) -> T {
        self.lower_left.y()
    }

    pub fn left(&self) -> T {
        self.lower_left.x()
    }

    pub fn contains(&self, pos: Pos2<T>) -> bool {
        self.lower_left.x() <= pos.x()
            && pos.x() <= self.upper_right.x()
            && self.lower_left.y() <= pos.y()
            && pos.y() <= self.upper_right.y()
    }

    pub fn widen(self, inc: T) -> Self {
        Self::new(
            self.lower_left - Pos2::splat(inc),
            self.upper_right + Pos2::splat(inc),
        )
    }
}

impl<'a, T> Area<T>
where
    T: Num + Ord + 'a + Copy,
{
    pub fn from_iterator<I>(mut iter: I) -> Option<Self>
    where
        I: Iterator<Item = &'a Pos2<T>>,
    {
        let first = *iter.next()?;
        let (upper, lower) = iter.fold((first, first), |(mx, mn), p| {
            (mx.max_components(*p), mn.min_components(*p))
        });

        Some(Area::new(lower, upper))
    }
}

impl<T> Area<T>
where
    T: Num + Copy,
{
    pub fn width(&self) -> T {
        self.upper_right.x() - self.lower_left.x() + T::one()
    }

    #[allow(dead_code)]
    pub fn height(&self) -> T {
        self.upper_right.y() - self.lower_left.y() + T::one()
    }
}

impl<T> Area<T>
where
    T: Num + Copy,
{
    #[allow(dead_code)]
    pub fn area(&self) -> T {
        self.width() * self.height()
    }
}

impl<T> Display for Area<T>
where
    T: Num + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}-{}]", self.lower_left, self.upper_right)
    }
}

impl<T> Area<T>
where
    T: Num + Copy,
{
    pub fn cells(&self, ascending: bool) -> CellIterator<'_, T> {
        CellIterator::new(self, ascending)
    }

    pub fn rows(&self, ascending: bool) -> RowIterator<'_, T> {
        RowIterator::new(self, ascending)
    }
}

#[derive(Debug)]
pub struct RowIterator<'a, T>
where
    T: Num + Copy,
{
    area: &'a Area<T>,
    row: T,
    ascending: bool,
}

impl<'a, T> RowIterator<'a, T>
where
    T: Num + Copy,
{
    fn new(area: &'a Area<T>, ascending: bool) -> RowIterator<'a, T> {
        RowIterator {
            area,
            row: if ascending {
                area.lower_left.y()
            } else {
                area.upper_right.y()
            },
            ascending,
        }
    }
}

impl<'a, T> Iterator for RowIterator<'a, T>
where
    T: Num + Ord + NumAssignOps + Copy,
{
    type Item = Row<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.ascending && self.row <= self.area.upper_right.y())
            || (!self.ascending && self.row >= self.area.lower_left.y())
        {
            let row = Row {
                area: self.area,
                row: self.row,
            };
            if self.ascending {
                self.row += T::one();
            } else {
                self.row -= T::one();
            }
            Some(row)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Row<'a, T>
where
    T: Num,
{
    area: &'a Area<T>,
    row: T,
}

impl<'a, T> Row<'a, T>
where
    T: Num + Copy,
{
    pub fn cols(&self, ascending: bool) -> ColIterator<'_, T> {
        ColIterator {
            area: self.area,
            row: self.row,
            col: if ascending {
                self.area.lower_left.x()
            } else {
                self.area.upper_right.x()
            },
            ascending,
        }
    }
}

#[derive(Debug)]
pub struct ColIterator<'a, T>
where
    T: Num,
{
    area: &'a Area<T>,
    row: T,
    col: T,
    ascending: bool,
}

impl<'a, T> Iterator for ColIterator<'a, T>
where
    T: Num + Ord + NumAssignOps + Copy,
{
    type Item = Pos2<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if (self.ascending && self.col <= self.area.upper_right.x())
            || (!self.ascending && self.col >= self.area.lower_left.x())
        {
            let pos = Pos2::new(self.col, self.row);
            if self.ascending {
                self.col += T::one();
            } else {
                self.col -= T::one();
            }
            Some(pos)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct CellIterator<'a, T>
where
    T: Num,
{
    area: &'a Area<T>,
    row: T,
    col: T,
    ascending: bool,
}

impl<'a, T> CellIterator<'a, T>
where
    T: Num + Copy,
{
    pub fn new(area: &'a Area<T>, ascending: bool) -> CellIterator<'a, T> {
        let (col, row) = if ascending {
            (area.lower_left.x(), area.lower_left.y())
        } else {
            (area.upper_right.x(), area.upper_right.y())
        };
        CellIterator {
            area,
            row,
            col,
            ascending,
        }
    }
}

impl<'a, T> Iterator for CellIterator<'a, T>
where
    T: Num + Ord + NumAssignOps + Copy,
{
    type Item = Pos2<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.ascending && self.row <= self.area.upper_right.y())
            || (!self.ascending && self.row >= self.area.lower_left.y())
        {
            let pos = Pos2::new(self.col, self.row);
            if self.ascending {
                self.col += T::one();
                if self.col > self.area.upper_right.x() {
                    self.row += T::one();
                    self.col = self.area.lower_left.x();
                }
            } else {
                self.col -= T::one();
                if self.col < self.area.lower_left.x() {
                    self.row -= T::one();
                    self.col = self.area.upper_right.x();
                }
            }

            Some(pos)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cell_iterator() {
        let area = Area::new(Pos2::new(-1, -1), Pos2::new(1, 1));
        let result = area.cells(true).collect::<Vec<_>>();
        let expected = vec![
            Pos2::new(-1, -1),
            Pos2::new(0, -1),
            Pos2::new(1, -1),
            Pos2::new(-1, 0),
            Pos2::new(0, 0),
            Pos2::new(1, 0),
            Pos2::new(-1, 1),
            Pos2::new(0, 1),
            Pos2::new(1, 1),
        ];
        assert_eq!(result, expected);
    }
}
