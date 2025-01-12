//! This library is made to iterate over a `&str` by a number of characters without allocating.
//!
//! ```rust
//! use substr_iterator::{Trigram, TrigramIter};
//!
//! let mut iter = TrigramIter::from("whatever");
//! assert_eq!(iter.next(), Some(['w', 'h', 'a']));
//! let mut iter = TrigramIter::from("今天我吃饭");
//! assert_eq!(iter.next(), Some(['今', '天', '我']));
//! ```
//!
//! It's also possible to handle bigger windows.
//!
//! ```rust
//! use substr_iterator::{Substr, SubstrIter};
//!
//! let mut iter = SubstrIter::<2>::from("whatever");
//! assert_eq!(iter.next(), Some(['w', 'h']));
//! let mut iter = SubstrIter::<2>::from("今天我吃饭");
//! assert_eq!(iter.next(), Some(['今', '天']));
//! ```
//!
//! When the `std` feature is enabled, the [SubstrWrapper] allows to display [Substr] as a [String].
//!
//! ```rust
//! use substr_iterator::{SubstrWrapper, Trigram, TrigramIter};
//!
//! let mut iter = TrigramIter::from("whatever");
//! let item = SubstrWrapper(iter.next().unwrap());
//! assert_eq!(item.to_string(), "wha");
//! ```
//!
//! When the `serde` feature is enabled, the [SubstrWrapper] allows to serialize and deserialize.
//!
//! ```rust
//! use substr_iterator::{SubstrWrapper, Trigram, TrigramIter};
//!
//! let data: Vec<SubstrWrapper<3>> = vec![
//!     SubstrWrapper(['a', 'b', 'c']),
//!     SubstrWrapper(['今', '天', '我']),
//! ];
//! assert_eq!(
//!     serde_json::to_string(&data).unwrap(),
//!     "[\"abc\",\"今天我\"]",
//! );
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
    cache: [char; N],
    index: usize,
}

impl<'a, const N: usize> From<&'a str> for SubstrIter<'a, N> {
    fn from(origin: &'a str) -> Self {
        let mut iter = origin.chars();
        let mut cache = ['\0'; N];
        for (idx, v) in (&mut iter).take(N - 1).enumerate() {
            cache[idx] = v;
        }
        Self {
            iter,
            cache,
            index: 0,
        }
    }
}

impl<const N: usize> SubstrIter<'_, N> {
    fn get(&self, offset: usize) -> char {
        self.cache[(self.index + offset) % N]
    }

    fn push(&mut self, value: char) {
        self.cache[(self.index + N - 1) % N] = value;
    }
}

impl<const N: usize> Iterator for SubstrIter<'_, N> {
    type Item = Substr<N>;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.iter.next()?;
        self.push(value);
        let res: [char; N] = core::array::from_fn(|i| self.get(i));
        self.index += 1;
        Some(res)
    }
}

/// Wrapper around [Substr] in order to bring extra capabilities.
///
/// ```rust
/// use substr_iterator::SubstrWrapper;
/// use std::str::FromStr;
///
/// let value: [char; 3] = ['a', 'b', 'c'];
/// let wrapped = SubstrWrapper(value);
/// // implements Display
/// assert_eq!(wrapped.to_string(), "abc");
///
/// // parsing &str
/// let parsed = SubstrWrapper::from_str("abc").unwrap();
/// assert_eq!(wrapped, parsed);
/// ```
///
/// When the `serde` feature is enabled, [SubstrWrapper] provides a way to [serde::Serialize] and [serde::Deserialize].
///
/// ```rust
/// let value: [char; 3] = ['a', 'b', 'c'];
/// let wrapped = substr_iterator::SubstrWrapper(value);
/// assert_eq!(serde_json::to_string(&wrapped).unwrap(), "\"abc\"");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubstrWrapper<const N: usize>(pub Substr<N>);

impl<const N: usize> SubstrWrapper<N> {
    /// Extract the [Substr] from the wrapper
    pub fn inner(self) -> Substr<N> {
        self.0
    }
}

impl<const N: usize> From<Substr<N>> for SubstrWrapper<N> {
    fn from(value: Substr<N>) -> Self {
        Self(value)
    }
}

