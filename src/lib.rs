//! clausen
//! =======
//!
//! The clausen package provides Rust implementations of the Standard
//! Clausen functions and Glaisher-Clausen functions of integer order
//! for real or complex arguments.
//!
//! # Example:
//! ```
//! use clausen::Cl;
//!
//! fn main() {
//!     let x = 1.0;
//!     let n = 2;
//!
//!     println!("Cl_{}({}) = {}", n, x, x.cl(n)); // Cl_n(x)
//! }
//! ```


mod cl;
mod cl1;
mod cl2;
mod cl3;
mod cl4;
mod cl5;
mod cl6;
mod cln;
mod range_reduction;
mod sl;

pub use self::cl::Cl;
pub use self::sl::Sl;
