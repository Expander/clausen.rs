//! clausen
//! =======
//!
//! The clausen package provides Rust implementations of the Standard
//! Clausen functions and Glaisher-Clausen functions of integer order
//! for real or complex arguments.
//!
//! # Example:
//! ```
//! extern crate num;
//! extern crate clausen;
//! use clausen::Cl;
//!
//! fn main() {
//!     let x = 1.0;
//!     let n = 2;
//!
//!     println!("Cl_{}({}) = {}", n, x, x.cl(n)); // Cl_n(x)
//! }
//! ```


extern crate num;

mod cl;
mod range_reduction;

pub use self::cl::Cl;
