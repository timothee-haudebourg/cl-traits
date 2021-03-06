# Collection Traits (cl-traits)

[![CI](https://github.com/c410-f3r/cl-traits/workflows/CI/badge.svg)](https://github.com/c410-f3r/cl-traits/actions?query=workflow%3ACI)
[![crates.io](https://img.shields.io/crates/v/cl-traits.svg)](https://crates.io/crates/cl-traits)
[![Documentation](https://docs.rs/cl-traits/badge.svg)](https://docs.rs/cl-traits)
[![License](https://img.shields.io/badge/license-APACHE2-blue.svg)](./LICENSE)
[![Rustc](https://img.shields.io/badge/rustc-1.42-lightgray")](https://blog.rust-lang.org/2020/03/12/Rust-1.42.html)

Yet another library that generalizes collections. This is a best effort without GAT.

Many data structures have unique features that make it difficult or even impossible to create a single `trait` that fits in all scenarios. This crate tries to circumvent such behavior by providing a single method for each `trait` to achieve maximum flexibility and freedom.

## Examples

```rust
use cl_traits::*;

struct SomeCustomVector(Vec<i32>, Vec<i32>);

impl Length for SomeCustomVector {
  #[inline]
  fn length(&self) -> usize {
    self.0.length() + self.1.length()
  }
}

fn main() {
  let v = SomeCustomVector(vec![1, 2], vec![3, 4, 5, 6]);
  assert_eq!(v.length(), 6);
}
```

You can see more complete examples in the `examples` directory.
