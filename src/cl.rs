mod cl1;
mod cl2;
mod cl3;
mod cl4;
mod cl5;
mod cl6;
mod cln;


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
            1 => cl1::cl1(*self),
            2 => cl2::cl2(*self),
            3 => cl3::cl3(*self),
            4 => cl4::cl4(*self),
            5 => cl5::cl5(*self),
            6 => cl6::cl6(*self),
            _ => cln::cln(n, *self)
        }
    }
}
