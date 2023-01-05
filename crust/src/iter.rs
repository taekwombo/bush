// https://www.youtube.com/watch?v=yozQ9C69pNs

pub fn flatten<O>(iter: O) -> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    Flatten::new(iter)
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outter: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(outter: O) -> Self {
        Self {
            outter,
            inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.inner.is_none() {
                self.inner = Some(self.outter.next()?.into_iter());
            }

            let inner_iter = self.inner.as_mut()?;

            // If inner iterator has some more elements return them.
            if let Some(next_inner_item) = inner_iter.next() {
                return Some(next_inner_item);
            }

            // Notify next call that the inner iterator does not have any more items.
            self.inner = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<&[()]>()).count(), 0);
        assert_eq!(flatten(vec![Vec::<&[u8]>::new()].into_iter()).count(), 0);
    }

    #[test]
    fn not_empty() {
        let mut flat = flatten(vec![vec![1, 2, 3], vec![1]].into_iter());

        assert_eq!(flat.next(), Some(1));
        assert_eq!(flat.next(), Some(2));
        assert_eq!(flat.next(), Some(3));
        assert_eq!(flat.next(), Some(1));
    }
}
