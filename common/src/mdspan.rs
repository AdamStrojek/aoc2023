
pub struct MDSpan<'a, T> {
    vec: &'a Vec<T>,
    col: usize,
    row: usize,
}

impl<'a, T> MDSpan<'a, T> {
    /// Constructs a new `MDSpan` with the given vector and dimensions.
    pub fn new(vec: &'a Vec<T>, row: usize, col: usize) -> Self {
        Self{ vec, col, row }
    }

    /// Returns an iterator over the elements of the `MDSpan`.
    /// The iterator yields tuples (i, j, value), where i is the index in the row dimension,
    /// and j is the index in the column dimension.
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.row).flat_map(move |i| {
            (0..self.col).map(move |j| (i, j, &self.vec[i * self.col + j]))
        })
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        if col < self.col && row < self.row {
            Some(&self.vec[row * self.col + col])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mdspan_get() {
        let array = vec![1, 2, 3, 4, 5, 6];
        let span = MDSpan::new(&array, 2, 3);

        assert_eq!(span.get(10, 10), None);
        assert_eq!(span.get(0, 0), Some(&1));
        assert_eq!(span.get(0, 1), Some(&2));
        assert_eq!(span.get(0, 2), Some(&3));
        assert_eq!(span.get(1, 0), Some(&4));
        assert_eq!(span.get(1, 1), Some(&5));
        assert_eq!(span.get(1, 2), Some(&6));
    }

    #[test]
    fn test_mdspan_iter() {
        let array = vec![1, 2, 3, 4, 5, 6];
        let span = MDSpan::new(&array, 2, 3);

        let mut it = span.iter();

        assert_eq!(it.next().unwrap(), (0, 0, &1));
        assert_eq!(it.next().unwrap(), (0, 1, &2));
        assert_eq!(it.next().unwrap(), (0, 2, &3));
        assert_eq!(it.next().unwrap(), (1, 0, &4));
        assert_eq!(it.next().unwrap(), (1, 1, &5));
        assert_eq!(it.next().unwrap(), (1, 2, &6));
    }
}