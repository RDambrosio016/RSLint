use std::collections::VecDeque;

/// An iterator which can be peeked multiple times, unlike Peekable.
pub struct MultiPeek<I: Iterator> {
    iter: I,
    buffer: VecDeque<I::Item>,
    pub idx: usize,
}

/// Creates a multi peekable iterator from an iterator
pub fn multipeek<I: Iterator>(iter: I) -> MultiPeek<I> {
    MultiPeek {
        iter,
        buffer: VecDeque::new(),
        idx: 0,
    }
}

impl<I: Iterator> MultiPeek<I> {
    /// Resets the peeking index
    pub fn reset(&mut self) {
        self.idx = 0;
    }

    /// Look ahead in the iterator without advancing it, the method can be called multiple times
    pub fn peek(&mut self) -> Option<&I::Item> {
        let ret = if self.idx < self.buffer.len() {
            Some(&self.buffer[self.idx])
        } else {
            match self.iter.next() {
                Some(i) => {
                    self.buffer.push_back(i);
                    Some(&self.buffer[self.idx])
                }

                None => return None,
            }
        };

        self.idx += 1;
        ret
    }
}

impl<I: Iterator> Iterator for MultiPeek<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.idx = 0;
        if self.buffer.is_empty() {
            self.iter.next()
        } else {
            self.buffer.pop_front()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::multipeek::*;

    #[test]
    fn multipeek_iter() {
        let mut iter = multipeek("oh hi mark".chars());
        assert_eq!(iter.peek().unwrap(), &'o');
        assert_eq!(iter.peek().unwrap(), &'h');
        iter.reset();
        assert_eq!(iter.peek().unwrap(), &'o');
    }
}