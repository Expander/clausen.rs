pub trait Sl<T> {
    /// Returns the n-th order Glaisher-Clausen function.
    ///
    /// # Example:
    /// ```
    /// use clausen::Sl;
    ///
    /// println!("Sl_{}({}) = {}", 2, 1.0, 1.0.sl(2));
    /// ```
    fn sl(&self, n: i64) -> T;
}


impl Sl<f64> for f64 {
    fn sl(&self, n: i64) -> f64 {
        match n {
            k if k < 1 => panic!("sl(n) not implemented for n < 1 (you've called {}.sl({}))", self, n),
            _ => 0.0
        }
    }
}
