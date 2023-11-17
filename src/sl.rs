use crate::range_reduction::{range_reduce_even, range_reduce_odd};


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
        let (r, sgn) = range_reduce(n, *self);

        match n {
            k if k < 1 => panic!("sl(n) not implemented for n < 1 (you've called {}.sl({}))", self, n),
            1 => sgn*sl1(r),
            2 => sgn*sl2(r),
            3 => sgn*sl3(r),
            _ => sgn*sln(n, r)
        }
    }
}


fn sl1(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else {
        0.5*(core::f64::consts::PI - x)
    }
}


fn sl2(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    1.0/6.0*pi*pi + (-0.5*pi + 0.25*x)*x
}


fn sl3(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    x*(1.0/6.0*pi*pi + (-0.25*pi + 1.0/12.0*x)*x)
}


/// returns Sl_n(x) using the expansion of Sl_n(x) in terms of sin(x) and cos(x)
fn sln(n: i64, x: f64) -> f64 {
    let kmax = (f64::EPSILON.powf(-(n as f64).recip())).ceil() as i64;

    if is_even(n) {
        let co = x.cos();
        let mut co2 = 1.0; // cos((n-2)*x)
        let mut co1 = co;  // cos((n-1)*x)
        let mut sum = co;
        for k in 2..=kmax {
            let con = 2.0*co*co1 - co2; // cos(n*x)
            co2 = co1;
            co1 = con;
            sum += con/(k as f64).powf(n as f64);
        }
        sum
    } else {
        let (mut si, co) = x.sin_cos();
        let mut si2 = 0.0; // sin((n-2)*x)
        let mut si1 = si;  // sin((n-1)*x)
        let mut sum = si;
        for k in 2..=kmax {
            si = 2.0*co*si1 - si2; // sin(n*x)
            si2 = si1;
            si1 = si;
            sum += si/(k as f64).powf(n as f64);
        }
        sum
    }
}


/// map x to [0,pi] using the symmetries of Sl_n(x)
fn range_reduce(n: i64, x: f64) -> (f64, f64) {
    if is_even(n) {
        (range_reduce_odd(x), 1.0)
    } else {
        range_reduce_even(x)
    }
}


fn is_even(n: i64) -> bool {
    n % 2 == 0
}
