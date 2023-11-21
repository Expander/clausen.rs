//! clausen
//! =======
//!
//! The clausen package provides Rust implementations of the Standard
//! Clausen functions and Glaisher-Clausen functions of integer order
//! for real or complex arguments.
//!
//! # Example:
//! ```
//! use clausen::{Cl, Sl};
//! use num::complex::Complex;
//!
//! let x = 1.0;
//! let z = Complex::new(1.0, 1.0);
//! let n = 2;
//!
//! println!("Cl_{}({}) = {}", n, x, x.cl(n)); // Cl_n(x)
//! println!("Cl_{}({}) = {}", n, z, z.cl(n)); // Cl_n(z)
//! println!("Sl_{}({}) = {}", n, x, x.sl(n)); // Sl_n(x)
//! println!("Sl_{}({}) = {}", n, z, z.sl(n)); // Sl_n(z)
//! ```


mod cl;
mod general;
mod range_reduction;
mod sl;

pub use self::cl::Cl;
pub use self::sl::Sl;
