use std::ops::Range;

#[derive(Clone, Copy, PartialEq)]
pub struct Dimension {
    pub rows: usize,
    pub columns: usize,
}
impl Dimension {
    pub fn new(rows: usize, columns: usize) -> Dimension {
        Dimension { rows, columns }
    }
    pub fn iter(self) -> Range<usize> {
        0..(self.rows * self.columns)
    }

    #[inline(always)]
    pub fn to_xy(self, index: usize) -> (usize, usize) {
        let row = index / self.columns;
        let column = index % self.columns;
        (row, column)
    }

    #[inline(always)]
    pub fn to_index(self, x: usize, y: usize) -> usize {
        x * self.columns + y
    }
}
