pub struct SizeHint<I> {
    iter: I,
    hint: usize,
}

/// A helper method available on all Iterators, to set size_hint
/// which is particulary useful in combination with std::iter::from_fn
/// or std::iter::successors in order to allow for efficent collection into Vec
pub trait HintSize {
    #[inline]
    fn hint_size(self, hint: usize) -> SizeHint<Self>
    where
        Self: Sized,
    {
        SizeHint { iter: self, hint }
    }
}

impl<I> HintSize for I
where
    I: Iterator + Sized,
{}

impl<I> Iterator for SizeHint<I>
where
    I: Iterator + Sized,
{
    type Item = <I as Iterator>::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.hint > 0 {
            self.hint -= 1;
        }
        self.iter.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (_approx, exact) = self.iter.size_hint();
        (self.hint, exact)
    }
}

#[cfg(test)]
mod tests {

    use crate::HintSize;
    use std::iter::successors;

    #[test]
    fn simple() {
        let max: usize = 10;
        let numbers = (0..max).collect::<Vec<_>>();
        let i = numbers.into_iter();
        assert_eq!(i.size_hint(), (max, Some(max)));
        let i = i.hint_size(max * 2);
        assert_eq!(i.size_hint(), (max * 2, Some(max)));
        let i = i.map(|a| a + 1).hint_size(max * 3);
        assert_eq!(i.size_hint(), (max * 3, Some(max)));
        let numbers: Vec<_> = i.collect();
        assert_eq!(numbers.capacity(), max * 3);
    }

    #[test]
    fn underflow() {
        let i = successors(Some(0), |prev| match prev {
            counter if *counter < 10 => Some(counter + 1),
            _ => None,
        })
        .hint_size(5);
        let numbers: Vec<_> = i.collect();
        assert_eq!(numbers.len(), 11);
    }
}
