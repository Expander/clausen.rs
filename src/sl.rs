mod sl1;
mod sl2;
mod sl3;
mod sl4;
mod sl5;
mod sl6;
mod sl7;
mod sl8;
mod sln;

use crate::range_reduction::{range_reduce_even, range_reduce_odd};
use crate::sl::sl1::sl1;
use crate::sl::sl2::sl2;
use crate::sl::sl3::sl3;
use crate::sl::sl4::sl4;
use crate::sl::sl5::sl5;
use crate::sl::sl6::sl6;
use crate::sl::sl7::sl7;
use crate::sl::sl8::sl8;
use crate::sl::sln::sln;


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
            4 => sgn*sl4(r),
            5 => sgn*sl5(r),
            6 => sgn*sl6(r),
            7 => sgn*sl7(r),
            8 => sgn*sl8(r),
            _ => sgn*sln(n, r)
        }
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
