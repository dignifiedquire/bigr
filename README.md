# bigr

> Fast, stack allocated fixed size numbers.


## Requirements

Currently requires `nightly` compiler as this makes heavy use of const generics and other improvements to const functions.

## Example

```rust
#![feature(const_loop)]
#![feature(const_fn)]
#![feature(const_if_match)]
#![feature(const_generics)]
#![feature(const_mut_refs)]
#![allow(incomplete_features)]

use bigr::Uint;

let x: Uint<4> = 123.into();
let y: Uint<4> = 456.into();
let z: Uint<4> = (123 * 456).into();

assert_eq!(x * y, z);
```
