use crate::cl1::cl1;
use crate::cl2::cl2;
use crate::cl3::cl3;
use crate::cl4::cl4;
use crate::cl5::cl5;
use crate::cl6::cl6;


pub trait Cl<T> {
    /// Returns the n-th order Standard Clausen function.
    ///
    /// # Example:
    /// ```
    /// use clausen::Cl;
    ///
    /// println!("Cl_{}({}) = {}", 2, 1.0, 1.0.cl(2));
    /// ```
    fn cl(&self, n: i64) -> T;
}


impl Cl<f64> for f64 {
    fn cl(&self, n: i64) -> f64 {
        match n {
            1 => cl1(*self),
            2 => cl2(*self),
            3 => cl3(*self),
            4 => cl4(*self),
            5 => cl5(*self),
            6 => cl6(*self),
            _ => 0.0
        }
    }
}
