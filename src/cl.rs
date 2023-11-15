use crate::range_reduction::range_reduce_odd;


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
            _ => 0.0
        }
    }
}


fn cl1(x: f64) -> f64 {
    let y = range_reduce_odd(x);

    if y == 0.0 {
        std::f64::INFINITY
    } else {
        -(2.0*(0.5*y).sin()).ln()
    }
}
