clausen
=======

[![Build Status](https://github.com/Expander/clausen.rs/workflows/test/badge.svg)](https://github.com/Expander/clausen.rs/actions)
[![Documentation](https://docs.rs/clausen/badge.svg)](https://docs.rs/clausen/)

The clausen package provides Rust implementations of the Standard
Clausen functions and Glaisher-Clausen functions of integer order for
real or complex arguments.

The clausen package depends on the `num` crate.


Example
-------

```rust
use clausen::{Cl, Sl};

fn main() {
    let x = 1.0;
    let n = 2;

    println!("Cl_{}({}) = {}", n, x, x.cl(n)); // Cl_n(x)
    println!("Sl_{}({}) = {}", n, x, x.sl(n)); // Sl_n(x)
}
```


Copying
-------
