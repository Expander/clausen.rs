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


fn range_reduce_odd(mut x: f64) -> f64 {
    if x < 0.0 {
        x = -x;
    }

    if x >= 2.0*std::f64::consts::PI {
        x = x.rem_euclid(2.0*std::f64::consts::PI);
    }

    if x > std::f64::consts::PI {
        let p0 = 6.28125;
        let p1 = 0.0019353071795864769253;
        x = (p0 - x) + p1;
    }

    x
}
