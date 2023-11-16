use crate::range_reduction::{range_reduce_even, range_reduce_odd};


/// Standard Clausen function Cl_n(x) for a real argument
pub fn cln(n: i64, x: f64) -> f64 {
    let (r, sgn) = range_reduce(n, x);

    0.0
}


fn range_reduce(n: i64, x: f64) -> (f64, f64) {
    if n % 2 == 0 {
        range_reduce_even(x)
    } else {
        (range_reduce_odd(x), 1.0)
    }
}
