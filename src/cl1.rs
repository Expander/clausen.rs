use crate::range_reduction::range_reduce_odd;


pub fn cl1(x: f64) -> f64 {
    let r = range_reduce_odd(x);

    if r == 0.0 {
        std::f64::INFINITY
    } else {
        -(2.0*(0.5*r).sin()).ln()
    }
}
