# Substr iterator

This library is made to iterate over a `&str` by a number of characters without allocating.

## Usage

```bash
cargo add substr-iterator
```

```rust
use substr_iterator::{Trigram, TrigramIter};

let mut iter = TrigramIter::from("whatever");
assert_eq!(iter.next(), Some(['w', 'h', 'a']));
let mut iter = TrigramIter::from("今天我吃饭");
assert_eq!(iter.next(), Some(['今', '天', '我']));
```

It's also possible to handle bigger windows.

```rust
use substr_iterator::{Substr, SubstrIter};

let mut iter = SubstrIter::<2>::from("whatever");
assert_eq!(iter.next(), Some(['w', 'h']));
let mut iter = SubstrIter::<2>::from("今天我吃饭");
assert_eq!(iter.next(), Some(['今', '天']));
```

When the `std` feature is enabled, the `SubstrWrapper` allows to display `Substr` as a `String`.

```rust
use substr_iterator::{SubstrWrapper, Trigram, TrigramIter};

let mut iter = TrigramIter::from("whatever");
let item = SubstrWrapper(iter.next().unwrap());
assert_eq!(item.to_string(), "wha");
```

When the `serde` feature is enabled, the `SubstrWrapper` allows to serialize and deserialize.

```rust
use substr_iterator::{SubstrWrapper, Trigram, TrigramIter};

let data: Vec<SubstrWrapper<3>> = vec![
    SubstrWrapper(['a', 'b', 'c']),
    SubstrWrapper(['今', '天', '我']),
];
assert_eq!(
    serde_json::to_string(&data).unwrap(),
    "[\"abc\",\"今天我\"]",
);
```
