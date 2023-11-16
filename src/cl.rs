use crate::cl1::cl1;
use crate::cl2::cl2;
use crate::cl3::cl3;
use crate::cl4::cl4;


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
            _ => 0.0
        }
    }
}
