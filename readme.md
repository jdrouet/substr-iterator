# Substr iterator

This library is made to iterate over a `&str` by a number of characters without allocating.

## Usage

```bash
cargo add substr-iterator
```

```rust
let mut iter = substr_iterator::TrigramIter::from("whatever");
assert_eq!(iter.next(), Some(['w', 'h', 'a']));
let mut iter = substr_iterator::TrigramIter::from("今天我吃饭");
assert_eq!(iter.next(), Some(['今', '天', '我']));
```
