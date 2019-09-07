# size_hint [![Crates.io Version](https://img.shields.io/crates/v/size_hint.svg)](https://crates.io/crates/size_hint)

A very simple library to provide a size_hint for an iterator which does not provide it's own such as std::iter::{from_fn, successors}

## Usage

```rust
use std::iter;
use size_hint::HintSize;
iter::successors(Some(inital_state), some_fn).hint_size(expected_items)
```