#[cfg(feature = "std")]
impl<const N: usize> std::fmt::Display for SubstrWrapper<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use std::fmt::Write;

        for c in self.0 {
            f.write_char(c)?;
        }
        Ok(())
    }
}

/// String parsing error for [SubstrWrapper].
///
/// ```rust
/// use substr_iterator::SubstrWrapper;
/// use std::str::FromStr;
///
/// let err = SubstrWrapper::<3>::from_str("abcd").unwrap_err();
/// assert_eq!(err.expected, 3);
/// assert_eq!(err.current, 4);
/// ```
#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug)]
pub struct SubstrParserError {
    /// The expected number of characters
    pub expected: usize,
    /// The given number of characters in the [&str]
    pub current: usize,
}

#[cfg(feature = "std")]
impl std::fmt::Display for SubstrParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "invalid length {}, expected {}",
            self.current, self.expected
        )
    }
}

#[cfg(feature = "std")]
impl<const N: usize> std::str::FromStr for SubstrWrapper<N> {
    type Err = SubstrParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut res = [' '; N];
        for (idx, item) in res.iter_mut().enumerate() {
            *item = chars.next().ok_or(SubstrParserError {
                expected: N,
                current: idx,
            })?
        }
        let rest = chars.count();
        if rest == 0 {
            Ok(SubstrWrapper(res))
        } else {
            Err(SubstrParserError {
                expected: N,
                current: res.len() + rest,
            })
        }
    }
}

#[cfg(feature = "serde")]
impl<const N: usize> serde::Serialize for SubstrWrapper<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
struct SubstrVisitor<const N: usize>;

#[cfg(feature = "serde")]
impl<const N: usize> serde::de::Visitor<'_> for SubstrVisitor<N> {
    type Value = SubstrWrapper<N>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a string of {} characters", N)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        std::str::FromStr::from_str(v).map_err(serde::de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::de::Deserialize<'de> for SubstrWrapper<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_str(SubstrVisitor)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use test_case::test_case;

    use crate::*;

    #[test_case("whatever", vec!["wha", "hat", "ate", "tev", "eve", "ver"]; "with simple characters")]
    #[test_case("今天我吃饭", vec!["今天我", "天我吃", "我吃饭"]; "with chinese characters")]
    fn should_window(word: &str, expected: Vec<&'static str>) {
        let all = Vec::from_iter(SubstrIter::<'_, 3>::from(word).map(SubstrWrapper));
        assert_eq!(
            all,
            expected
                .iter()
                .map(|v| SubstrWrapper::from_str(v).unwrap())
                .collect::<Vec<_>>()
        );
    }

    #[test_case(vec![['a', 'b', 'c']], vec!["abc"]; "with simple characters")]
    #[test_case(vec![['今','天','我'], ['天','我','吃'], ['我','吃','饭']], vec!["今天我", "天我吃", "我吃饭"]; "with chinese characters")]
    #[cfg(feature = "std")]
    fn should_display(subsets: Vec<[char; 3]>, expected: Vec<&'static str>) {
        let displayed = subsets
            .into_iter()
            .map(|v| SubstrWrapper(v).to_string())
            .collect::<Vec<_>>();
        assert_eq!(displayed, expected);
    }

    #[test]
    fn should_serialize() {
        let res: Vec<SubstrWrapper<3>> = vec![
            SubstrWrapper(['a', 'b', 'c']),
            SubstrWrapper(['今', '天', '我']),
        ];
        let json = serde_json::to_string(&res).unwrap();
        assert_eq!(json, "[\"abc\",\"今天我\"]");
        let decoded: Vec<SubstrWrapper<3>> = serde_json::from_str(&json).unwrap();
        assert_eq!(res, decoded);
    }

    #[test]
    #[should_panic(expected = "invalid length 4, expected 3")]
    fn should_not_deserialize_with_invalid_length() {
        let _: Vec<SubstrWrapper<3>> = serde_json::from_str("[\"abcd\",\"今天我\"]").unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid type: integer `42`, expected a string of 3 characters")]
    fn should_not_deserialize_with_invalid_type() {
        let _: SubstrWrapper<3> = serde_json::from_str("42").unwrap();
    }
}
