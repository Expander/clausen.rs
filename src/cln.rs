use crate::range_reduction::{range_reduce_even, range_reduce_odd};


/// Standard Clausen function Cl_n(x) for a real argument
pub fn cln(n: i64, x: f64) -> f64 {
    let (r, sgn) = range_reduce(n, x);

    if is_even(n) && (r == 0.0 || r == core::f64::consts::PI) {
        0.0
    } else if n < 10 {
        sgn*0.0 // @todo(alex): implement advanced algorithm
    } else {
        sgn*0.0 // @todo(alex): implement series expansion
    }
}


fn range_reduce(n: i64, x: f64) -> (f64, f64) {
    if is_even(n) {
        range_reduce_even(x)
    } else {
        (range_reduce_odd(x), 1.0)
    }
}


fn is_even(n: i64) -> bool {
    n % 2 == 0
}
