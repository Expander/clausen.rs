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
//!
//! let x = 1.0;
//! let n = 2;
//!
//! println!("Cl_{}({}) = {}", n, x, x.cl(n)); // Cl_n(x)
//! println!("Sl_{}({}) = {}", n, x, x.sl(n)); // Sl_n(x)
//! ```


mod cl;
mod range_reduction;
mod sl;

pub use self::cl::Cl;
pub use self::sl::Sl;
