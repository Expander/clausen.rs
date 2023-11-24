clausen
=======

[![Build Status](https://github.com/Expander/clausen.rs/workflows/test/badge.svg)](https://github.com/Expander/clausen.rs/actions)
[![Documentation](https://docs.rs/clausen/badge.svg)](https://docs.rs/clausen/)

The clausen package provides Rust implementations of the Standard
Clausen functions and Glaisher-Clausen functions of integer order for
real or complex arguments.

The clausen package depends on the `num` and the `polylog` crates.


Example
-------

```rust
use clausen::{Cl, Sl};
use num::complex::Complex;

let x = 1.0;
let z = Complex::new(1.0, 1.0);
let n = 2;

println!("Cl_{}({}) = {}", n, x, x.cl(n)); // Cl_n(x)
println!("Cl_{}({}) = {}", n, z, z.cl(n)); // Cl_n(z)
println!("Sl_{}({}) = {}", n, x, x.sl(n)); // Sl_n(x)
println!("Sl_{}({}) = {}", n, z, z.sl(n)); // Sl_n(z)
```


Notes
-----

The implementation of the Standard Clausen function `x.cl(n)` for real
`x` and positive `n` follows the approach presented in [Jiming Wu,
Xiaoping Zhang, Dongjie Liu, "An efficient calculation of the Clausen
functions Cl_n(0)(n >= 2)", Bit Numer Math 50, 193-206 (2010)
[https://doi.org/10.1007/s10543-009-0246-8](https://doi.org/10.1007/s10543-009-0246-8)].


Copying
-------

clausen is licenced under the GNU Lesser General Public License (GNU
LGPL) version 3.
