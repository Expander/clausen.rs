use crate::general::{co, si};


/// returns Sl_n(x) using the expansion of Sl_n(x) in terms of sin(x) and cos(x) for n > 1
pub fn sln(n: i32, x: f64) -> f64 {
    if is_even(n) {
        co(n, x)
    } else {
        si(n, x)
    }
}


fn is_even(n: i32) -> bool {
    n % 2 == 0
}
