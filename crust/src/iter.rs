// https://www.youtube.com/watch?v=yozQ9C69pNs

pub fn flatten<O>(iter: O) -> Flatten<O::IntoIter>
where
    O: IntoIterator,
    O::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outter: O,
    front: Option<<O::Item as IntoIterator>::IntoIter>,
    back: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(outter: O) -> Self {
        Self {
            outter,
            front: None,
            back: None,
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
            if self.front.is_none() {
                self.front = self.outter.next().map(|v| v.into_iter());
            }

            let inner_iter = self.front.as_mut().or(self.back.as_mut())?;

            // If inner iterator has some more elements return them.
            let next_inner_item = inner_iter.next();
            if next_inner_item.is_some() {
                return next_inner_item;
            }

            // Notify next loop that the inner iterator does not have any more items.
            self.front = None;
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if self.back.is_none() {
                self.back = self.outter.next_back().map(|v| v.into_iter());
            }

            let inner_iter = self.back.as_mut().or(self.front.as_mut())?;

            // If inner iterator has some more elements return them.
            let next_inner_item = inner_iter.next_back();
            if next_inner_item.is_some() {
                return next_inner_item;
            }

            // Notify next loop that the inner iterator does not have any more items.
            self.back = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<&[()]>()).count(), 0);
        assert_eq!(flatten(vec![Vec::<&[u8]>::new()]).count(), 0);
    }

    #[test]
    fn not_empty() {
        let mut flat = flatten(vec![vec![1, 2, 3], vec![1]]);

        assert_eq!(flat.next(), Some(1));
        assert_eq!(flat.next(), Some(2));
        assert_eq!(flat.next(), Some(3));
        assert_eq!(flat.next(), Some(1));
    }

    #[test]
    fn rev() {
        assert_eq!(
            flatten(vec![vec![1, 2, 3]]).rev().collect::<Vec<_>>(),
            vec![3, 2, 1],
        );
    }

    #[test]
    fn two_ends() {
        let mut flat = flatten(vec![vec![1, 2], vec![3, 4]]);

        assert_eq!(flat.next(), Some(1));
        assert_eq!(flat.next_back(), Some(4));
        assert_eq!(flat.next_back(), Some(3));
        assert_eq!(flat.next_back(), Some(2));
    }
}
