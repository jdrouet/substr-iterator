//! This library is made to iterate over a `&str` by a number of characters without allocating.
//!
//! ```rust
//! let mut iter = substr_iterator::TrigramIter::from("whatever");
//! assert_eq!(iter.next(), Some(['w', 'h', 'a']));
//! let mut iter = substr_iterator::TrigramIter::from("今天我吃饭");
//! assert_eq!(iter.next(), Some(['今', '天', '我']));
//! ```

/// A set of N characters stored as an array.
pub type Substr<const N: usize> = [char; N];
/// A set of 3 characters stored as an array.
pub type Trigram = Substr<3>;

/// An iterator for only 3 characters. This is just an alias to [SubstrIter].
pub type TrigramIter<'a> = SubstrIter<'a, 3>;

/// This is an iterator that allows to take a number of characters out of a string
/// and iterate like a window.
///
/// ```rust
/// let mut iter = substr_iterator::TrigramIter::from("whatever");
/// ```
pub struct SubstrIter<'a, const N: usize> {
    iter: std::str::Chars<'a>,
}

impl<'a, const N: usize> From<&'a str> for SubstrIter<'a, N> {
    fn from(origin: &'a str) -> Self {
        Self {
            iter: origin.chars(),
        }
    }
}

impl<const N: usize> Iterator for SubstrIter<'_, N> {
    type Item = Substr<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res: Substr<N> = [' '; N];
        res[0] = self.iter.next()?;
        let mut iter = self.iter.clone();
        for item in res.iter_mut().take(N).skip(1) {
            *item = iter.next()?;
        }
        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::SubstrIter;
    use test_case::test_case;

    fn as_array(input: &str) -> [char; 3] {
        input
            .chars()
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap()
    }

    #[test_case("whatever", ["wha", "hat", "ate", "tev", "eve", "ver"]; "with simple characters")]
    #[test_case("今天我吃饭", ["今天我", "天我吃", "我吃饭"]; "with chinese characters")]
    fn should_work(word: &str, expected: impl IntoIterator<Item = &'static str>) {
        let all = Vec::from_iter(SubstrIter::<'_, 3>::from(word));
        assert_eq!(all, expected.into_iter().map(as_array).collect::<Vec<_>>());
    }
}
